use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::time::Duration;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteData {
    pub b: u8,
    pub g: u8,
    pub height: u32,
    pub r: u8,
    pub width: u32,
    pub x: u32,
    pub y: u32,
}

pub struct SpriteModel {
    client: Client,
    tx: mpsc::Sender<SpriteData>,
}

impl SpriteModel {
    pub fn new(tx: mpsc::Sender<SpriteData>) -> Self {
        let client = Client::new();
        Self { client, tx }
    }

    pub async fn fetch_sprite_data(&self) {
        loop {
            let response: SpriteData = self
                .client
                .get(
                    "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler",
                )
                .send()
                .await
                .expect("Failed to send request")
                .json()
                .await
                .expect("Failed to parse JSON");

            println!("network thread: {:?}", response);
            self.tx.send(response).expect("Failed to send sprite data");

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
