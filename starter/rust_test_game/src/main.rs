mod controller;
mod model;
use controller::SpriteController;
use model::SpriteModel;
use my_game_engine::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    let model = SpriteModel::new(tx);
    let mut controller = SpriteController::new(rx);

    tokio::spawn(async move {
        model.fetch_sprite_data().await;
    });

    controller.update().await;

    rust_terminate_glfw();
    rust_clear_screen();
    Ok(())
}
