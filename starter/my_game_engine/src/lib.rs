mod ffi;
#[macro_use]
mod macros;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

#[cfg(test)]
mod tests {
    use crate::ffi::*;
    use crate::macros::*;
    use crate::Sprite;

    fn setup_window() -> *mut std::ffi::c_void {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");
        window
    }

    fn run_game_loop() {
        while !rust_window_should_close() {
            rust_update_game_window();
        }
        rust_terminate_glfw();
    }

    #[test]
    fn test_create_sprite() {
        let _window = setup_window();

        let sprite = rust_create_sprite(100.0, 100.0, 50, 50, 255, 0, 0);
        assert!(!sprite.is_null(), "Sprite creation failed");

        rust_render_sprite(sprite);
        run_game_loop();
    }

    #[test]
    fn test_render_sprite() {
        let _window = setup_window();

        let sprite = rust_create_sprite(200.0, 200.0, 50, 50, 0, 255, 0);
        assert!(!sprite.is_null(), "Sprite creation failed");

        rust_render_sprite(sprite);
        rust_update_game_window();
        run_game_loop();
        rust_terminate_glfw();
    }

    #[test]
    fn test_update_sprite_position() {
        let _window = setup_window();
        let sprite = rust_create_sprite(100.0, 100.0, 50, 50, 255, 0, 0);
        assert!(!sprite.is_null(), "Sprite creation failed");

        let mut x = 100.0;
        let mut y = 100.0;

        while !rust_window_should_close() {
            rust_clear_screen();

            x += 1.0;
            y += 1.0;
            rust_update_sprite_position(sprite, x, y);

            rust_render_sprite(sprite);
            rust_update_game_window();
        }

        rust_terminate_glfw();
    }

    #[test]
    fn test_spawn_sprite() {
        let _window = setup_window();
        spawn_sprite!(100.0, 150.0, 50, 50, 255, 0, 0);
        std::thread::sleep(std::time::Duration::from_secs(2));
        rust_clear_screen();
    }

    #[test]
    fn test_tick() {
        let _window = setup_window();
        tick!(100);
        std::thread::sleep(std::time::Duration::from_secs(1));
        rust_clear_screen();
    }

    #[test]
    fn test_change_sprite_color() {
        let window = setup_window();

        let mut sprite = Sprite {
            x: 100.0,
            y: 100.0,
            width: 50,
            height: 50,
            r: 255,
            g: 0,
            b: 0,
        };

        change_sprite_color!(sprite, 0, 0, 255);
        std::thread::sleep(std::time::Duration::from_secs(2));
        run_game_loop();
        rust_clear_screen();
    }
}
