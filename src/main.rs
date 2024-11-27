use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::App;

mod app;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new();
    event_loop.run_app(&mut app).expect("Error in event loop");
}
