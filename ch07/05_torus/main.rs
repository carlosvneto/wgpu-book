use winit::event_loop::EventLoop;

use wgpu_book::math_func;

#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

#[path = "../common/vertex.rs"]
mod vertex;

use crate::app::App;
use crate::vertex::{vertex, Vertex};

fn create_vertices(r_torus: f32, r_tube: f32, n_torus: usize, n_tube: usize) -> Vec<Vertex> {
    let mut pts: Vec<Vertex> = Vec::with_capacity((4 * (n_torus - 1) * (n_tube - 1)) as usize);
    for i in 0..n_torus - 1 {
        for j in 0..n_tube - 1 {
            let u = i as f32 * 360.0 / (n_torus as f32 - 1.0);
            let v = j as f32 * 360.0 / (n_tube as f32 - 1.0);
            let u1 = (i as f32 + 1.0) * 360.0 / (n_torus as f32 - 1.0);
            let v1 = (j as f32 + 1.0) * 360.0 / (n_tube as f32 - 1.0);
            let p0 = math_func::torus_position(r_torus, r_tube, u.to_radians(), v.to_radians(),);
            let p1 = math_func::torus_position(r_torus, r_tube, u1.to_radians(), v.to_radians(),);
            let p3 = math_func::torus_position(r_torus, r_tube, u.to_radians(), v1.to_radians(),);

            pts.push(vertex(p0));
            pts.push(vertex(p1));
            pts.push(vertex(p0));
            pts.push(vertex(p3));
        }
    }

    pts.to_vec()
}

fn main() {
    let title = "ch07 torus";
    let mesh_data = create_vertices(1.5, 0.3, 40, 13);

    let _ = run(&mesh_data, title);

    pub fn run(mesh_data: &Vec<Vertex>, title: &str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(&mesh_data, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
