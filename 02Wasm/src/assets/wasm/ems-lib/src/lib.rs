#![no_main]

#[link(name="m")]
extern "C" {}

use std::ffi::c_void;
use std::ptr;
use std::ffi::{CString};

pub static GL_COLOR_BUFFER_BIT: u32 = 0x00004000;


#[allow(missing_copy_implementations)]
pub enum GLFWmonitor {}

#[allow(missing_copy_implementations)]
pub enum GLFWwindow {}

#[link(name = "glfw3")]
unsafe extern "C" {
    fn glfwInit() -> i32;
    fn glfwPollEvents();
	fn glfwCreateWindow(width: i32, height: i32, title: *const i8, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
	fn glfwMakeContextCurrent(window: *mut GLFWwindow);
	fn glfwWindowShouldClose(window: *mut GLFWwindow) -> i32;
	fn glfwSwapBuffers(window: *mut GLFWwindow);
	fn glfwSetWindowSizeCallback(window: *mut GLFWwindow, onResizeCallback: extern "C" fn(window: *mut GLFWwindow, i32, i32));
	fn glfwGetWindowUserPointer (window: *mut GLFWwindow) -> *mut c_void;	
	fn glfwWindowHint (hint: i32 , value: i32 )-> *mut c_void;
	fn glfwDestroyWindow (window: *mut GLFWwindow)-> *mut c_void;
	fn glfwTerminate()-> *mut c_void;
	fn glfwGetWindowSize(window: *mut GLFWwindow, width: *mut i32 , height: *mut i32 )-> *mut c_void;
}

#[link(name = "gl")]
unsafe extern "C" {
	fn glClearColor(r: f32, g: f32, b: f32, a: f32);
	fn glClear(bitmask: u32);
}

#[cfg(target_arch = "wasm32")]
type EmArgCallbackFunc = extern "C" fn(*mut c_void);

#[cfg(target_arch = "wasm32")]
type EmCallbackFunc = extern "C" fn();


#[cfg(target_arch = "wasm32")]
extern "C" {
    fn emscripten_set_main_loop_arg(func: EmArgCallbackFunc, arg: *mut c_void, fps: i32, simulate_infinite_loop: i32);
	fn emscripten_set_main_loop(func: EmCallbackFunc, fps: i32, simulate_infinite_loop: i32);
}

extern "C"
fn main_loop(arg: *mut c_void) {
    unsafe {	
	   glClearColor(0.3, 0.4, 0.1, 1.0);
	   glClear(GL_COLOR_BUFFER_BIT);
	   glfwPollEvents();
	   glfwSwapBuffers(arg as *mut GLFWwindow);
    }
}

extern "C"
fn run()  {
    unsafe {
		let string = CString::new("Rust GLFW3 Window".as_bytes()).unwrap();
		let title = string.as_bytes_with_nul().as_ptr() as *const i8;
		glfwInit();		
        let window = glfwCreateWindow(1600, 900, title, ptr::null_mut(), ptr::null_mut());
		glfwMakeContextCurrent(window);				
		emscripten_set_main_loop_arg(main_loop, window as *mut c_void, 0, 1);    
       
    };
}

#[link(name="c")]
#[no_mangle]
pub extern "C"
fn main(_argc: isize, _argv: *const *const u8) -> isize  {
	run();
	return 0;
}
