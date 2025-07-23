use std::{fs::File, io::BufReader, path::PathBuf};

use console::graphics::App;
use winit::event_loop::EventLoop;
pub fn main() {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new(PathBuf::from("out.wasm"), true).unwrap();
    event_loop.run_app(&mut app).unwrap();
}
