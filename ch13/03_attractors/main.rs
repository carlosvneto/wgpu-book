mod app;
mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut num_particles = "10000";
    let mut size = "2.0";
    let opacity = 0.5;
    let mass = vec![10.0, 10.0, 10.0];

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        num_particles = &args[1];
    }
    let np = num_particles.parse::<u32>().unwrap();
    if args.len() > 2 {
        size = &args[2];
    }
    let sz = size.parse::<f32>().unwrap();

    let title = "ch13 attractors";

    let _ = run(np, sz, opacity, mass, title);

    pub fn run(
        np: u32,
        sz: f32,
        opacity: f32,
        mass: Vec<f32>,
        title: &'static str,
    ) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(np, sz, opacity, mass, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
