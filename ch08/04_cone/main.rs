#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

#[path = "../common/vertex.rs"]
mod vertex;

#[path = "../common/light.rs"]
mod light;

use wgpu_book::vertex_data;

use winit::event_loop::EventLoop;

use crate::app::App;
use crate::light::Light;
use crate::vertex::{vertex, Vertex};

fn create_vertices(rtop: f32, rbottom: f32, height: f32, n: usize) -> Vec<Vertex> {
    let (pos, normal, _uvs) = vertex_data::cone_data(rtop, rbottom, height, n);
    let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i]));
    }
    data.to_vec()
}

fn main() {
    let title = "ch08 cone";
    let vertex_data = create_vertices(0.5, 1.5, 2.0, 30);
    let light_data = light::light([1.0, 0.0, 0.0], [1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0);

    let _ = run(&vertex_data, light_data, title);

    pub fn run(
        vertex_data: &Vec<Vertex>,
        light_data: Light,
        title: &str,
    ) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(vertex_data, light_data, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
