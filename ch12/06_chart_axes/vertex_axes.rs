use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex3 {
    position: [f32; 4],
    color: [f32; 4],
}

pub fn vertex3(p: [f32; 3], c: [f32; 3]) -> Vertex3 {
    Vertex3 {
        position: [p[0], p[1], p[2], 1.0],
        color: [c[0], c[1], c[2], 1.0],
    }
}

pub fn create_vertices3(center: [f32; 3], size: [f32; 3]) -> Vec<Vertex3> {
    let mut data: Vec<Vertex3> = Vec::with_capacity(6);
    data.push(vertex3([center[0], center[1], center[2]], [1.0, 0.0, 0.0]));
    data.push(vertex3(
        [center[0] + size[0], center[1], center[2]],
        [1.0, 0.0, 0.0],
    ));
    data.push(vertex3([center[0], center[1], center[2]], [0.0, 1.0, 0.0]));
    data.push(vertex3(
        [center[0], center[1] + size[1], center[2]],
        [0.0, 1.0, 0.0],
    ));
    data.push(vertex3([center[0], center[1], center[2]], [0.0, 0.0, 1.0]));
    data.push(vertex3(
        [center[0], center[1], center[2] + size[2]],
        [0.0, 0.0, 1.0],
    ));
    data.to_vec()
}

impl Vertex3 {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex3>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
