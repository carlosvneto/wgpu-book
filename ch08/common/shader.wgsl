// vertex shader

struct Uniforms {
    model_mat : mat4x4f,
    view_project_mat : mat4x4f,
    normal_mat : mat4x4f,
};

@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position : vec4f,
    @location(0) v_position : vec4f,
    @location(1) v_normal : vec4f,
};

@vertex
fn vs_main(@location(0) pos: vec4f, @location(1) normal: vec4f) -> Output {
    var output: Output;
    let m_position:vec4f = uniforms.model_mat * pos; 
    output.v_position = m_position;
    output.v_normal =  uniforms.normal_mat * normal;
    output.position = uniforms.view_project_mat * m_position;               
    return output;
}


// fragment shader

struct FragUniforms {
    light_position : vec4f,
    eye_position : vec4f,
};
@binding(1) @group(0) var<uniform> frag_uniforms : FragUniforms;

struct LightUniforms {
    color : vec4f,
    specular_color : vec4f,
    ambient_intensity: f32,
    diffuse_intensity :f32,
    specular_intensity: f32,
    specular_shininess: f32,
};
@binding(2) @group(0) var<uniform> light_uniforms : LightUniforms;

@fragment
fn fs_main(@location(0) v_position: vec4f, @location(1) v_normal: vec4f) ->  @location(0) vec4f {
    let N:vec3<f32> = normalize(v_normal.xyz);
    let L:vec3<f32> = normalize(frag_uniforms.light_position.xyz - v_position.xyz);
    let V:vec3<f32> = normalize(frag_uniforms.eye_position.xyz - v_position.xyz);
    let H:vec3<f32> = normalize(L + V);
    let diffuse:f32 = light_uniforms.diffuse_intensity * max(dot(N, L), 0.0);
    let specular: f32 = light_uniforms.specular_intensity * pow(max(dot(N, H),0.0), light_uniforms.specular_shininess);
    let ambient:f32 = light_uniforms.ambient_intensity;
    return light_uniforms.color*(ambient + diffuse) + light_uniforms.specular_color * specular;
}
