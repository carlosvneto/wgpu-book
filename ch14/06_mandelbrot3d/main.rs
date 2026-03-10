#[path = "../05_mandelbulb/app.rs"]
mod app;

mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut max_iter: f32 = 3.0;
    let mut scale: f32 = 1.0;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        max_iter = args[1].parse().unwrap();
    }
    if args.len() > 2 {
        scale = args[2].parse().unwrap();
    }

    let title = "ch14 mandelbrot3d";

    let _ = run(max_iter, scale, false, title);

    pub fn run(
        max_iter: f32,
        scale: f32,
        mouse_control: bool,
        title: &'static str,
    ) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(max_iter, scale, mouse_control, title);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
