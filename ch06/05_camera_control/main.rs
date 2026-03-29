use app::App;
use winit::event_loop::EventLoop;

mod app;
mod camera;
mod state;
mod vertex;

fn main() {
    let title = "ch06 camera control - press right mouse button";

    let _ = run(title);

    pub fn run(title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(title);

        println!("Press and hold mouse right button to move camera");

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
