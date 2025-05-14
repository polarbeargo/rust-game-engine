use my_game_engine::*;
use reqwest::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio::main;

#[derive(Debug, Serialize, Deserialize)]
struct SpriteData {
    b: u8,
    g: u8,
    height: u32,
    r: u8,
    width: u32,
    x: u32,
    y: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let response = client
        .get("https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler")
        .send()
        .await?
        .json::<SpriteData>()
        .await?;

    println!("{:?}", response);     
    let sprite = rust_create_sprite(200.0, 200.0, 50, 50, 0, 255, 0);
    rust_update_sprite_position(
        sprite,
        response.x as f32,
        response.y as f32,
    );
    rust_clear_screen();
    Ok(())
}
