#[path = "../common/app.rs"]
mod app;

mod state;
mod vertex;

use winit::event_loop::EventLoop;

use app::App;

fn main() {
    let title = "ch04 square using index buffer";

    let _ = run(title);

    pub fn run(title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(title);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
