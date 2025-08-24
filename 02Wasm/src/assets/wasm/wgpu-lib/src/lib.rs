mod app;
mod graphics;
use std::{iter, sync::Arc};
use crate::{app::App, graphics::Graphics};
use winit::event_loop::{ControlFlow, EventLoop};
#[allow(unused_imports)]
use wasm_bindgen::{prelude::wasm_bindgen, throw_str, JsCast, UnwrapThrowExt};

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

#[cfg(target_arch = "wasm32")]
const CANVAS_ID: &str = "wgpu-canvas";

/*fn run_app(event_loop: EventLoop<Graphics>, mut app: App) {
    // Allows the setting of the log level through RUST_LOG env var.
    // It also allows wgpu logs to be seen.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    // Runs the app on the current thread.
    let _ = event_loop.run_app(&mut app);
}*/

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
fn run() -> Result<(), wasm_bindgen::JsValue>{
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();

    let canvas = document.create_element("canvas").unwrap_throw();
    canvas.set_id(CANVAS_ID);
    canvas.set_attribute("width", "500").unwrap_throw();
    canvas.set_attribute("height", "500").unwrap_throw();

    let body = document
        .get_element_by_id("wasm-example")
        .unwrap_throw();
    body.append_with_node_1(canvas.unchecked_ref())
        .unwrap_throw();

    // <T> (T -> AppEvent) extends regular platform specific events (resize, mouse, etc.).
    // This allows our app to inject custom events and handle them alongside regular ones.
    // let event_loop = EventLoop::<()>::new().unwrap();
    let event_loop = EventLoop::<Graphics>::with_user_event().build().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //event_loop.set_control_flow(ControlFlow::Wait);

    let app = App::new(&event_loop);
    run_app(event_loop, app);
    Ok(())
}

fn main() {
    //wgpu_winit_example::run();
}