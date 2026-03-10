struct Uniforms {
    model_mat : mat4x4f,
    view_project_mat : mat4x4f,
    normal_mat : mat4x4f,
    color: vec4f,
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

@vertex
fn vs_main(@location(0) pos: vec4f) ->  @builtin(position) vec4f {
    return uniforms.view_project_mat * uniforms.model_mat * pos;                
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return uniforms.color;            
}
