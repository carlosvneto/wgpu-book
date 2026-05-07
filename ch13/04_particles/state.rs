use rand::distr::{Distribution, Uniform};
use std::sync::Arc;
use std::time::SystemTime;
use wgpu::util::DeviceExt;
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};

use wgpu_book::wgpu_simplified as ws;

use wgpu_book::transforms;

const PARTICLES_PER_GROUP: u32 = 64;

pub struct State {
    init: ws::InitWgpu,

    // compute
    particle_buffer: wgpu::Buffer,
    particle_uniform_data: Vec<f32>,
    particle_uniform_buffer: wgpu::Buffer,
    compute_bind_group: wgpu::BindGroup,
    compute_pipeline: wgpu::ComputePipeline,
    work_group_count: u32,
    num_particles: u32,

    // render
    vertex_buffer: wgpu::Buffer,
    render_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,

    // parameters
    unif_mp: Uniform<f32>,
    rng: rand::rngs::ThreadRng,
    start: SystemTime,
    t0: f32,
    t1: f32,
}

impl State {
    pub async fn new(
        window: Arc<Window>,
        num_particles: u32,
        particle_size: f32,
    ) -> Self {
        let start = SystemTime::now();
        let init = ws::InitWgpu::init_wgpu(window, 1).await;

        // Loading Shader
        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("particles.wgsl").into()),
            });

        // compute **********************************************************************************************

        let mut particle_data = vec![0.0f32; num_particles as usize * 10];

        //let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut rng = rand::rng();
        let unif_mp = Uniform::new(-1.0, 1.0).unwrap();
        let unif_p = Uniform::new(0.0, 1.0).unwrap();
        for particle_chunck in particle_data.chunks_mut(8) {
            // position
            particle_chunck[0] = unif_p.sample(&mut rng) * init.config.width as f32 * 2.0;
            particle_chunck[1] = unif_p.sample(&mut rng) * init.config.height as f32 * 2.0;
            // velocity
            particle_chunck[2] = unif_mp.sample(&mut rng) * 400.0;
            particle_chunck[3] = unif_mp.sample(&mut rng) * 400.0;
            // radius
            particle_chunck[4] = unif_p.sample(&mut rng) + 3.0;
            // color rgb
            particle_chunck[5] = unif_p.sample(&mut rng);
            particle_chunck[6] = unif_p.sample(&mut rng);
            particle_chunck[7] = unif_p.sample(&mut rng);
        }

        let particle_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Particle Buffer")),
                contents: bytemuck::cast_slice(&particle_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
            });

        let particle_uniform_data = [
            init.config.width as f32, // size
            init.config.height as f32,
            0.0,                              // delta_frame
            0.6,                              // bounce_factor
            unif_mp.sample(&mut rng) * 240.0, // acceleration left
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0, // acceleration right
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
        ]
        .to_vec();

        let particle_uniform_buffer =
            init.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Particle Uniform Buffer")),
                    contents: bytemuck::cast_slice(&particle_uniform_data),
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
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
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

        let compute_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: particle_uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("Compute Bind Group"),
        });

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

        let camera_position = (0.0, 0.0, 2.0).into();
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = (0.0, 1.0, 0.0).into();
        let right = init.config.width as f32;
        let top = init.config.height as f32;

        let (_, _, vp_mat) = transforms::create_view_projection_ortho(
            0.0,
            right,
            0.0,
            top,
            -2.0,
            3.0,
            camera_position,
            look_direction,
            up_direction,
        );

        let vp_ref: &[f32; 16] = vp_mat.as_ref();
        let uniform_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(vp_ref),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let vertex_data = vec![
            -particle_size / 2.0,
            -particle_size / 2.0,
            particle_size / 2.0,
            -particle_size / 2.0,
            -particle_size / 2.0,
            particle_size / 2.0,
            particle_size / 2.0,
            particle_size / 2.0,
        ]
        .to_vec();

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

        let render_pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 2 * 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: 4 * 8,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![1 => Float32x2, 2 => Float32x2, 3=> Float32, 4=>Float32x3],
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
            primitive: wgpu::PrimitiveState{
                topology:wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format:Some(wgpu::IndexFormat::Uint32),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let work_group_count =
            ((num_particles as f32) / (PARTICLES_PER_GROUP as f32)).ceil() as u32;

        Self {
            init,

            // compute
            particle_buffer,
            particle_uniform_data,
            particle_uniform_buffer,
            compute_bind_group,
            compute_pipeline,
            work_group_count,
            num_particles,

            // render
            vertex_buffer,
            render_pipeline,
            render_bind_group,

            // parameters
            unif_mp,
            rng,
            start,
            t0: 0.0,
            t1: 0.0,
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
        let t = self.start.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let dt0 = t - self.t0;
        if dt0 >= 2.0 {
            for i in 4..12 {
                self.particle_uniform_data[i] = self.unif_mp.sample(&mut self.rng) * 240.0;
            }
            self.t0 = t;
        }

        let dt1 = t - self.t1;
        self.t1 = t;
        self.particle_uniform_data[2] = dt1;

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
                    label: Some("Render Encoder"),
                });

        {
            // compute pass
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
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

            self.init.queue.write_buffer(
                &self.particle_uniform_buffer,
                0,
                bytemuck::cast_slice(&self.particle_uniform_data),
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.particle_buffer.slice(..));
            render_pass.draw(0..4, 0..self.num_particles);
        }

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
