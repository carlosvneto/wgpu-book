// Ensuring memory alignment
#[repr(C)]
// Vertex needs to be copied to create a buffer
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        // vertex a, index = 0
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        // vertex b, index = 1
        position: [0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        // vertex c, index = 2
        position: [0.5, 0.5],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        // vertex d, index = 3
        position: [-0.5, 0.5],
        color: [1.0, 1.0, 0.0],
    },
];

pub const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];
