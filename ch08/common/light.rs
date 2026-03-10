use bytemuck::{Pod, Zeroable};

// Ensuring memory alignment
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Light {
    color: [f32; 4],
    specular_color: [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

pub fn light(c: [f32; 3], sc: [f32; 3], ai: f32, di: f32, si: f32, ss: f32) -> Light {
    Light {
        color: [c[0], c[1], c[2], 1.0],
        specular_color: [sc[0], sc[1], sc[2], 1.0],
        ambient_intensity: ai,
        diffuse_intensity: di,
        specular_intensity: si,
        specular_shininess: ss,
    }
}
