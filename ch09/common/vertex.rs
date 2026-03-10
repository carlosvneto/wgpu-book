use wgpu_book::surface_data::{parametric_surface_data, simple_surface_data};

// Ensuring memory alignment
#[repr(C)]
// Vertex needs to be copied to create a buffer
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
}

pub fn vertex(p: [f32; 3], n: [f32; 3], c: [f32; 3]) -> Vertex {
    Vertex {
        position: [p[0], p[1], p[2], 1.0],
        normal: [n[0], n[1], n[2], 1.0],
        color: [c[0], c[1], c[2], 1.0],
    }
}

#[allow(dead_code)]
pub fn create_vertices(
    f: &dyn Fn(f32, f32) -> [f32; 3],
    colormap_name: &str,
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    nx: usize,
    nz: usize,
    scale: f32,
    scaley: f32,
) -> Vec<Vertex> {
    let (pos, normal, color, _uv, _uv1) = simple_surface_data(
        f,
        colormap_name,
        xmin,
        xmax,
        zmin,
        zmax,
        nx,
        nz,
        scale,
        scaley,
    );
    let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i], color[i]));
    }
    data.to_vec()
}

#[allow(dead_code)]
pub fn create_vertices_param(
    f: &dyn Fn(f32, f32) -> [f32; 3],
    colormap_name: &str,
    umin: f32,
    umax: f32,
    vmin: f32,
    vmax: f32,
    nu: usize,
    nv: usize,
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    scale: f32,
    scaley: f32,
) -> Vec<Vertex> {
    let (pos, normal, color, _uv, _uv1) = parametric_surface_data(
        f,
        colormap_name,
        umin,
        umax,
        vmin,
        vmax,
        nu,
        nv,
        xmin,
        xmax,
        zmin,
        zmax,
        scale,
        scaley,
    );
    let mut data: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], normal[i], color[i]));
    }
    data.to_vec()
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
