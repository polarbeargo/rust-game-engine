use crate::model::SpriteData;
use my_game_engine::*;
use std::process;
use tokio::sync::mpsc;

pub struct SpriteController {
    rx: mpsc::Receiver<SpriteData>,
}

impl SpriteController {
    pub fn new(rx: mpsc::Receiver<SpriteData>) -> Self {
        Self { rx }
    }

    pub async fn update(&mut self) {
        start_window_and_game_loop!("Rubric Game Window", 800, 600, {
            while let Some(sprite_data) = self.rx.recv().await {
                println!("main thread: {:?}", sprite_data);

                let sprite = rust_create_sprite(
                    sprite_data.x as f32,
                    sprite_data.y as f32,
                    sprite_data.width as i32,
                    sprite_data.height as i32,
                    sprite_data.r.into(),
                    sprite_data.g.into(),
                    sprite_data.b.into(),
                );

                rust_update_sprite_position(sprite, sprite_data.x as f32, sprite_data.y as f32);
                rust_render_sprite(sprite);
                rust_update_game_window();
                on_key_press!(rust_get_window_ptr(), GLFW_KEY_ESCAPE, {
                    process::exit(0);
                });
            }
        });
    }
}
