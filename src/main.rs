use std::fs;
use std::io::Read;

use libflate::gzip;
use prost::Message;
use ureq::json;
use ureq::serde_json::Value;

use tachiyomi_backup_to_anilist::asdf;
use tachiyomi_backup_to_anilist::tachiyomi::backup::Backup;

const QUERY: &str = "
query ($id: Int) { # Define which variables will be used in the query (id)
  Media (id: $id, type: ANIME) { # Insert our variables into the query arguments (id) (type: ANIME is hard-coded in the query)
    id
    title {
      romaji
      english
      native
    }
  }
}
";

fn main() {
    let x = asdf();

    dbg!(x);

    let file = fs::read("./assets/backup/backup.protobuf").unwrap();
    let bytes = prost::bytes::Bytes::from(file);
    let backup = Backup::decode(bytes).unwrap();

    dbg!(&backup.backup_manga.len());

    let tachibk = fs::read("assets/backup.tachibk").unwrap();
    let mut decoder = gzip::Decoder::new(&tachibk[..]).unwrap();
    let mut file = vec![];
    decoder.read_to_end(&mut file).unwrap();
    let bytes = prost::bytes::Bytes::from(file);
    let backup = Backup::decode(bytes).unwrap();

    dbg!(&backup.backup_manga.len());

    let json = json!({"query": QUERY, "variables": {"id": 15125}});

    // let resp = client.post("https://graphql.anilist.co/")
    //                 .header("Content-Type", "application/json")
    //                 .header("Accept", "application/json")
    //                 .body(json.to_string())
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .text()
    //                 .await;

    let resp = ureq::post("https://graphql.anilist.co/")
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .send_json(json)
        .unwrap()
        .into_string()
        .unwrap();

    dbg!(&resp);

    let json: Value = ureq::serde_json::from_str(&resp).unwrap();

    println!("{json}");
}
