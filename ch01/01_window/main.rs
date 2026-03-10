mod app;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let _ = run();

    pub fn run() -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::default();

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
