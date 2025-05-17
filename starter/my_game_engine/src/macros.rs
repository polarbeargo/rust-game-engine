#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = rust_create_sprite($x, $y, $width, $height, $r, $g, $b);
        rust_render_sprite(sprite);
    }};
}

#[macro_export]
macro_rules! tick {
    ($duration:expr) => {{
        rust_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis($duration));
    }};
}

#[macro_export]
macro_rules! on_key_press {
    ($window:expr, $key:expr, $action:block) => {{
        if rust_get_key($window, $key) == GLFW_PRESS {
            $action
        }
    }};
}

#[macro_export]
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
macro_rules! start_window_and_game_loop {
    ($title:expr, $width:expr, $height:expr, $body:block) => {{
        rust_create_window($title, $width, $height);
        while !rust_window_should_close() {
            $body
        }
    }};
}
