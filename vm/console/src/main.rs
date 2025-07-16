use std::{fs::File, io::BufReader};

use console::graphics::App;
use winit::event_loop::EventLoop;
pub fn main() {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let file = File::open("out.wasm").unwrap();
    let mut reader = BufReader::new(file);

    let mut app = App::new(&mut reader).unwrap();
    event_loop.run_app(&mut app).unwrap();
}
