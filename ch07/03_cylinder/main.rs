use winit::event_loop::EventLoop;

use wgpu_book::math_func;

#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

#[path = "../common/vertex.rs"]
mod vertex;

use crate::app::App;
use crate::vertex::{Vertex, vertex};

fn create_vertices(rin: f32, rout: f32, height: f32, n: usize) -> Vec<Vertex> {
    let h = height / 2.0;
    let mut pts: Vec<Vertex> = Vec::with_capacity(16 * (n - 1));

    for i in 0..n - 1 {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta1 = (i as f32 + 1.0) * 360.0 / (n as f32 - 1.0);
        let p0 = math_func::cylinder_position(rout, h, theta.to_radians());
        let p1 = math_func::cylinder_position(rout, -h, theta.to_radians());
        let p2 = math_func::cylinder_position(rin, -h, theta.to_radians());
        let p3 = math_func::cylinder_position(rin, h, theta.to_radians());
        let p4 = math_func::cylinder_position(rout, h, theta1.to_radians());
        let p5 = math_func::cylinder_position(rout, -h, theta1.to_radians());
        let p6 = math_func::cylinder_position(rin, -h, theta1.to_radians());
        let p7 = math_func::cylinder_position(rin, h, theta1.to_radians());

        // top face 3 lines
        pts.push(vertex(p0));
        pts.push(vertex(p3));
        pts.push(vertex(p3));
        pts.push(vertex(p7));
        pts.push(vertex(p4));
        pts.push(vertex(p0));

        // bottom face 3 lines
        pts.push(vertex(p1));
        pts.push(vertex(p2));
        pts.push(vertex(p2));
        pts.push(vertex(p6));
        pts.push(vertex(p5));
        pts.push(vertex(p1));

        // side 2 lines
        pts.push(vertex(p0));
        pts.push(vertex(p1));
        pts.push(vertex(p3));
        pts.push(vertex(p2));
    }

    pts.to_vec()
}

fn main() {
    let title = "ch07 cylinder";
    let mesh_data = create_vertices(0.4, 1.0, 2.5, 20);

    let _ = run(&mesh_data, title);

    pub fn run(mesh_data: &Vec<Vertex>, title: &str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = App::new(&mesh_data, title, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
