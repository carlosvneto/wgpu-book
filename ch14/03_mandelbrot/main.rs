mod app;
mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut select_color = "0";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        select_color = &args[1];
    }

    let title = "ch14 madelbrot";
    let color_id = select_color.parse::<f32>();

    let cx = -0.5f32;
    let cy = 0.0f32;

    let _ = run(color_id.expect("REASON"), cx, cy, title);

    pub fn run(color_id: f32, cx: f32, cy: f32, title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(color_id, cx, cy, title);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
