use aria2_rs::options::TaskOptions;
use aria2_rs::{
    call::{AddUriCall, MultiCall},
    Client as Aria2Client, ConnectionMeta, SmallVec,
};
use log::{debug, info};
use reqwest::header;
use reqwest::Client;
use serde_json::Value;

pub async fn get_music_download_json(
    songs_id: i64,
    headers_token: String,
    body_token: String,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "authorization",
        format!("Bearer {}", headers_token).parse().unwrap(),
    );

    let client = Client::new();

    let payload_string = r#"{"url":"https://music.163.com/#/song?id=1490104654","level":"jymaster","type":"song","token":"92d24d0550cf28bfeac6e133decfb5a0"}"#;

    let mut payload: Value = serde_json::from_str(payload_string).unwrap();

    payload["token"] = Value::String(body_token);
    payload["url"] = Value::String(format!("https://music.163.com/song?id={}", songs_id));

    let res = match client
        .post("https://api.toubiec.cn/api/music_v1.php")
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err("CanNotDownload".into());
        }
    };

    if !res.status().is_success() {
        return Err("CanNotDownload".into());
    }

    let json: Value = res.json().await?;

    Ok(json)
}

#[derive(Debug, Default, Clone)]
pub struct MusicInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover: String, // pic url
    pub level: String,
    pub netease_id: i64,
    pub download_url: String,
    pub download_size: String,
    pub download_format: String,
    pub download_filename: String,
}
pub fn get_music_info(
    music_download_json: Value,
    netease_id: i64,
) -> Result<MusicInfo, Box<dyn std::error::Error>> {
    let json = music_download_json.clone();

    let song_info = json["song_info"].clone();
    debug!("{:?}", song_info);

    let title = match song_info["name"].as_str() {
        Some(s) => s.to_string(),
        None => return Err("CanNotDownload".into()),
    };
    let artist = song_info["artist"].as_str().unwrap().to_string();
    let album = song_info["album"].to_string();
    let cover = song_info["cover"].as_str().unwrap().to_string();
    let level = song_info["level"].as_str().unwrap().to_string();
    let netease_id = netease_id;

    let url_info = json["url_info"].clone();
    debug!("{:?}", url_info);

    let download_url = url_info["url"].as_str().unwrap().to_string();
    let download_size = url_info["size"].as_str().unwrap().to_string();
    let download_format = url_info["type"].as_str().unwrap().to_string();
    let download_filename = format!("{}/{}.{}", artist, title, download_format);

    Ok(MusicInfo {
        title,
        artist,
        album,
        cover,
        level,
        netease_id,
        download_url,
        download_size,
        download_format,
        download_filename,
    })
}

pub async fn push_to_aria2(download_url: String, download_filename: String) {
    let client = Aria2Client::connect(
        ConnectionMeta {
            url: "ws://127.0.0.1:6800/jsonrpc".to_string(),
            token: Some("token:123456".to_string()),
        },
        10,
    )
    .await
    .unwrap();

    let mut opt = TaskOptions::default();
    opt.out = Some(download_filename.parse().unwrap());

    let r = client
        .call(&AddUriCall {
            uris: SmallVec::from_iter([download_url]),
            options: Some(opt),
        })
        .await
        .unwrap();
    info!("response: {r:?}");
}
