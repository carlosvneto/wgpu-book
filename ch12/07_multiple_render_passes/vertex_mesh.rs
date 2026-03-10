use wgpu_book::math_func;
use wgpu_book::surface_data;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex2 {
    position: [f32; 4],
}

pub fn vertex2(p: [f32; 3]) -> Vertex2 {
    Vertex2 {
        position: [p[0], p[1], p[2], 1.0],
    }
}

pub fn create_vertices2() -> Vec<Vertex2> {
    let mesh =
        surface_data::simple_mesh_data(&math_func::sinc, -8.0, 8.0, -8.0, 8.0, 30, 30, 2.0, 0.3);
    let mut data: Vec<Vertex2> = Vec::with_capacity(mesh.len());
    for i in 0..mesh.len() {
        data.push(vertex2(mesh[i]));
    }
    data.to_vec()
}

impl Vertex2 {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0=>Float32x4];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex2>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
