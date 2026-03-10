#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

#[path = "../common/vertex.rs"]
mod vertex;

#[path = "../common/light.rs"]
mod light;

use wgpu_book::math_func;
use wgpu_book::surface_data;

use std::f32::consts::PI;
use winit::event_loop::EventLoop;

use crate::app::App;
use crate::light::{light, Light};
use crate::vertex::{vertex, Vertex};

fn create_vertices(colormap_name: &str) -> Vec<Vertex> {
    let (pos, normal, color, _uv, uv1) = surface_data::parametric_surface_data(
        &math_func::klein_bottle,
        colormap_name,
        0.0,
        PI,
        0.0,
        2.0 * PI,
        70,
        30,
        -2.0,
        2.0,
        -2.0,
        3.0,
        2.0,
        0.0,
    );
    let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i], uv1[i], color[i]));
    }
    data.to_vec()
}

fn main() {
    let mut file_name = "whitesquare2.png";
    let mut colormap_name = "jet";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        file_name = &args[1];
    }
    if args.len() > 2 {
        colormap_name = &args[2];
    }

    let title = "ch11 klein bottle: ".to_owned()
        + "image file: "
        + file_name
        + " colormap: "
        + colormap_name;
    let vertex_data = create_vertices(colormap_name);
    let light_data = light([1.0, 1.0, 0.0], 0.1, 0.8, 0.4, 30.0, 1);
    let u_mode = wgpu::AddressMode::ClampToEdge;
    let v_mode = wgpu::AddressMode::ClampToEdge;

    let _ = run(&vertex_data, light_data, file_name, u_mode, v_mode, &title);

    pub fn run(
        vertex_data: &Vec<Vertex>,
        light_data: Light,
        file_name: &str,
        u_mode: wgpu::AddressMode,
        v_mode: wgpu::AddressMode,
        title: &str,
    ) -> anyhow::Result<()> {
        let path = "ch11/assets/";
        let img_file = [path, file_name].join("");

        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(
            vertex_data,
            light_data,
            &img_file,
            u_mode,
            v_mode,
            title,
            None,
        );

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
