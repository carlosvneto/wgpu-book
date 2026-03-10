struct Uniforms {
    model_mat : mat4x4f,
    view_project_mat : mat4x4f,
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position : vec4f,
    @location(0) v_color : vec4f,
};

@vertex
fn vs_main(@location(0) pos: vec4f, @location(1) color: vec4f) -> Output {
    var output:Output;
    output.position = uniforms.view_project_mat * uniforms.model_mat * pos;
    output.v_color = color;
    return output;              
}

@fragment
fn fs_main(in:Output) -> @location(0) vec4f {
    return in.v_color;
}
