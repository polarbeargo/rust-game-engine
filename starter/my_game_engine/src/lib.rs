pub mod ffi;

#[cfg(test)]
mod tests {
    use crate::ffi::*;

    #[test]
    fn test_create_window() {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");
    }

    #[test]
    fn test_get_key() {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");

        let key_state = rust_get_key(window, GLFW_KEY_SPACE);
        assert_eq!(
            key_state, false,
            "Expected GLFW_KEY_SPACE to not be pressed"
        );
    }

    #[test]
    fn test_create_sprite() {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");

        let sprite = rust_create_sprite(100, 100, 50, 50);
        assert!(!sprite.is_null(), "Sprite creation failed");

        rust_render_sprite(sprite);
    }

    #[test]
    fn test_window_should_close() {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");

        let should_close = rust_window_should_close();
        assert_eq!(should_close, false, "Expected window to not be closing");
    }

    #[test]
    fn test_update_game_window() {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");

        rust_update_game_window();
    }

    #[test]
    fn test_clear_screen() {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");

        rust_clear_screen();
    }

    #[test]
    fn test_update_sprite_position() {
        let window = rust_create_window("Test Window", 800, 600);
        assert!(!window.is_null(), "Window creation failed");

        let sprite = rust_create_sprite(100, 100, 50, 50);
        assert!(!sprite.is_null(), "Sprite creation failed");

        rust_update_sprite_position(sprite, 150, 150);
    }
}
