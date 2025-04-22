use std::ptr;
use std::ffi::{CString};

#[allow(missing_copy_implementations)]
pub enum GLFWmonitor {}

#[allow(missing_copy_implementations)]
pub enum GLFWwindow {}

pub static GL_COLOR_BUFFER_BIT: u32 = 0x00004000;

#[link(name = "libglfw3")]
unsafe extern "C" {
    fn glfwInit() -> i32;
    fn glfwPollEvents() -> i32;
	fn glfwCreateWindow(width: i32, height: i32, title: *const i8, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
	fn glfwMakeContextCurrent(window: *mut GLFWwindow);
	fn glfwWindowShouldClose(window: *mut GLFWwindow) -> i32;
	fn glfwSwapBuffers(window: *mut GLFWwindow);
	fn glfwSetWindowSizeCallback(window: *mut GLFWwindow, onResizeCallback: extern "C" fn(window: *mut GLFWwindow, i32, i32));
}

#[link(name = "libglew")]
unsafe extern "C" { //this is some wicked mumbo-jumbo for windows macro and dll
	fn glewInit() -> i32;
}

#[link(name = "OpenGL32")]
unsafe extern "C" {
	fn glClearColor(r: f32, g: f32, b: f32, a: f32);
	fn glClear(bitmask: u32);
}

#[allow(unused_variables)]
extern "C" fn on_resize_callback(window: *mut GLFWwindow, width: i32, height: i32) {
    println!("I'm called from C with value {0} and {1}", width, height);
}

fn main() {
    unsafe {
        let string = CString::new("Rust GLFW3 Window".as_bytes()).unwrap();
		let title = string.as_bytes_with_nul().as_ptr() as *const i8;

        glfwInit();
        let window = glfwCreateWindow(1600, 900, title, ptr::null_mut(), ptr::null_mut());
        glfwSetWindowSizeCallback(window, on_resize_callback);
		glfwMakeContextCurrent(window);
		println!("GLFW window was opened!");

		if glewInit() == 0 {
			println!("GLEW initialized!");
		}
		else {
			println!("GLEW failed to initialize!");
		}

		glClearColor(0.3, 0.4, 0.1, 1.0);
		loop {
			glfwPollEvents();
			if glfwWindowShouldClose(window) == 1 {
				break;
			}

			glClear(GL_COLOR_BUFFER_BIT);
			glfwSwapBuffers(window);
		};
    };
}
