use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::platform::web::WindowExtWebSys;
#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
    let event_loop = EventLoop::new().unwrap();
    let gwin = WindowBuilder::new().build(&event_loop).unwrap();
    let _ = gwin.request_inner_size(PhysicalSize::new(640, 480));
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("game")?;
            let got_canvas = gwin.canvas()?;
            got_canvas.set_width(640);
            got_canvas.set_height(480);
            let canvas = web_sys::Element::from(got_canvas);
            dst.append_child(&canvas).ok()?;
            Some(())
        })
        .expect("Couldn't append canvas to document body.");
}
