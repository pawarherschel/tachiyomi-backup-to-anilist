#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::missing_const_for_fn)]

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::hash::BuildHasher;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};

use indicatif::ProgressIterator;
use libflate::gzip;
use prost::Message;
use ureq::serde_json::Value;
use ureq::{json, Error, Response};

use tachiyomi_backup_to_anilist::anilist::{get_code, get_token};
use tachiyomi_backup_to_anilist::responses::{Format, Medum, Root};
use tachiyomi_backup_to_anilist::tachiyomi_backup::Backup;
use tachiyomi_backup_to_anilist::{get_pb, time_it, write_items_to_file};

const SEARCH_QUERY: &str = "\
query ($search: String, $mediaType: MediaType) {
    Page(perPage: 3) {
        media(search: $search, type: $mediaType) {
            id
            title {
                romaji
                english
                native
                userPreferred
            }
            synonyms
            format
            isLicensed
            mediaListEntry {
                id
                mediaId
            }
        }
    }
}\
";

const MUTATION_QUERY: &str = "\
mutation ($mediaId: Int, $status: MediaListStatus, $progress: Int) {
    SaveMediaListEntry (mediaId: $mediaId, status: $status, progress: $progress) {
        id 
        status
        progress
    }
}\
";

// test 100%
// 29-sai Dokushin wa Isekai de Jiyuu ni Ikita……katta
// test 0%
// When you're crossdressing, what do you do about your underwear?
// exists but cant find due to characters with accent
// "Kōryakuhon" o Kushi Suru Saikyō no Mahōtsukai ~<Meirei sa Sero> to wa Iwa Senai Oreryū Maō Tōbatsu Saizen Rūto ~

pub fn remove_accents(input: String) -> (std::string::String, bool, std::vec::Vec<char>) {
    let mut flag = false;
    let mut removed = vec![];

    let s = input
        .chars()
        .filter_map(|c| {
            if c.is_ascii() {
                Some(c)
            } else {
                flag = true;
                match c {
                    'é' | 'É' => Some('e'),
                    'ö' | 'ō' => Some('o'),
                    'ū' => Some('u'),
                    c => {
                        removed.push(c);
                        None
                    }
                }
                // Some(c)
            }
        })
        .collect();

    (s, flag, removed)
}

pub fn separator() {
    println!("{}", (0..15).map(|_| '-').collect::<String>());
}

const MAX_RETRIES: u8 = 5;

pub fn rate_limited_query(
    name: String,
    cache: &mut HashMap<String, Vec<Medum>>,
    try_num: Option<u8>,
) -> Option<u64> {
    if let Some(media) = cache.get(&name) {
        return find_id(&name, media);
    }

    let resp = send_search_request(name.as_str());

    let now = Instant::now();

    let resp = match resp {
        Ok(r) => r,
        Err(e) => match e {
            Error::Status(429, r) => {
                let retry_after = r.header("retry-after").unwrap().parse().unwrap();
                eprintln!("rate limit exceeded");
                sleep(Duration::from_secs(retry_after));
                return rate_limited_query(name, cache, Some(try_num.unwrap_or_default()));
            }
            Error::Status(500, r) => {
                eprintln!("bad request");
                let slept_for = sleep_till(&now, 2);
                if try_num.is_some_and(|num| num > MAX_RETRIES) {
                    return None;
                }
                return rate_limited_query(name, cache, Some(try_num.unwrap_or_default()));
            }
            Error::Status(status_code, r) => {
                if try_num.is_some_and(|num| num > MAX_RETRIES) {
                    return None;
                }
                panic!(
                    "error code: {status_code}\nresponse:{}",
                    r.into_json::<Value>().unwrap()
                );
            }
            Error::Transport(e) => {
                if try_num.is_some_and(|num| num > MAX_RETRIES) {
                    return None;
                }
                panic!("transport error: {e}");
            }
        },
    };

    let headers = extract_headers(&resp);

    let ratelimit_remaining = resp
        .header("x-ratelimit-remaining")
        .unwrap_or_default()
        .parse()
        .unwrap_or(0_u32);

    if ratelimit_remaining < 30 {
        println!("x-ratelimit-remaining: {ratelimit_remaining}",);
    }

    let root = resp.into_json::<Root>().unwrap();
    let list = root.data.page.media;
    let found = find_id(&name, &list);

    cache.insert(name, list);
    let slept_for = sleep_till(&now, 2);

    found
}

pub fn extract_headers(resp: &Response) -> Vec<(String, String)> {
    let headers = resp.headers_names();
    headers
        .into_iter()
        .map(|it| {
            (
                it.clone(),
                resp.header(it.as_str()).unwrap_or_default().to_string(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn send_search_request(search_string: &str) -> Result<Response, ureq::Error> {
    let variables = json!({
        "search": search_string,
        "mediaType": "MANGA"
    });
    let json = json!({
        "query": SEARCH_QUERY,
        "variables": variables
    });

    ureq::post("https://graphql.anilist.co")
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .send_json(json)
}

pub fn sleep_till(now: &Instant, seconds: u64) -> Duration {
    let elapsed = now.elapsed();
    let sleep_time = Duration::from_secs(seconds).saturating_sub(elapsed);

    sleep(sleep_time);

    sleep_time
}

#[must_use]
pub fn find_id(name: &str, media: &[Medum]) -> Option<u64> {
    let list = media
        .iter()
        .filter(|it| matches!(it.format, Format::Manga))
        .collect::<Vec<_>>();

    let name = Some(name.to_owned());

    let found = list.iter().find(|it| {
        name == it.title.romaji
            || name == it.title.english
            || name == it.title.native
            || name == Some(it.title.user_preferred.clone())
    });

    found.map(|m| m.id)
}

#[allow(clippy::too_many_lines)]
fn main() {
    let tachibk = fs::read("assets/backup.tachibk").unwrap();
    let mut decoder = gzip::Decoder::new(&tachibk[..]).unwrap();
    let mut file = vec![];
    decoder.read_to_end(&mut file).unwrap();
    let bytes = prost::bytes::Bytes::from(file);
    let backup = Backup::decode(bytes).unwrap();

    debug_assert!(backup.backup_manga.len() == 1998);

    let Backup {
        backup_manga,
        backup_categories,
        backup_sources,
        backup_preferences,
        backup_source_preferences,
    } = backup;

    let not_tracked = backup_manga
        .iter()
        .filter(|it| it.tracking.is_empty())
        .collect::<Vec<_>>();

    let tracked = backup_manga
        .iter()
        .filter(|it| !it.tracking.is_empty())
        .collect::<Vec<_>>();

    dbg!(backup_manga.len());
    dbg!(not_tracked.len());
    dbg!(tracked.len());

    separator();

    let set = backup_manga
        .iter()
        .map(tachiyomi_backup_to_anilist::tachiyomi_backup::BackupManga::title)
        .map(|it| remove_accents(it.to_string()))
        .filter(|(_, it, _)| *it)
        .flat_map(|(_, _, it)| it)
        .collect::<BTreeSet<_>>();

    println!("{set:?}");

    separator();

    let mut cache = fs::read_to_string("temp/cache.ron").map_or_else(
        |_| HashMap::new(),
        |contents| ron::from_str(contents.as_str()).unwrap(),
    );

    let x_none = time_it! {at once | "x_none" =>
        backup_manga
            .iter()
            .progress_with(get_pb(backup_manga.len().try_into().unwrap(), ""))
            .map(|it| {
                (
                    it.clone(),
                    rate_limited_query(it.title().to_string(), &mut cache, None),
                )
            })
            .filter(|(_, it)| it.is_none())
            .map(|(it, _)| it)
            .collect::<Vec<_>>()
    };

    let x_some = time_it! {at once | "x_some" =>
        backup_manga
            .iter()
            .progress_with(get_pb(backup_manga.len().try_into().unwrap(), ""))
            .map(|it| {
                (
                    it.clone(),
                    rate_limited_query(it.title().to_string(), &mut cache, None),
                )
            })
            .filter(|(_, it)| it.is_some())
            .map(|(m, it)|(m, it.unwrap()))
            .collect::<Vec<_>>()
    };

    separator();

    dbg!(x_none.len());
    dbg!(x_some.len());
    dbg!(cache.len());

    separator();

    write_items_to_file!(x_none);
    write_items_to_file!(x_some);
    write_items_to_file!(cache);

    separator();

    let mut untracked_some = vec![];
    let mut untracked_none = vec![];

    not_tracked
        .iter()
        .map(|it| (it, it.title()))
        .map(|(m, it)| (m, it, rate_limited_query(it.to_string(), &mut cache, None)))
        .for_each(|(m, n, it)| {
            it.map_or_else(
                || untracked_none.push((n, m)),
                |it| untracked_some.push((n, m, it)),
            );
        });

    dbg!(untracked_some.len());
    dbg!(untracked_none.len());

    separator();

    write_items_to_file!(untracked_some);
    write_items_to_file!(untracked_none);

    separator();

    let mut tracked_some = vec![];
    let mut tracked_none = vec![];

    tracked
        .iter()
        .map(|it| (it, it.title()))
        .map(|(m, it)| (m, it, rate_limited_query(it.to_string(), &mut cache, None)))
        .for_each(|(m, n, it)| {
            it.map_or_else(
                || tracked_none.push((n, m)),
                |it| tracked_some.push((n, m, it)),
            );
        });

    dbg!(tracked_some.len());
    dbg!(tracked_none.len());

    separator();

    write_items_to_file!(tracked_some);
    write_items_to_file!(tracked_none);

    separator();

    todo!();

    let mutation_variables = json!({
        "mediaId": 173388,
        "status":"CURRENT",
        "progress": 1
    });

    let json = json!({
        "query": MUTATION_QUERY,
        "variables": mutation_variables
    });

    let code = get_code();
    let token = get_token(code);
    let token_len = token.len();

    println!(
        "\
        sending mutation: {MUTATION_QUERY}\n\
        w/ variables: {mutation_variables}\n\
        payload: {json}\n\
        token len: {token_len}
        "
    );

    let resp = ureq::post("https://graphql.anilist.co")
        .set("Authorization", &format!("Bearer {}", token))
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .send_json(json);

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => match e {
            Error::Status(s, r) => {
                panic!("{}", r.into_json::<Value>().unwrap());
            }
            Error::Transport(e) => {
                panic!("{e}")
            }
        },
    };

    println!("got response: {}", resp.into_json::<Value>().unwrap());
    //
    // let json = json!({"query": QUERY, "variables": {"id": 385024188}});
    // //                                                           385876378
    // //                                                           385876378
    // //                                                           385024188
    // //                                                           385024188
    // //                                                           385024188
    // let resp = ureq::post("https://graphql.anilist.co/")
    //     .set("Content-Type", "application/json")
    //     .set("Accept", "application/json")
    //     .send_json(json);
    //
    // let resp = match resp {
    //     Ok(r) => r,
    //     Err(e) => match e {
    //         Error::Status(s, r) => {
    //             panic!("{}", r.into_json::<Value>().unwrap())
    //         }
    //         Error::Transport(e) => {
    //             panic!("{e}")
    //         }
    //     },
    // };

    //
    // let json = json!({"query": QUERY, "variables": {"id": 15125}});
    //
    // // let resp = client.post("https://graphql.anilist.co/")
    // //                 .header("Content-Type", "application/json")
    // //                 .header("Accept", "application/json")
    // //                 .body(json.to_string())
    // //                 .send()
    // //                 .await
    // //                 .unwrap()
    // //                 .text()
    // //                 .await;
    //
    // let resp = ureq::post("https://graphql.anilist.co/")
    //     .set("Content-Type", "application/json")
    //     .set("Accept", "application/json")
    //     .send_json(json)
    //     .unwrap()
    //     .into_string()
    //     .unwrap();
    //
    // dbg!(&resp);
    //
    // let json: Value = ureq::serde_json::from_str(&resp).unwrap();
    //
    // println!("{json}");
}
