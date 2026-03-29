mod app;
mod state;
mod vertex;

use winit::event_loop::EventLoop;

use app::App;

fn main() {
    let title = "ch06 rotate cube";

    let _ = run(title);

    pub fn run(title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
