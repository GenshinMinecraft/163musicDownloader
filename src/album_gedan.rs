use reqwest::Client;
use serde_json::Value;

pub async fn get_gedan_songs(gedan_id: i64) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut song_ids = Vec::new();
    let res: Value = client
        .get(format!(
            "https://neteasecloudmusicapi.vercel.app/playlist/track/all?id={}",
            gedan_id
        ))
        .send()
        .await?
        .json()
        .await?;

    for song in res["songs"].as_array().unwrap() {
        song_ids.push(song["id"].as_i64().unwrap());
    }
    Ok(song_ids)
}
