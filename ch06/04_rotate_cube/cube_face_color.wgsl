struct Uniforms {
    mvpMatrix : mat4x4f,
};

@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) Position : vec4f,
    @location(0) vColor : vec4f,
};

@vertex
fn vs_main(@location(0) pos: vec4f, @location(1) color: vec4f) -> Output {
    var output: Output;
    output.Position = uniforms.mvpMatrix * pos;
    output.vColor = color;
    return output;
}

@fragment
fn fs_main(@location(0) vColor: vec4f) -> @location(0) vec4f {
    return vColor;
}
