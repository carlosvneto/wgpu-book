#[path = "../05_mandelbulb/app.rs"]
mod app;

mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let max_iter: f32 = 10.0;
    let scale: f32 = 1.0;

    let title = "ch14 mandelbox";

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
