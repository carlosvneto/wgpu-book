#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

#[path = "../common/vertex.rs"]
mod vertex;

#[path = "../common/light.rs"]
mod light;

use wgpu_book::math_func;

use std::f32::consts::PI;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::light::{light, Light};
use crate::vertex::{create_vertices_param, Vertex};

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

    let title = "ch09 sievert enneper: ";
    let vertex_data = create_vertices_param(
        &math_func::sievert_enneper,
        colormap_name,
        -PI / 2.001,
        PI / 2.001,
        0.000001,
        PI,
        60,
        200,
        -2.0,
        2.0,
        -2.0,
        2.0,
        2.5,
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
