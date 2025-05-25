#[macro_export]
/// The `spawn_sprite!` macro creates a sprite with the given parameters and renders it.
/// It takes the x and y coordinates, width and height of the sprite, and its color (RGB).
/// It uses the `rust_create_sprite` and `rust_render_sprite` functions to create and render the sprite.
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = rust_create_sprite($x, $y, $width, $height, $r, $g, $b);
        rust_render_sprite(sprite);
    }};
}

#[macro_export]
/// The `tick!` macro is used to update the game window and sleep for a specified duration.
/// It takes a duration in milliseconds as an argument.
/// It uses the `rust_update_game_window` function to update the window and `std::thread::sleep`
/// to pause the execution for the specified duration.
/// This is useful for controlling the frame rate of the game loop.
macro_rules! tick {
    ($duration:expr) => {{
        rust_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis($duration));
    }};
}

#[macro_export]
/// The `on_key_press!` macro checks if a specific key is pressed.
/// It takes a window reference, a key code, and a block of code to execute if the key is pressed.
/// It uses the `rust_get_key` function to check the key state.
/// This is useful for handling user input in the game loop.
macro_rules! on_key_press {
    ($window:expr, $key:expr, $action:block) => {
        if rust_get_key($window, $key) == GLFW_PRESS
            $action
    };
}

#[macro_export]
/// The `change_sprite_color!` macro changes the color of a sprite.
/// It takes a sprite reference and RGB values as arguments.
/// It uses the `spawn_sprite!` macro to create a new sprite with the same position and size,
/// but with the new color.
/// This is useful for changing the appearance of a sprite dynamically.
macro_rules! change_sprite_color {
    ($sprite:expr, $r:expr, $g:expr, $b:expr) => {{
        spawn_sprite!(
            $sprite.x,
            $sprite.y,
            $sprite.width,
            $sprite.height,
            $r,
            $g,
            $b
        );
    }};
}

#[macro_export]
/// The `start_window_and_game_loop!` macro initializes the game window and starts the game loop.
/// It takes the window title, width, height, and a block of code to execute in the game loop.
/// It uses the `rust_create_window` function to create the window and `rust_window_should_close`
/// to check if the window should close.
/// This is the main entry point for the game loop, where the game logic and rendering occur.
/// The block of code passed to the macro is executed repeatedly until the window is closed.
/// This is useful for setting up the game environment and running the main game loop.
macro_rules! start_window_and_game_loop {
    ($title:expr, $width:expr, $height:expr, $body:block) => {{
        rust_create_window($title, $width, $height);
        while !rust_window_should_close() {
            $body
        }
    }};
}
