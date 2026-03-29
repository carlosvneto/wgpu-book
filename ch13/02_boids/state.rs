use rand::distr::{Distribution, Uniform};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{
    event_loop::{ActiveEventLoop, OwnedDisplayHandle},
    keyboard::KeyCode,
    window::Window,
};

use wgpu_book::wgpu_simplified as ws;

const NUM_PARTICLES: u32 = 5000;
const PARTICLES_PER_GROUP: u32 = 64;

pub struct State {
    init: ws::InitWgpu,
    particle_bind_groups: Vec<wgpu::BindGroup>,
    particle_buffers: Vec<wgpu::Buffer>,
    vertices_buffer: wgpu::Buffer,
    compute_pipeline: wgpu::ComputePipeline,
    render_pipeline: wgpu::RenderPipeline,
    render_bind_group: wgpu::BindGroup,
    work_group_count: u32,
    frame_num: usize,
}

impl State {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>, color_scale: f32) -> Self {
        let init = ws::InitWgpu::init_wgpu(display, window, 1).await;

        // Loading Shader
        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("boids.wgsl").into()),
            });

        let param_data = [
            0.04f32, // deltaT
            0.1,     // rule1Distance
            0.025,   // rule2Distance
            0.025,   // rule3Distance
            0.02,    // rule1Scale
            0.05,    // rule2Scale
            0.005,   // rule3Scale
        ]
        .to_vec();

        let color_scale_buffer =
            init.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Color Scale Buffer"),
                    contents: bytemuck::cast_slice(&[color_scale]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let param_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Parameter Buffer"),
                contents: bytemuck::cast_slice(&param_data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let compute_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: None,
                });

        let compute_pipeline_layout =
            init.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("compute"),
                    bind_group_layouts: &[Some(&compute_bind_group_layout)],
                    immediate_size: 0,
                });

        let render_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });

        let render_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color_scale_buffer.as_entire_binding(),
            }],
            label: Some("Render Bind Group"),
        });

        let render_pipeline_layout =
            init.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("render"),
                    bind_group_layouts: &[Some(&render_bind_group_layout)],
                    immediate_size: 0,
                });

        let render_pipeline = init
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: 4 * 4,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                            // particle -> 0 = position  , 1 = velocity
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: 2 * 4,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![2 => Float32x2],
                            // vertex -> 2 = position
                        },
                    ],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(init.config.format.into())],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        let compute_pipeline =
            init.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("Compute pipeline"),
                    layout: Some(&compute_pipeline_layout),
                    module: &shader,
                    entry_point: Some("cs_main"),
                    cache: None,
                    compilation_options: Default::default(),
                });

        let vertex_data = [-0.01f32, -0.02, 0.01, -0.02, 0.00, 0.02];
        let vertices_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::bytes_of(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        let mut initial_particle_data = vec![0.0f32; (4 * NUM_PARTICLES) as usize];

        let mut rng = rand::rng();
        let unif = Uniform::new(-1.0, 1.0).unwrap();

        for particle_instance_chunk in initial_particle_data.chunks_mut(4) {
            particle_instance_chunk[0] = unif.sample(&mut rng); // posx
            particle_instance_chunk[1] = unif.sample(&mut rng); // posy
            particle_instance_chunk[2] = unif.sample(&mut rng) * 0.1; // velx
            particle_instance_chunk[3] = unif.sample(&mut rng) * 0.1; // vely
        }

        let mut particle_buffers = Vec::<wgpu::Buffer>::new();
        let mut particle_bind_groups = Vec::<wgpu::BindGroup>::new();
        for i in 0..2 {
            particle_buffers.push(init.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Particle Buffer {}", i)),
                    contents: bytemuck::cast_slice(&initial_particle_data),
                    usage: wgpu::BufferUsages::VERTEX
                        | wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST,
                },
            ));
        }

        for i in 0..2 {
            particle_bind_groups.push(init.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &compute_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: param_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: particle_buffers[i % 2].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: particle_buffers[(i + 1) % 2].as_entire_binding(), // bind to opposite buffer
                    },
                ],
                label: None,
            }));
        }

        let work_group_count =
            ((NUM_PARTICLES as f32) / (PARTICLES_PER_GROUP as f32)).ceil() as u32;

        Self {
            init,
            particle_bind_groups,
            particle_buffers,
            vertices_buffer,
            compute_pipeline,
            render_pipeline,
            render_bind_group,
            work_group_count,
            frame_num: 0,
        }
    }

    pub fn window(&self) -> &Window {
        &self.init.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            // The surface needs to be reconfigured every time the window is resized.
            self.init.config.width = width;
            self.init.config.height = height;
            self.init
                .surface
                .configure(&self.init.device, &self.init.config);
        }
    }

    pub fn handle_key_input(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        match (key, pressed) {
            (KeyCode::Escape, true) => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    pub fn update(&mut self, _dt: std::time::Duration) {
        // empty
    }

    pub fn render(&mut self) -> Option<()> {
        let output = match self.init.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => {
                return None;
            }
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                return None;
            }
            other => {
                eprintln!("Failed to get surface texture: {other:?}");
                return None;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            // compute pass
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.particle_bind_groups[self.frame_num % 2], &[]);
            compute_pass.dispatch_workgroups(self.work_group_count, 1, 1);
        }
        {
            // render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store, // true,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass
                .set_vertex_buffer(0, self.particle_buffers[(self.frame_num + 1) % 2].slice(..));
            render_pass.set_vertex_buffer(1, self.vertices_buffer.slice(..));
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.draw(0..3, 0..NUM_PARTICLES);
        }

        self.frame_num += 1;

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Some(())
    }
}
