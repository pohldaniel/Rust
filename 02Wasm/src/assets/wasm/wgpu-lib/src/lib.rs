pub mod app;
pub mod graphics;

use winit::{
    event_loop::{EventLoop,  ControlFlow},
};


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use app::App;
use graphics::Graphics;

fn run_app(event_loop: EventLoop<Graphics>, app: App) {
    // Sets up panics to go to the console.error in browser environments
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Error).expect("Couldn't initialize logger");

    // Runs the app async via the browsers event loop
    use winit::platform::web::EventLoopExtWebSys;
    wasm_bindgen_futures::spawn_local(async move {
        event_loop.spawn_app(app);
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();

    let event_loop = EventLoop::<Graphics>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    run_app(event_loop, app);

    Ok(())
}
