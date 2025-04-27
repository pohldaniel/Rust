#![allow(bad_style)]

use std::os::raw::{c_float, c_uint};

pub const GL_COLOR_BUFFER_BIT: c_uint = 0x00004000;

unsafe extern "C" {
	pub fn glClearColor(r: c_float, g: c_float, b: c_float, a: c_float);
	pub fn glClear(bitmask: c_uint);
}