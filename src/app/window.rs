use winit::dpi::{PhysicalPosition, PhysicalSize, Position};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;

pub fn create_window(
    width: u32,
    height: u32,
    event_loop: &ActiveEventLoop,
) -> Result<winit::window::Window, winit::error::OsError> {
    let monitor = event_loop.primary_monitor().unwrap_or_else(|| {
        event_loop
            .available_monitors()
            .next()
            .expect("No monitor available")
    });
    let screen_size = monitor.size();
    let window_width = width.min(screen_size.width);
    let window_height = height.min(screen_size.height);
    let resolution = winit::dpi::Size::Physical(PhysicalSize::new(window_width, window_height));

    let attributes = WindowAttributes::default()
        .with_resizable(false)
        .with_title(env!("CARGO_PKG_NAME"))
        .with_position(center_position(
            screen_size,
            monitor,
            window_width,
            window_height,
        ))
        .with_max_inner_size(resolution)
        .with_transparent(false)
        .with_visible(true)
        .with_inner_size(resolution);
    event_loop.create_window(attributes)
}

fn center_position(
    screen_size: PhysicalSize<u32>,
    monitor: winit::monitor::MonitorHandle,
    width: u32,
    height: u32,
) -> Position {
    Position::Physical(PhysicalPosition::new(
        (screen_size.width.saturating_sub(width) as f64 / 2. + monitor.position().x as f64) as i32,
        (screen_size.height.saturating_sub(height) as f64 / 2. + monitor.position().y as f64)
            as i32,
    ))
}
