use std::process::exit;
use anyhow::anyhow;
pub use anyhow::Result;
use json::{self, JsonValue};
use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
use serde::Serialize;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut dafault_headers = HeaderMap::with_capacity(4);
    dafault_headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    dafault_headers.insert("Host", HeaderValue::from_static("music.163.com"));
    dafault_headers.insert(
        "Referer",
        HeaderValue::from_static("http://music.163.com/search/"),
    );
    dafault_headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"));
    match Client::builder().default_headers(dafault_headers).build() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("{err}");
            exit(-1)
        }
    }
});

#[derive(Serialize)]
struct Request {
    s: String,
    r#type: i64,
    offset: i64,
    limit: i64,
}

pub(super) async fn fetch_song_info(search_name: String) -> Result<JsonValue> {
    let url = "https://music.163.com/api/search/get/web";
    let req = Request {
        s: search_name,
        r#type: 1,
        offset: 0,
        limit: 10,
    };
    let resp = CLIENT
        .post(url)
        .form(&req)
        .send()
        .await
        .map_err(|err| anyhow!("[ERROR] 原因: {err}"))?
        .text()
        .await
        .map_err(|err| anyhow!("[ERROR] 原因: {err}"))?;
    Ok(json::parse(&resp)?)
}

pub(super) async fn fetch_lyric(id: i64) -> Result<JsonValue> {
    let url = format!("http://music.163.com/api/song/lyric?lv=1&kv=1&tv=-1&id={id}");
    let resp = CLIENT
        .get(url)
        .send()
        .await
        .map_err(|err| anyhow!("[ERROR] 原因: {err}"))?
        .text()
        .await
        .map_err(|err| anyhow!("[ERROR] 原因: {err}"))?;
    Ok(json::parse(&resp).map_err(|err| anyhow!("[ERROR] 原因: {err}"))?)
}
