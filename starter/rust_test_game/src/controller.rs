use crate::model::SpriteData;
use my_game_engine::*;
use std::sync::mpsc;

pub struct SpriteController {
    rx: mpsc::Receiver<SpriteData>,
}

impl SpriteController {
    pub fn new(rx: mpsc::Receiver<SpriteData>) -> Self {
        Self { rx }
    }

    pub fn update(&self) {
        start_window_and_game_loop!("Backpack Game Window", 800, 600, {
            if let Ok(sprite_data) = self.rx.try_recv() {
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
            }
        });
    }
}
