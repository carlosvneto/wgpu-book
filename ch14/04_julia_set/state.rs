use std::sync::Arc;
use std::time::SystemTime;
use wgpu::util::DeviceExt;
use winit::{
    event_loop::{ActiveEventLoop, OwnedDisplayHandle},
    keyboard::KeyCode,
    window::Window,
};

use wgpu_book::wgpu_simplified as ws;

pub struct State {
    init: ws::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    cxy: Vec<Vec<f32>>,
    max_iter: i32,
    cxy_num: usize,
    start: SystemTime,
    t0: f32,
    select_color: f32,
}

impl State {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>, select_color: f32) -> Self {
        let init = ws::InitWgpu::init_wgpu(display, window, 1).await;
        let start = SystemTime::now();
        let cxy = vec![
            vec![0.3f32, 0.5],
            vec![0.3, 0.45],
            vec![0.3, 0.46],
            vec![0.3, 0.47],
            vec![0.3, 0.48],
            vec![0.3, 0.49],
            vec![0.24, 0.55],
            vec![0.234, 0.545],
            vec![0.234, 0.54],
            vec![0.235, 0.53],
        ];

        // Loading shader
        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("julia_set.wgsl").into()),
            });

        // uniform data
        let param_data = vec![
            2.0,
            cxy[0][0],
            cxy[0][1],
            init.config.width as f32,
            init.config.height as f32,
            select_color,
        ];

        let frag_uniform_buffer =
            init.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Fragment Uniform Buffer"),
                    contents: bytemuck::cast_slice(&param_data),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let uniform_bind_group_layout =
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
                    label: Some("Uniform Bind Group Layout"),
                });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: frag_uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let pipeline_layout = init
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[Some(&uniform_bind_group_layout)],
                immediate_size: 0,
            });

        let pipeline = init
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: init.config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        Self {
            init,
            pipeline,
            uniform_bind_group,
            uniform_buffer: frag_uniform_buffer,
            max_iter: 2,
            cxy,
            cxy_num: 0,
            start,
            t0: 0.0,
            select_color,
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

            let param_data = vec![
                self.max_iter as f32,
                self.cxy[self.cxy_num][0],
                self.cxy[self.cxy_num][1],
                self.init.config.width as f32,
                self.init.config.height as f32,
                self.select_color,
            ];
            self.init.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(&param_data),
            );
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

    pub fn update(&mut self) {
        // We don't have anything to update yet
    }

    pub fn render(&mut self) -> Option<()> {
        let t = self.start.elapsed().unwrap().as_millis() as f32;
        let dt = t - self.t0;
        if dt >= 20.0 {
            let param_data = vec![
                self.max_iter as f32,
                self.cxy[self.cxy_num][0],
                self.cxy[self.cxy_num][1],
                self.init.config.width as f32,
                self.init.config.height as f32,
                self.select_color,
            ];
            self.init.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(&param_data),
            );

            self.max_iter += 1;
            if self.max_iter > 200 {
                self.max_iter = 2;
                self.cxy_num += 1;
            }
            if self.cxy_num >= self.cxy.len() {
                self.cxy_num = 0;
            }
            self.t0 = t;
        }

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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.247,
                            b: 0.314,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            // Pipeline
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Some(())
    }
}
