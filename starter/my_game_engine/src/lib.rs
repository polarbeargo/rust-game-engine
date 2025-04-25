pub mod ffi;

#[cfg(test)]
mod tests {
    use crate::ffi::*;

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
    fn test_create_window() {
        let _window = setup_window();
        run_game_loop();
    }

    #[test]
    fn test_get_key() {
        let window = setup_window();
        rust_simulate_key_press(GLFW_KEY_SPACE, GLFW_PRESS);

        let key_state = rust_get_key(window, GLFW_KEY_SPACE);
        assert_eq!(key_state, true, "Expected GLFW_KEY_SPACE to be pressed");

        run_game_loop();
    }

    #[test]
    fn test_create_sprite() {
        let _window = setup_window();

        let sprite = rust_create_sprite(100, 100, 50, 50);
        assert!(!sprite.is_null(), "Sprite creation failed");

        rust_render_sprite(sprite);
        run_game_loop();
    }

    #[test]
    fn test_window_should_close() {
        let _window = setup_window();

        let should_close = rust_window_should_close();
        assert_eq!(should_close, false, "Expected window to not be closing");

        run_game_loop();
    }

    #[test]
    fn test_update_game_window() {
        let _window = setup_window();
        rust_update_game_window();
        run_game_loop();
    }

    #[test]
    fn test_clear_screen() {
        let _window = setup_window();
        rust_clear_screen();
        run_game_loop();
    }

    #[test]
    fn test_update_sprite_position() {
        let _window = setup_window();

        let sprite = rust_create_sprite(100, 100, 50, 50);
        assert!(!sprite.is_null(), "Sprite creation failed");

        rust_update_sprite_position(sprite, 150, 150);
        run_game_loop();
    }
}
