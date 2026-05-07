use rand::distr::{Distribution, Uniform};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};

use wgpu_book::wgpu_simplified as ws;

pub struct State {
    init: ws::InitWgpu,

    // compute
    position_buffers: Vec<wgpu::Buffer>,
    color_buffer: wgpu::Buffer,
    compute_bind_groups: Vec<wgpu::BindGroup>,
    compute_pipeline: wgpu::ComputePipeline,
    num_particles: u32,

    // render
    vertex_buffer: wgpu::Buffer,
    render_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    frame_num: usize,
}

impl State {
    pub async fn new(
        window: Arc<Window>,
        num_particles: u32,
        particle_size: f32,
        color_opacity: f32,
        mass_factor: Vec<f32>,
    ) -> Self {
        let init = ws::InitWgpu::init_wgpu(window, 1).await;

        // Loading Shader
        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("attractors.wgsl").into()),
            });

        // compute **********************************************************************************************

        let mut position_data = vec![0.0f32; num_particles as usize * 4];
        //println!("position data size: {}", std::mem::size_of_val(&position_data)); // 24

        let mut velocity_data = vec![0.0f32; num_particles as usize * 4];
        let mut color_data = vec![0.0f32; num_particles as usize * 4];
        //let mut rng = rand::thread_rng();
        let mut rng = rand::rng();
        let unif_mp = Uniform::new(-1.0, 1.0).unwrap();
        let unif_p = Uniform::new(0.0, 1.0).unwrap();

        for position_chunck in position_data.chunks_mut(4) {
            position_chunck[0] = unif_mp.sample(&mut rng);
            position_chunck[1] = unif_mp.sample(&mut rng);
            position_chunck[2] = 0.0;
            position_chunck[3] = 1.0;
        }
        //println!("position data[0] size: {}", std::mem::size_of_val(&position_data[0])); // 24
        //println!("position data len: {}", position_data.len()); //

        for velocity_chunck in velocity_data.chunks_mut(4) {
            velocity_chunck[0] = unif_mp.sample(&mut rng) * 0.001;
            velocity_chunck[1] = unif_mp.sample(&mut rng) * 0.001;
            velocity_chunck[2] = 0.0;
            velocity_chunck[3] = 1.0;
        }
        for color_chunck in color_data.chunks_mut(4) {
            color_chunck[0] = unif_p.sample(&mut rng);
            color_chunck[1] = unif_p.sample(&mut rng);
            color_chunck[2] = unif_p.sample(&mut rng);
            color_chunck[3] = color_opacity;
        }

        let mut position_buffers = Vec::<wgpu::Buffer>::new();
        let mut velocity_buffers = Vec::<wgpu::Buffer>::new();

        for i in 0..2 {
            position_buffers.push(init.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Position Buffer {}", i)),
                    contents: bytemuck::cast_slice(&position_data),
                    usage: wgpu::BufferUsages::VERTEX
                        | wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST,
                },
            ));
            velocity_buffers.push(init.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Velocity Buffer {}", i)),
                    contents: bytemuck::cast_slice(&velocity_data),
                    usage: wgpu::BufferUsages::VERTEX
                        | wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST,
                },
            ));
        }

        let color_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Color Buffer")),
                contents: bytemuck::cast_slice(&color_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        let mass_uniform_data = [
            unif_mp.sample(&mut rng),
            unif_mp.sample(&mut rng),
            0.0,
            1.0, // mass 1 position
            unif_mp.sample(&mut rng),
            unif_mp.sample(&mut rng),
            0.0,
            1.0, // mass 2 position
            unif_mp.sample(&mut rng),
            unif_mp.sample(&mut rng),
            0.0,
            1.0,                                                             // mass 3 position
            unif_p.sample(&mut rng) * mass_factor[0] / num_particles as f32, // mass 1 factor
            unif_p.sample(&mut rng) * mass_factor[1] / num_particles as f32, // mass 2 factor
            unif_p.sample(&mut rng) * mass_factor[2] / num_particles as f32, // mass 3 factor
            0.0,                                                             // padding
        ]
        .to_vec();

        let mass_uniform_buffer =
            init.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Mass Attractor Uniform Buffer")),
                    contents: bytemuck::cast_slice(&mass_uniform_data),
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
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
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
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: None,
                });

        let mut compute_bind_groups = Vec::<wgpu::BindGroup>::new();

        for i in 0..2 {
            compute_bind_groups.push(init.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &compute_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: position_buffers[i].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: velocity_buffers[i % 2].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: position_buffers[(i + 1) % 2].as_entire_binding(), // bind to opposite buffer
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: velocity_buffers[(i + 1) % 2].as_entire_binding(), // bind to opposite buffer
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: mass_uniform_buffer.as_entire_binding(),
                    },
                ],
                label: None,
            }));
        }

        let compute_pipeline_layout =
            init.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("compute"),
                    bind_group_layouts: &[Some(&compute_bind_group_layout)],
                    immediate_size: 0,
                });

        let compute_pipeline =
            init.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("Compute Pipeline"),
                    layout: Some(&compute_pipeline_layout),
                    module: &shader,
                    entry_point: Some("cs_main"),
                    cache: None,
                    compilation_options: Default::default(),
                });

        // render **********************************************************************************************

        let uniform_data = vec![
            init.config.width as f32,
            init.config.height as f32,
            particle_size as f32,
            0.0 as f32,
        ];

        let uniform_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Uniform Buffer")),
                contents: bytemuck::cast_slice(&uniform_data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let vertex_data = vec![-1.0f32, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0].to_vec();

        let vertex_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Vertex Buffer")),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let render_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
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
                resource: uniform_buffer.as_entire_binding(),
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
                            array_stride: 8,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: 16,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &wgpu::vertex_attr_array![1 => Float32x4],
                            //attributes: &wgpu::vertex_attr_array![1 => Unorm8x4],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: 16,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &wgpu::vertex_attr_array![2 => Float32x4],
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
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: Some(wgpu::IndexFormat::Uint32),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        Self {
            init,

            // compute
            position_buffers,
            color_buffer,
            compute_bind_groups,
            compute_pipeline,
            num_particles,

            // render
            vertex_buffer,
            render_pipeline,
            render_bind_group,
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

    pub fn render(&mut self) -> anyhow::Result<()> {
        let output = match self.init.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.init.surface.configure(&self.init.device, &self.init.config);
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Skip this frame
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.init.surface.configure(&self.init.device, &self.init.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        {
            // compute pass
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_bind_groups[self.frame_num % 2], &[]);
            compute_pass.dispatch_workgroups(self.num_particles, 1, 1);
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
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.color_buffer.slice(..));
            render_pass
                .set_vertex_buffer(2, self.position_buffers[(self.frame_num + 1) % 2].slice(..));

            render_pass.draw(0..4, 0..self.num_particles);
        }

        self.frame_num += 1;

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
