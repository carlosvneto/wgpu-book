// vertex shader

struct Uniforms {
    mvpMatrix : array<mat4x4f, 35>,
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) Position : vec4f,
    @location(0) vColor : vec4f,
};

@vertex
fn vs_main(@builtin(instance_index) instanceIdx : u32, @location(0) pos: vec4f, @location(1) color: vec4f) -> Output {
    var output: Output;
    output.Position = uniforms.mvpMatrix[instanceIdx] * pos;
    output.vColor = color;
    return output;
}

// fragment shader
@fragment
fn fs_main(@location(0) vColor: vec4f) -> @location(0) vec4f {
    return vColor;
}
