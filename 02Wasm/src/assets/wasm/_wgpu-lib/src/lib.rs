use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;
use wasm_bindgen::JsValue;
mod gfx_state;
use crate::gfx_state::GFXState;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId, WindowAttributes};
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;

#[cfg(target_arch = "wasm32")]
const CANVAS_ID: &str = "wgpu-canvas";

#[derive(Default)]
pub struct Application2<'a>  {
    window_attributes: WindowAttributes,
    window: Option<Arc<Window>>,
    gfx_state: Option<GFXState<'a>>,
}

impl<'a>  Application2<'a>  {
    fn default() -> Self {
        Self { window_attributes: Window::default_attributes().with_inner_size(PhysicalSize::new(450, 400)),
               window: None,
               gfx_state: None 
             }
    }

    pub fn gfx_state_from_window(window: Arc<Window>, size: PhysicalSize<u32>) -> GFXState<'static> {
        pollster::block_on(GFXState::new(window, size))
    }
}

impl<'a> ApplicationHandler for Application2<'a>  {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("INIT");

        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        use web_sys::wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;

        web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = doc.create_element("canvas").unwrap();
                    canvas.set_attribute("width", "500").unwrap();
                    canvas.set_attribute("height", "500").unwrap();
                    let html_canvas_element = canvas.clone().unchecked_into();
                    window_attributes = window_attributes.clone().with_canvas(Some(html_canvas_element));
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");

        let window =  Arc::new(event_loop.create_window(window_attributes).unwrap());

        let gfx_state = Application2::gfx_state_from_window(window.clone(), PhysicalSize::new(500, 500));
        self.window = Some(window);
        self.gfx_state = Some(gfx_state);      
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId,event: winit::event::WindowEvent) {
        log::info!("EVENT");
        match &event {
            WindowEvent::Resized(physical_size) => {
                //if let Some(ref mut gfx_state) = self.gfx_state {
                //    gfx_state.resize(*physical_size);
                //}
            }
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                //if input_handler::process_inputs(event) {
                //    event_loop.exit()
                //}
            }

            WindowEvent::RedrawRequested => {
                //TODO: Add flag for checking if surface has been configured
                if let Some(ref window) = &self.window {
                    window.request_redraw();

                    if let Some(ref mut gfx_state) = self.gfx_state {
                        match gfx_state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                gfx_state.resize(window.inner_size());
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                log::error!("Out of memory!");
                                event_loop.exit();
                            }
                            Err(e) => log::error!("{:?}", e),
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    let event_loop = EventLoop::new().unwrap();
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);
    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = Application2::default();
    event_loop.run_app(&mut app);
}