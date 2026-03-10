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

fn create_vertices(r: f32, u: usize, v: usize) -> Vec<Vertex> {
    let mut pts: Vec<Vertex> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    for i in 0..u - 1 {
        for j in 0..v - 1 {
            let theta = i as f32 * 180.0 / (u as f32 - 1.0);
            let phi = j as f32 * 360.0 / (v as f32 - 1.0);
            let theta1 = (i as f32 + 1.0) * 180.0 / (u as f32 - 1.0);
            let phi1 = (j as f32 + 1.0) * 360.0 / (v as f32 - 1.0);
            let p0 = math_func::sphere_position(r, theta.to_radians(), phi.to_radians());
            let p1 = math_func::sphere_position(r, theta1.to_radians(), phi.to_radians());
            let p3 = math_func::sphere_position(r, theta.to_radians(), phi1.to_radians());
            pts.push(vertex(p0));
            pts.push(vertex(p1));
            pts.push(vertex(p0));
            pts.push(vertex(p3));
        }
    }

    pts.to_vec()
}

fn main() {
    let title = "ch07 sphere";
    let mesh_data = create_vertices(1.7, 15, 20);

    let _ = run(&mesh_data, title);

    pub fn run(mesh_data: &Vec<Vertex>, title: &str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
    
        let mut app = App::new(&mesh_data, title, None);
        
        event_loop.run_app(&mut app)?;

        Ok(())
    }
}