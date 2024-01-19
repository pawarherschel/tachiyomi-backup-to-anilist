use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use ureq::serde_json::Value;
use ureq::{json, Error};
use url::Url;

pub fn get_code() -> String {
    let client_id = fs::read_to_string("assets/api-id").unwrap();

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

    webbrowser::open(url.as_str()).unwrap();

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

pub fn get_token(code: String) -> String {
    let uri = "https://anilist.co/api/v2/oauth/token";
    let client_id = fs::read_to_string("assets/api-id").unwrap();
    let client_secret = fs::read_to_string("assets/api-secret").unwrap();
    let json = json!({
        "grant_type": "authorization_code",
        "client_id": client_id,
        "client_secret": client_secret,
        "redirect_uri": "http://localhost:1234",
        "code": code
    });

    let resp = ureq::post("https://anilist.co/api/v2/oauth/token")
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .send_json(json);

    let resp = match resp {
        Ok(r) => r,
        Err(e) => match e {
            Error::Status(s, r) => {
                panic!("{}", r.into_json::<Value>().unwrap())
            }
            Error::Transport(e) => {
                panic!("{e}")
            }
        },
    };

    let resp = resp.into_json::<Value>().unwrap();

    let token = resp.get("access_token").unwrap();

    token.to_string().trim_matches('"').to_string()
}
