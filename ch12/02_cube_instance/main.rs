#[path = "../01_two_cubes/app.rs"]
mod app;

mod state;
mod vertex;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
    let title = "ch12 cube instance";

    let _ = run(title);

    pub fn run(title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
