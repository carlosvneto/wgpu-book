mod app;
mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut color_scale = "0.1";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        color_scale = &args[1];
    }
    let clr = color_scale.parse::<f32>();

    let title = "ch13 boids";

    let _ = run(clr.unwrap(), title);

    pub fn run(clr: f32, title: &'static str) -> anyhow::Result<()> {
        env_logger::init();
        
        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(clr, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
