#[path = "../05_chart_pipelines/app.rs"]
mod app;

mod light;
mod state;
mod vertex_mesh;
mod vertex_surface;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let mut colormap_name = "jet";
    let mut color = "1.0,1.0,1.0";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        colormap_name = &args[1];
    }
    if args.len() > 2 {
        color = &args[2];
    }
    let clr = color
        .split(",")
        .filter_map(|s| s.parse::<f32>().ok())
        .collect::<Vec<_>>();

    let title = "ch12 multiple render passes";

    let _ = run(colormap_name, clr, title);

    pub fn run(colormap_name: &str, clr: Vec<f32>, title: &str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(colormap_name, clr, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
