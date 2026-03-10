use bytemuck::{Pod, Zeroable};

// Ensuring memory alignment
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Light {
    specular_color: [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
    is_two_side: i32,
}

pub fn light(
    sc: [f32; 3],
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    two_side: i32,
) -> Light {
    Light {
        specular_color: [sc[0], sc[1], sc[2], 1.0],
        ambient_intensity: ambient,
        diffuse_intensity: diffuse,
        specular_intensity: specular,
        specular_shininess: shininess,
        is_two_side: two_side,
    }
}
