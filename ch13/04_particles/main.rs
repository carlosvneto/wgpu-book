mod app;
mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut num_particles = "10000";
    let mut size = "1.0";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        num_particles = &args[1];
    }
    let np = num_particles.parse::<u32>().unwrap();
    if args.len() > 2 {
        size = &args[2];
    }
    let sz = size.parse::<f32>().unwrap();

    let title = "ch13 particles";

    let _ = run(np, sz, title);

    pub fn run(np: u32, sz: f32, title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(np, sz, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
