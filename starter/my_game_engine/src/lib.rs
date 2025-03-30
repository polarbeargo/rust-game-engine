#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_game_loop() {
        
        create_window(800, 600);

        while !window_should_close() {
            update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[test]
    fn test_sprite_rendering() {
     
        create_window(800, 600);
        let sprite = create_sprite(100, 100, 50, 50);
        render_sprite(sprite);

        std::thread::sleep(std::time::Duration::from_secs(2));
        close_window();
    }

    #[test]
    fn test_screen_clearing() {
        create_window(800, 600);
 
        let red_sprite = create_sprite(100, 100, 50, 50);
        render_sprite(red_sprite);

        std::thread::sleep(std::time::Duration::from_secs(5)); 
        clear_screen();

        let green_sprite = create_sprite(100, 100, 50, 50);
        render_sprite(green_sprite);

        std::thread::sleep(std::time::Duration::from_secs(2)); 

        close_window();
    }

    #[test]
    fn test_key_presses() {
 
        create_window(800, 600);
        
        let mut key_pressed = false;

  
        while !window_should_close() {
            if get_key(GLFW_KEY_SPACE) == GLFW_PRESS {
                key_pressed = true;
            }

            if key_pressed {
                break;
            }

            update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        close_window();
    }

    #[test]
    fn test_sprite_position_update() {
        create_window(800, 600);
        
        let sprite = create_sprite(100, 100, 50, 50); 
        let mut position = 0;

        while !window_should_close() {
           
            update_sprite_position(sprite, position, 100); 
            render_sprite(sprite);

            std::thread::sleep(std::time::Duration::from_millis(16));

            position += 1;
            clear_screen();
        }

        close_window();
    }
}
