pub const IS_PERSPECTIVE: bool = true;

// Ensuring memory alignment
#[repr(C)]
// Vertex needs to be copied to create a buffer
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

pub fn create_vertices() -> [Vertex; 300] {
    let mut vertices = [Vertex {
        position: [0.0, 0.0, 0.0],
    }; 300];
    for i in 0..300 {
        let t = 0.1 * (i as f32) / 30.0;
        let x = (-t).exp() * (30.0 * t).sin();
        let z = (-t).exp() * (30.0 * t).cos();
        let y = 2.0 * t - 1.0;
        vertices[i] = Vertex {
            position: [x, y, z],
        };
    }
    vertices
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0=>Float32x3];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
