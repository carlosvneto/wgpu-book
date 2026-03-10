use app::App;
use winit::event_loop::EventLoop;

mod app;
mod light;
mod state;
mod vertex_cube;
mod vertex_sphere;

fn main() {
    let title = "ch12 multiple pipelines";
    let _ = run(title);

    pub fn run(title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
