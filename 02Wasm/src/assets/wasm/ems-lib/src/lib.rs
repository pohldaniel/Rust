mod renderer;
mod gles2_renderer;
mod objloader;

use glfw3::*;
use ems::*;
use gl::*;
use gles2_renderer::*;
use renderer::*;
use objloader::*;
use rs_ctypes::*;
use rs_alloc::*;
use rs_math3d::*;
use std::ptr;
use std::ffi::{CString};

pub struct State {
    program : Option<Box<dyn Program>>,
    monkey_vb   : StaticVertexBuffer,
    monkey_ib   : StaticIndexBuffer,
    angle   : f32,
}

struct Uniforms {
    pvm         : Mat4f,
}

impl UniformBlock for Uniforms {
    fn descriptors() -> Vec<UniformDataDesc> {
        let mut v = Vec::new();
        v.push(UniformDataDesc::new(String::from("uPVM"), UniformDataType::Float4x4, 0, 0));
        v
    }
}

static VERTEX_SHADER : &'static str = "
attribute highp vec4 aPosition;
attribute highp vec3 aNormal;

uniform highp mat4 uPVM;

varying highp vec3 vNormal;

void main() {
    gl_Position = uPVM * aPosition;
    vNormal = aNormal;
}\0";

static FRAGMENT_SHADER : &'static str = "
precision mediump float;
varying highp vec3 vNormal;
void main() {
    gl_FragColor = vec4(vNormal.xyz, 1.0);
}\0";

extern "C"
fn main_loop(win_: *mut c_void) {
    unsafe {	
		let win = win_ as *mut GLFWwindow;
        let state = glfwGetWindowUserPointer(win) as *mut State;

        let mut width = 990;
        let mut height = 720;
        glViewport(0, 0, width, height);
        glClearColor(0.0, 0.0, 0.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glEnable(GL_DEPTH_TEST);
        let proj = rs_math3d::perspective(45.0, 640 as f32 / 480 as f32, 1., 100.0);
        let view = rs_math3d::lookat(&Vec3f::new(0.0, 0.0, 15.0), &Vec3f::new(0.0, 0.0, 0.0), &Vec3f::new(0.0, 1.0, 0.0));
	
        let model =rs_math3d::translate(Vec3f::new(0.0, 0.0, 10.0)) * Quatf::of_axis_angle(&Vec3f::new(0.0, 1.0, 0.0), (*state).angle).mat4();
        (*state).angle += 0.01;
        let u = Uniforms { pvm: proj * view * model };

        match &(*state).program {
            Some(p) => {
                glEnable(GL_CULL_FACE);
                draw_indexed(p, &(*state).monkey_vb, &(*state).monkey_ib, &u);
            },
            None => ()
        }

        glfwSwapBuffers(win);
        glfwPollEvents();
    }
}

extern "C"
fn run()  {
    unsafe {	
		glfwInit();		
		glfwWindowHint(GLFW_CONTEXT_CREATION_API as rs_ctypes::c_int, GLFW_EGL_CONTEXT_API as rs_ctypes::c_int);
        glfwWindowHint(GLFW_CLIENT_API as rs_ctypes::c_int, GLFW_OPENGL_ES_API as rs_ctypes::c_int);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR as rs_ctypes::c_int, 2);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR as rs_ctypes::c_int, 0);
        glfwWindowHint(GLFW_SAMPLES as rs_ctypes::c_int, 8);
        glfwWindowHint(GLFW_ALPHA_BITS as rs_ctypes::c_int, 0);

        let win = glfwCreateWindow(640, 480,
            "App\0".as_bytes().as_ptr() as *const u8 as *const i8,
            core::ptr::null::<GLFWmonitor>() as *mut GLFWmonitor,
            core::ptr::null::<GLFWwindow>() as *mut GLFWwindow);
        glfwMakeContextCurrent(win);
		
        let attribs = [
            VertexAttributeDesc::new(String::from("aPosition"), VertexFormat::Float3, 0),
            VertexAttributeDesc::new(String::from("aNormal"), VertexFormat::Float3, 12),
            ];
        let uniforms = [ UniformDesc::new(String::from("uPVM"), UniformDataType::Float4x4, 0) ];
        let program = GLProgram::load_program(&VERTEX_SHADER, &FRAGMENT_SHADER, &attribs, &uniforms);

        let m =
            match Mesh::read_obj("suzane.obj") {
                Ok(m) => {
                    println!("verts     : {}\nuvws      : {}\ntris      : {}\nquads     : {}", m.verts().len(), m.uvws().len(), m.tris().len(), m.quads().len());
                    GPUMesh::from(&m)
                },
                _ => panic!("Error reading file")
            };

        let monkey_vb = StaticVertexBuffer::new(m.verts());
        let monkey_ib = StaticIndexBuffer::new(m.tris());

        let state = Box::new(State { program : program, monkey_vb: monkey_vb, monkey_ib: monkey_ib, angle: 0.0 });
        glfwSetWindowUserPointer(win, state.as_ref() as *const State as *mut ::core::ffi::c_void);
		emscripten_set_main_loop_arg(main_loop, win as *mut c_void, 0, 1);       
    };
}

#[unsafe(no_mangle)]
pub extern "C"
fn main(_argc: isize, _argv: *const *const u8) -> isize  {
	run();
	return 1;
}
