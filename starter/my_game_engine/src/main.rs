macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr) => {{
        let sprite = create_sprite($x, $y, $width, $height);
        render_sprite(sprite);
    }};
}

macro_rules! on_key_press {
    ($key:expr, $action:block) => {{
        if get_key($key) == GLFW_PRESS {
            $action
        }
    }};
}

macro_rules! tick {
    ($duration:expr) => {{
        update_game_window();
        std::thread::sleep(std::time::Duration::from_millis($duration));
    }};
}

macro_rules! start_window_and_game_loop {
    ($width:expr, $height:expr) => {{
        create_window($width, $height);
        while !window_should_close() {
            tick!(16);
        }
    }};
}

fn main() {
    start_window_and_game_loop!(800, 600);

    spawn_sprite!(100, 150, 50, 50);

    on_key_press!(GLFW_KEY_SPACE, {
        println!("Space key was pressed!");
    });
}
