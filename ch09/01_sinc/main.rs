#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

#[path = "../common/vertex.rs"]
mod vertex;

#[path = "../common/light.rs"]
mod light;

use wgpu_book::math_func;

use winit::event_loop::EventLoop;

use crate::app::App;
use crate::light::{Light, light};
use crate::vertex::{Vertex, create_vertices};

fn main() {
    let mut colormap_name = "jet";
    let mut is_two_side: i32 = 1;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        colormap_name = &args[1];
    }
    if args.len() > 2 {
        is_two_side = args[2].parse().unwrap();
    }

    let title = "ch09 sinc: ";
    let vertex_data = create_vertices(
        &math_func::sinc,
        colormap_name,
        -8.0,
        8.0,
        -8.0,
        8.0,
        30,
        30,
        2.0,
        0.3,
    );
    let light_data = light([1.0, 1.0, 1.0], 0.1, 0.8, 0.4, 30.0, is_two_side);

    let _ = run(&vertex_data, light_data, colormap_name, title);

    pub fn run(
        vertex_data: &Vec<Vertex>,
        light_data: Light,
        colormap_name: &str,
        title: &str,
    ) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(vertex_data, light_data, colormap_name, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
