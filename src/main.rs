#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::missing_const_for_fn)]

use std::fs;
use std::io::Read;

use libflate::gzip;
use prost::Message;
use ureq::serde_json::Value;
use ureq::{json, Error};

use tachiyomi_backup_to_anilist::anilist::{get_code, get_token};
use tachiyomi_backup_to_anilist::tachiyomi::backup::Backup;

const QUERY: &str = "
query ($id: Int) {
    Media (id: $id, type: ANIME) {
        id
        title {
            romaji
            english
            native
        }
    }
}
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
