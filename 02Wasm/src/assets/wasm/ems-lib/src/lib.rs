use glfw3;
use gl;
use ems;
use crate::gl::GL_COLOR_BUFFER_BIT;

use std::ffi::c_void;
use std::ptr;
use std::ffi::{CString};

extern "C"
fn main_loop(arg: *mut c_void) {
    unsafe {	
	   gl::glClearColor(0.3, 0.4, 0.1, 1.0);
	   gl::glClear(GL_COLOR_BUFFER_BIT);
	   glfw3::glfwPollEvents();
	   glfw3::glfwSwapBuffers(arg as *mut glfw3::GLFWwindow);
    }
}

extern "C"
fn run()  {
    unsafe {
		let string = CString::new("Rust GLFW3 Window".as_bytes()).unwrap();
		let title = string.as_bytes_with_nul().as_ptr() as *const i8;
		glfw3::glfwInit();		
        let window = glfw3::glfwCreateWindow(1600, 900, title, ptr::null_mut(), ptr::null_mut());
		glfw3::glfwMakeContextCurrent(window);				
		ems::emscripten_set_main_loop_arg(main_loop, window as *mut c_void, 0, 1);    
       
    };
}

#[no_mangle]
pub extern "C"
fn main(_argc: isize, _argv: *const *const u8) -> isize  {
	run();
	return 0;
}
