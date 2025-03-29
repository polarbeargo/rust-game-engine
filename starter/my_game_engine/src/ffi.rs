use std::os::raw::c_int;

extern "C" {
    fn get_key(window: *mut std::ffi::c_void, key: c_int) -> c_int;
}

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

pub fn rust_get_key(window: *mut std::ffi::c_void, key: c_int) -> bool {
    unsafe {
        get_key(window, key) == GLFW_PRESS
    }
}
