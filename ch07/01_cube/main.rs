use winit::event_loop::EventLoop;

#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

#[path = "../common/vertex.rs"]
mod vertex;

use crate::app::App;
use crate::vertex::{Vertex, vertex};

pub fn create_vertices() -> Vec<Vertex> {
    // vertex positions
    let p: [[f32; 3]; 8] = [
        [-1.0, 1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [1.0, 1.0, -1.0],
        [1.0, 1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, -1.0, 1.0],
    ];

    // line segments
    let lines: [[f32; 3]; 24] = [
        // 4 lines on top face
        p[0], p[1], p[1], p[2], p[2], p[3], p[3], p[0], // 4 lines on bottom face
        p[4], p[5], p[5], p[6], p[6], p[7], p[7], p[4], // 4 lines on side
        p[0], p[4], p[1], p[5], p[2], p[6], p[3], p[7],
    ];

    let mut data: Vec<Vertex> = Vec::with_capacity(lines.len());
    for i in 0..lines.len() {
        data.push(vertex(lines[i]));
    }
    data.to_vec()
}

fn main() {
    let title = "ch07 cube";
    let mesh_data = create_vertices();

    let _ = run(&mesh_data, title);

    pub fn run(mesh_data: &Vec<Vertex>, title: &str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(&mesh_data, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
