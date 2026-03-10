// vertex shader

struct Transform {
    viewProjectionMatrix: mat4x4f,
};
@binding(0) @group(0) var<uniform> transform: Transform;

struct Input {
    @location(0) position: vec2f,
    @location(1) instancePosition: vec2f,
    @location(2) instanceVelocity: vec2f,
    @location(3) scaleFactor: f32,
    @location(4) color: vec3f,
};

struct Output {
    @builtin(position) Position : vec4f,
    @location(0) color : vec4f,
};

@vertex
fn vs_main (input: Input) -> Output {
    var output: Output;
    let scaleMatrix = mat4x4<f32>(
        vec4f(input.scaleFactor,  0.0,                0.0,               0.0),
        vec4f(0.0,                input.scaleFactor,  0.0,               0.0),
        vec4f(0.0,                0.0,                input.scaleFactor, 0.0),
        vec4f(0.0,                0.0,                0.0,               1.0)
    );    
    let pos = vec4f(input.position, 0.0, 1.0);
    let ins_pos = vec4f(input.instancePosition, 0.0, 1.0);
    let transformedPos = scaleMatrix *pos + ins_pos;
    output.Position = transform.viewProjectionMatrix * transformedPos;    
    output.color = vec4f(input.color, 1.0);
    return output;
}

// fragment shader
      
@fragment
fn fs_main (in: Output) -> @location(0) vec4f {
    return in.color;
}

// compute shader

struct ParticleData {
    position: vec2f,   // 8 bytes aligned
    velocity: vec2f,   // 8 bytes aligned
    radius: f32,           // 4 bytes aligned
    pad0: f32,             // 4 bytes padding
    pad1: f32,             // 4 bytes padding
    pad2: f32,             // 4 bytes padding
};                         // 32 bytes aligned

struct ParticlesBuffer {
    particles: array<ParticleData>,
};      
@binding(0) @group(0) var<storage, read_write> particlesBuffer: ParticlesBuffer;

struct Uniforms {
    canvasSize: vec2f,
    deltaTime: f32,
    bounceFactor: f32,
    acceLeft: vec4f,
    acceRight: vec4f,
};
@binding(1) @group(0) var<uniform> uniforms : Uniforms;

@compute @workgroup_size(64, 1, 1)
fn cs_main(@builtin(global_invocation_id) GlobalInvocationID : vec3<u32>) {
    let index = GlobalInvocationID.x;
    let canvasWidth = uniforms.canvasSize.x * 2.0;
    let canvasHeight = uniforms.canvasSize.y * 2.0;
    
    let particleRadius = particlesBuffer.particles[index].radius;
    let vx = particlesBuffer.particles[index].velocity.x;
    let vy = particlesBuffer.particles[index].velocity.y;
    if (particlesBuffer.particles[index].position.x < canvasWidth * 0.5) {
        if (particlesBuffer.particles[index].position.y > canvasHeight * 0.5) {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceLeft.x * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceLeft.y * uniforms.deltaTime;
        } else {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceLeft.z * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceLeft.w * uniforms.deltaTime;
        }
    } else {
        if (particlesBuffer.particles[index].position.y > canvasHeight * 0.5) {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceRight.x * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceRight.y * uniforms.deltaTime;
        } else {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceRight.z * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceRight.w * uniforms.deltaTime;
        }
    }
    particlesBuffer.particles[index].position.x = particlesBuffer.particles[index].position.x 
        + particlesBuffer.particles[index].velocity.x * uniforms.deltaTime;
    particlesBuffer.particles[index].position.y = particlesBuffer.particles[index].position.y 
        + particlesBuffer.particles[index].velocity.y * uniforms.deltaTime;
    
    // handle screen viewport
    if (particlesBuffer.particles[index].position.x + particleRadius * 0.5 > canvasWidth) {
        particlesBuffer.particles[index].position.x = canvasWidth - particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.x = vx * -uniforms.bounceFactor;
    } else if (particlesBuffer.particles[index].position.x - particleRadius * 0.5 < 0.0) {
        particlesBuffer.particles[index].position.x = particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.x = vx * -uniforms.bounceFactor;
    }
    if (particlesBuffer.particles[index].position.y + particleRadius * 0.5 > canvasHeight) {
        particlesBuffer.particles[index].position.y = canvasHeight - particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.y = vy * -uniforms.bounceFactor;
    } else if (particlesBuffer.particles[index].position.y - particleRadius * 0.5 < 0.0) {
        particlesBuffer.particles[index].position.y = particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.y = vy * -uniforms.bounceFactor;
    }
}
