#[path = "../01_domain_color/app.rs"]
mod app;

mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut select = "0";
    let mut select_color = "0";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        select = &args[1];
    }
    if args.len() > 2 {
        select_color = &args[2];
    }

    let title = "ch14 iterate function";
    let select_id = select.parse::<f32>();
    let color_id = select_color.parse::<f32>();

    let _ = run(select_id.expect("REASON"), color_id.expect("REASON"), title);

    pub fn run(select_id: f32, color_id: f32, title: &'static str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(select_id, color_id, title);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
