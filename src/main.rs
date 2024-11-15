mod album_gedan;
use album_gedan::*;
mod utils;

use log::{debug, info};
use serde_json::Value;
use utils::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let musics = get_gedan_songs(12216954372).await?;

    for music in musics {
        debug!("{}", music);
        let json = match get_music_download_json(
            music,
            String::from("18e6bf629a92e8cd1ccb776902f3d4a1"),
            String::from("daa7751e3e8e5eb17829cf4ae5efe76f"),
        )
        .await
        {
            Ok(json) => json,
            Err(e) => continue,
        };
        let json = match get_music_info(json, music) {
            Ok(json) => json,
            Err(e) => continue,
        };
        info!("{:?}", json);
        push_to_aria2(json.download_url, json.download_filename).await;
    }

    Ok(())
}
