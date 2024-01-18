#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::missing_const_for_fn)]

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

use ureq::json;
use url::Url;

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
    // let x = asdf();
    //
    // dbg!(x);
    //
    // let file = fs::read("./assets/backup/backup.protobuf").unwrap();
    // let bytes = prost::bytes::Bytes::from(file);
    // let backup = Backup::decode(bytes).unwrap();
    //
    // dbg!(&backup.backup_manga.len());
    //
    // let tachibk = fs::read("assets/backup.tachibk").unwrap();
    // let mut decoder = gzip::Decoder::new(&tachibk[..]).unwrap();
    // let mut file = vec![];
    // decoder.read_to_end(&mut file).unwrap();
    // let bytes = prost::bytes::Bytes::from(file);
    // let backup = Backup::decode(bytes).unwrap();
    //
    // dbg!(&backup.backup_manga.len());
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

    let x = auth();

    println!("{x}");
}

pub fn auth() -> String {
    let client_id = fs::read_to_string("assets/api-id").unwrap();
    let redirect_uri = "http://localhost:8080/callback".to_string();

    // https://anilist.co/api/v2/oauth/authorize?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code
    let url = Url::parse_with_params(
        "https://anilist.co/api/v2/oauth/authorize",
        &[
            ("client_id", client_id.to_string()),
            // ("redirect_uri", redirect_uri.to_string()),
            ("response_type", "code".to_string()),
        ],
    )
    .unwrap();

    let json = json!({
        "grant_type": "authorization_code",
        "client_id": client_id,
        "redirect_uri": redirect_uri
    });

    webbrowser::open(url.as_str());

    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    let stream = listener.incoming().next().unwrap();
    let mut stream = stream.unwrap();
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let code = request_line
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split_once('=')
        .unwrap()
        .1;

    let msg = "You can close this window now.";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length:{}\r\n\r\n{}",
        msg.len(),
        msg
    );
    stream.write_all(response.as_bytes()).unwrap();

    code.into()
}

// https://github.com/nnazo/fubuki/blob/master/src/anilist/auth.rs
// pub async fn auth() -> Result<String> {
//     let code;
//     let state = CsrfToken::new_random().secret().to_string();
//     let url_state;
//     let client_id = "2355";
//     let redirect_uri = "http://localhost:8080/callback";
//     let url = Url::parse_with_params(
//         "https://anilist.co/api/v2/oauth/authorize",
//         &[
//             ("client_id", client_id.to_string()),
//             ("redirect_uri", redirect_uri.to_string()),
//             ("response_type", "code".to_string()),
//             ("state", state.to_string()),
//         ],
//     )?;
//
//     let mut json = HashMap::new();
//     json.insert("grant_type", "authorization_code");
//     json.insert("client_id", client_id);
//     json.insert("redirect_uri", redirect_uri);
//
//     debug!("attempting to open browser to oauth URL");
//     open::that(url.to_string())?;
//
//     let listener = TcpListener::bind("127.0.0.1:8080")?;
//     for stream in listener.incoming() {
//         let mut stream = stream?;
//         debug!("found ok stream");
//         let mut reader = BufReader::new(&stream);
//         let mut request_line = String::new();
//         reader.read_line(&mut request_line)?;
//
//         if let Some(url_code) = request_line.split_whitespace().nth(1) {
//             let find_key = |key: &str, url: &Url| {
//                 if let Some(pair) = url.query_pairs().find(|pair| {
//                     let &(ref k, _) = pair;
//                     k == key
//                 }) {
//                     pair.1.into_owned()
//                 } else {
//                     String::default()
//                 }
//             };
//             let url = format!("http://localhost{}", url_code);
//             let url = Url::parse(&url)?;
//             code = find_key("code", &url);
//             url_state = find_key("state", &url);
//         } else {
//             code = String::default();
//             url_state = String::default();
//         }
//
//         if state != url_state {
//             return Err(anyhow!(
//                 "state in oauth redirect was not the same as the generated state"
//             ));
//         }
//
//         json.insert("code", &code);
//         let client = reqwest::Client::new();
//         let res = client
//             .post("https://auth.fubuki.dev/oauth/token")
//             .header("Accept", "application/json")
//             .json(&json)
//             .send()
//             .await?
//             .text()
//             .await?;
//
//         let msg = "You can close this window now.";
//         let response = format!(
//             "HTTP/1.1 200 OK\r\ncontent-length:{}\r\n\r\n{}",
//             msg.len(),
//             msg
//         );
//         stream.write_all(response.as_bytes())?;
//
//         let body: serde_json::Map<String, serde_json::Value> = serde_json::from_str(res.as_str())?;
//         if let Some(tok) = body.get("access_token") {
//             if let Some(tok) = tok.as_str() {
//                 return Ok(tok.to_string());
//             }
//         }
//         break;
//     }
//
//     Ok(String::default())
// }
