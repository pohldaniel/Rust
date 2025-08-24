

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::{
    gfx_state::GFXState,

};

pub struct Application<'a> {
    window_attributes: WindowAttributes,
    gfx_state: Option<GFXState<'a>>,
    window: Option<Arc<Window>>,
}

impl<'a> Application<'a> {
    pub fn new(title: &str) -> Self {
        Self {
            window_attributes: Window::default_attributes().with_title(title),
            gfx_state: None,
            window: None,
        }
    }

    pub fn gfx_state_from_window(window: Arc<Window>, size: PhysicalSize) -> GFXState<'static> {
        pollster::block_on(GFXState::new(window, size))
    }
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(self.window_attributes.clone()).unwrap());
        let gfx_state = Application::gfx_state_from_window(window.clone());

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        self.window = Some(window);
        self.gfx_state = Some(gfx_state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId,event: winit::event::WindowEvent) {
        log::info!("#########");
        match &event {
            WindowEvent::Resized(physical_size) => {
                if let Some(ref mut gfx_state) = self.gfx_state {
                    gfx_state.resize(*physical_size);
                }
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