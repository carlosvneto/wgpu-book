use wgpu_book::vertex_data;

// Ensuring memory alignment
#[repr(C)]
// Vertex needs to be copied to create a buffer
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2 {
    position: [f32; 4],
    color: [f32; 4],
}

pub fn vertex2(p: [i8; 3], c: [i8; 3]) -> Vertex2 {
    Vertex2 {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
    }
}

pub fn create_vertices2() -> Vec<Vertex2> {
    let (pos, col, _uv, _normal) = vertex_data::cube_data();
    let mut data: Vec<Vertex2> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex2(pos[i], col[i]));
    }
    data.to_vec()
}

impl Vertex2 {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex2>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
