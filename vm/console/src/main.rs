use console::graphics::App;
use winit::event_loop::EventLoop;
pub fn main() {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
