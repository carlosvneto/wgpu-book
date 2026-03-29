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
use crate::vertex::Vertex;

fn vertex(p: [i8; 3], n: [i8; 3]) -> vertex::Vertex {
    vertex::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
    }
}

fn create_vertices() -> Vec<Vertex> {
    let (pos, _col, _uv, normal) = vertex_data::cube_data();
    let mut data: Vec<vertex::Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i]));
    }
    data.to_vec()
}

fn main() {
    let title = "ch08 cube";
    let vertex_data = create_vertices();
    let light_data = light::light([1.0, 0.0, 0.0], [1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0);

    let _ = run(&vertex_data, light_data, title);

    pub fn run(vertex_data: &Vec<Vertex>, light_data: Light, title: &str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(vertex_data, light_data, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
