use crate::app::raytracer::Raytracer;
use clap::Parser;
use softbuffer::{Context, Surface};
use std::cell::{LazyCell, OnceCell};
use std::num::{NonZero, NonZeroUsize};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

mod raytracer;

pub struct App {
    raytracer: Raytracer,
    window: Option<Arc<Window>>,
    width: u32,
    height: u32,
    context: OnceCell<Context<Arc<Window>>>,
    surface: OnceCell<Surface<Arc<Window>, Arc<Window>>>,
}

impl App {
    pub fn new() -> Self {
        let args = Args::parse();
        let raytracer = Raytracer::new(args.width, args.height, args.bounces);
        rayon::ThreadPoolBuilder::default()
            .num_threads(args.thread_count.get())
            .build_global()
            .unwrap();

        Self {
            raytracer,
            window: None,
            width: args.width,
            height: args.height,
            context: Default::default(),
            surface: Default::default(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested
                if self.window.as_ref().is_some_and(|w| w.id() == window_id) =>
            {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_ref() {
                    let context = self.context.get_or_init(|| {
                        Context::new(window.clone())
                            .expect("Failed to create window buffer context")
                    });
                    let _surface = self.surface.get_or_init(|| {
                        Surface::new(context, window.clone())
                            .expect("Failed to create window surface")
                    });
                    let surface = self.surface.get_mut().unwrap();
                    let mut data = surface.buffer_mut().expect("Failed to get surface buffer");
                    self.raytracer.raytrace(&mut data);
                }
            }
            _ => {}
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = std::thread::available_parallelism().unwrap_or(NonZero::new(1usize).unwrap())
    )]
    thread_count: NonZeroUsize,
    #[clap(short, long, default_value = "1920")]
    width: u32,
    #[clap(short, long, default_value = "1080")]
    height: u32,
    #[clap(short, long, default_value = "4")]
    bounces: u32,
}
