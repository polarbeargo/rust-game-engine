use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn simulate_key_press(key: c_int, action: c_int);
    fn get_key(window: *mut std::ffi::c_void, key: c_int) -> c_int;
    fn create_game_window(
        title: *const c_char,
        width: c_int,
        height: c_int,
    ) -> *mut std::ffi::c_void;
    fn create_sprite(x: i32, y: i32, width: i32, height: i32) -> *mut std::ffi::c_void;
    fn window_should_close() -> bool;
    fn update_game_window();
    fn render_sprite(sprite: *mut std::ffi::c_void);
    fn clear_screen();
    fn update_sprite_position(sprite: *mut std::ffi::c_void, x: i32, y: i32);
    fn glfwTerminate();
}

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

pub fn rust_get_key(window: *mut std::ffi::c_void, key: c_int) -> bool {
    unsafe { get_key(window, key) == GLFW_PRESS }
}

pub fn rust_create_window(title: &str, width: i32, height: i32) -> *mut std::ffi::c_void {
    let c_title = CString::new(title).expect("Failed to convert title to CString");
    unsafe { create_game_window(c_title.as_ptr(), width, height) }
}

pub fn rust_create_sprite(x: i32, y: i32, width: i32, height: i32) -> *mut std::ffi::c_void {
    unsafe { create_sprite(x, y, width, height) }
}

pub fn rust_window_should_close() -> bool {
    unsafe { window_should_close() }
}

pub fn rust_update_game_window() {
    unsafe {
        update_game_window();
    }
}

pub fn rust_render_sprite(sprite: *mut std::ffi::c_void) {
    unsafe {
        render_sprite(sprite);
    }
}

pub fn rust_clear_screen() {
    unsafe {
        clear_screen();
    }
}

pub fn rust_update_sprite_position(sprite: *mut std::ffi::c_void, x: i32, y: i32) {
    unsafe {
        update_sprite_position(sprite, x, y);
    }
}

pub fn rust_simulate_key_press(key: c_int, action: c_int) {
    unsafe {
        simulate_key_press(key, action);
    }
}

pub fn rust_terminate_glfw() {
    unsafe {
        glfwTerminate();
    }
}
