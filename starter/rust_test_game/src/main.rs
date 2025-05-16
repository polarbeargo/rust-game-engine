mod controller;
mod model;
use controller::SpriteController;
use model::SpriteModel;
use my_game_engine::*;
use std::sync::mpsc;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let (tx, rx) = mpsc::channel();
    let model = SpriteModel::new(tx);
    let controller = SpriteController::new(rx);

    tokio::spawn(async move {
        model.fetch_sprite_data().await;
    });

    let _window = rust_create_window("Backpack Game Window", 800, 600);
    controller.update();

    rust_terminate_glfw();
    rust_clear_screen();
    Ok(())
}
