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
use crate::light::{Light, light};
use crate::vertex::Vertex;

fn vertex(p: [i8; 3], n: [i8; 3], t: [i8; 2]) -> Vertex {
    Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        uv: [t[1] as f32, t[0] as f32],
    }
}

fn create_vertices() -> Vec<Vertex> {
    let (pos, _col, uv, normal) = vertex_data::cube_data();
    let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i], uv[i]));
    }
    data.to_vec()
}

fn main() {
    let mut file_name = "brick.png";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        file_name = &args[1];
    }

    let title = "ch10 cube: ";
    let vertex_data = create_vertices();
    let light_data = light([1.0, 1.0, 0.0], 0.1, 0.8, 0.4, 30.0, 1);
    let u_mode = wgpu::AddressMode::ClampToEdge;
    let v_mode = wgpu::AddressMode::ClampToEdge;

    let _ = run(&vertex_data, light_data, file_name, u_mode, v_mode, title);

    pub fn run(
        vertex_data: &Vec<Vertex>,
        light_data: Light,
        file_name: &str,
        u_mode: wgpu::AddressMode,
        v_mode: wgpu::AddressMode,
        title: &str,
    ) -> anyhow::Result<()> {
        let path = "ch10/assets/";
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
