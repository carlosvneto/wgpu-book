#[path = "../../ch09/common/app.rs"]
mod app;
#[path = "../../ch09/common/light.rs"]
mod light;
#[path = "../../ch09/common/state.rs"]
mod state;
#[path = "../../ch09/common/vertex.rs"]
mod vertex;

use wgpu_book::math_func;

use std::f32::consts::PI;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::light::{light, Light};
use crate::vertex::{create_vertices, create_vertices_param, Vertex};

fn add_center(mut data: Vec<Vertex>, center: [f32; 3]) -> Vec<Vertex> {
    for i in 0..data.len() {
        let p0 = data[i].position[0] + center[0];
        let p1 = data[i].position[1] + center[1];
        let p2 = data[i].position[2] + center[2];
        data[i].position = [p0, p1, p2, 1.0];
    }
    data
}

fn get_vertex_data() -> Vec<Vertex> {
    // create sinc surface
    let mut vertex_data = create_vertices(
        &math_func::sinc,
        "jet",
        -8.0,
        8.0,
        -8.0,
        8.0,
        30,
        30,
        1.2,
        0.3,
    );
    vertex_data = add_center(vertex_data, [-3.0, -1.0, 0.0]);

    // create peaks surface
    let mut peaks_data = create_vertices(
        &math_func::peaks,
        "cool",
        -3.0,
        3.0,
        -3.0,
        3.0,
        31,
        31,
        1.0,
        0.3,
    );
    peaks_data = add_center(peaks_data, [3.0, -1.0, 0.0]);

    // create sphere
    let mut sphere_data = create_vertices_param(
        &math_func::sphere,
        "hsv",
        0.0,
        2.0 * PI,
        -PI / 2.0,
        PI / 2.0,
        20,
        15,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.2,
        0.0,
    );
    sphere_data = add_center(sphere_data, [0.0, 1.0, 0.0]);

    // create torus
    let mut torus_data = create_vertices_param(
        &math_func::torus,
        "hot",
        0.0,
        2.0 * PI,
        0.0,
        2.0 * PI,
        40,
        15,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.0,
        1.5,
    );
    torus_data = add_center(torus_data, [1.0, -2.0, 0.0]);

    // combine vertex data
    vertex_data.extend(peaks_data);
    vertex_data.extend(sphere_data);
    vertex_data.extend(torus_data);

    vertex_data
}

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

    let title = "ch12 different objects";
    let vertex_data = get_vertex_data();
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
