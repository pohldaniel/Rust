#![allow(bad_style)]

use std::os::raw::{c_int, c_void};

pub type EmArgCallbackFunc = extern "C" fn(*mut c_void);
pub type EmCallbackFunc = extern "C" fn();

unsafe extern "C" {
    pub fn emscripten_set_main_loop_arg(func: EmArgCallbackFunc, arg: *mut c_void, fps: c_int, simulate_infinite_loop: c_int);
	pub fn emscripten_set_main_loop(func: EmCallbackFunc, fps: c_int, simulate_infinite_loop: c_int);
}