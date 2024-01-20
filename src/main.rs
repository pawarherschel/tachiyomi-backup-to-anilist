#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::missing_const_for_fn)]

use std::collections::HashMap;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs, time};

use indicatif::ProgressIterator;
use libflate::gzip;
use prost::Message;
use ureq::serde_json::Value;
use ureq::{json, Error, Response};

use tachiyomi_backup_to_anilist::anilist::{get_code, get_token};
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

// 700 milliseconds delay per request

pub fn try_again(name: String) -> Result<Response, ureq::Error> {
    let variables = json!({
        "search": name.as_str(),
        "mediaType": "MANGA"
    });
    let json = json!({
        "query": SEARCH_QUERY,
        "variables": variables
    });
    let resp = ureq::post("https://graphql.anilist.co")
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .send_json(json);
    resp
}

#[allow(clippy::too_many_lines)]
pub fn rate_limited_query(name: String, cache: &mut HashMap<String, Value>) -> Option<u64> {
    if let Some(data) = cache.get(&name) {
        let binding = data["data"]["Page"]["media"].clone();
        let list = binding.as_array().unwrap();
        let list = list
            .into_iter()
            .filter(|it| it["format"] == "MANGA")
            .collect::<Vec<_>>();

        let found = list.iter().find(|it| {
            let romaji = &it["title"]["romaji"];
            let english = &it["title"]["english"];
            let native = &it["title"]["native"];
            let user_preferred = &it["title"]["user_preferred"];
            match &name {
                romanji => true,
                english => true,
                native => true,
                user_preferred => true,
                _ => false,
            }
        });

        return found.map(|m| m["id"].as_number().unwrap().as_u64().unwrap());
    }

    let variables = json!({
        "search": name.as_str(),
        "mediaType": "MANGA"
    });
    let json = json!({
        "query": SEARCH_QUERY,
        "variables": variables
    });
    let resp = ureq::post("https://graphql.anilist.co")
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .send_json(json.clone());

    let mut now = time::Instant::now();

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => match e {
            Error::Status(429, r) => {
                let headers = r.headers_names();
                for header in headers {
                    let header_details = r.header(header.as_str());
                    println!("{header}: {header_details:?}");
                }
                let retry_after = r.header("retry-after").unwrap().parse().unwrap();
                eprintln!("rate limit exceeded: {}", r.into_json::<Value>().unwrap());
                sleep(Duration::from_secs(retry_after));
                return rate_limited_query(name, cache);
            }
            Error::Status(s, r) => {
                let headers = r.headers_names();
                let headers = headers
                    .iter()
                    .map(|it| r.header(it.as_str()))
                    .map(|it| it.unwrap_or_default().to_string())
                    .collect::<Vec<_>>();

                println!(
                    "response: {}\n\
                    headers: {:?}\n\
                    json: {:?}\n\
                    ---- retrying",
                    r.into_json::<Value>().unwrap(),
                    headers,
                    json
                );
                let elapsed = now.elapsed();
                let sleep_time = Duration::from_secs_f32(2.0).saturating_sub(elapsed);

                if sleep_time < Duration::from_millis(500) {
                    println!("{headers:?}");
                    println!("slept for {sleep_time:?}");
                }

                sleep(sleep_time);

                now = Instant::now();

                let response = try_again(name.clone());

                match response {
                    Ok(r) => {
                        println!("!!!!worked");
                        r
                    }
                    Err(e) => match e {
                        Error::Status(500, _) => return None,
                        Error::Status(s, r) => {
                            panic!("different error fml\n{}", r.into_json::<Value>().unwrap())
                        }
                        Error::Transport(_) => {
                            panic!()
                        }
                    },
                }
            }
            Error::Transport(e) => {
                panic!("{e}");
            }
        },
    };

    let headers = resp.headers_names();
    let headers = headers
        .into_iter()
        .map(|it| {
            (
                it.clone(),
                resp.header(it.as_str()).unwrap_or_default().to_string(),
            )
        })
        .collect::<Vec<_>>();

    let ratelimit_remaining = resp
        .header("x-ratelimit-remaining")
        .unwrap_or_default()
        .parse()
        .unwrap_or(0_u32);

    if ratelimit_remaining < 30 {
        println!("x-ratelimit-remaining: {ratelimit_remaining}",);
    }

    let data = resp.into_json::<Value>().unwrap();
    let binding = data["data"]["Page"]["media"].clone();
    let list = binding.as_array().unwrap();
    let list = list
        .into_iter()
        .filter(|it| it["format"] == "MANGA")
        .collect::<Vec<_>>();

    let found = list.iter().find(|it| {
        let romaji = &it["title"]["romaji"];
        let english = &it["title"]["english"];
        let native = &it["title"]["native"];
        let user_preferred = &it["title"]["user_preferred"];
        match &name {
            romanji => true,
            english => true,
            native => true,
            user_preferred => true,
            _ => false,
        }
    });

    cache.insert(name, data);

    let elapsed = now.elapsed();
    let sleep_time = Duration::from_secs_f32(2.0).saturating_sub(elapsed);

    if sleep_time < Duration::from_millis(500) {
        println!("{headers:?}");
        println!("slept for {sleep_time:?}");
    }

    sleep(sleep_time);

    found.map(|m| m["id"].as_number().unwrap().as_u64().unwrap())
}

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
    //
    // let ten = not_tracked
    //     .iter()
    //     .map(|it| it.title.clone())
    //     .take(10)
    //     .collect::<Vec<_>>();
    // dbg!(ten);
    //
    // separator();
    //
    // let set = backup_manga
    //     .iter()
    //     .map(tachiyomi_backup_to_anilist::tachiyomi_backup::BackupManga::title)
    //     .map(|it| remove_accents(it.to_string()))
    //     .filter(|(_, it, _)| *it)
    //     .flat_map(|(_, _, it)| it)
    //     .collect::<BTreeSet<_>>();
    //
    // println!("{set:?}");
    //
    // separator();

    let mut cache = HashMap::new();

    let x_none = time_it! {at once | "x_none" =>
        backup_manga
            .iter()
            .progress_with(get_pb(backup_manga.len().try_into().unwrap(), ""))
            .map(|it| {
                (
                    it.clone(),
                    rate_limited_query(it.title().to_string(), &mut cache),
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
                    rate_limited_query(it.title().to_string(), &mut cache),
                )
            })
            .filter(|(_, it)| it.is_some())
            .collect::<Vec<_>>()
    };

    write_items_to_file!(x_none);
    write_items_to_file!(x_some);
    write_items_to_file!(cache);

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
