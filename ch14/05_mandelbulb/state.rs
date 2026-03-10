use std::sync::Arc;
use std::time::SystemTime;
use glam::DVec2;
use wgpu::util::DeviceExt;
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    event::MouseButton,
    window::Window,
};

use wgpu_book::wgpu_simplified as ws;

pub struct State {
    init: ws::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    start: SystemTime,
    time: f32,
    max_iter: f32,
    mousex: f32,
    mousey: f32,
    pub mouse_pressed: bool,
    prior_mouse_pos: Option<DVec2>,
    scale: f32,
    mouse_control: bool,
}

impl State {
    pub async fn new(window: Arc<Window>, max_iter: f32, scale: f32, mouse_control: bool) -> Self {
        let init = ws::InitWgpu::init_wgpu(window, 1).await;
        let start = SystemTime::now();

        // Loading shader
        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("mandelbulb.wgsl").into()),
            });

        // initial mouse position coordinates
        let prior_mouse_pos = DVec2::new(1.0, 1.0);

        // uniform data
        let param_data = vec![
            0.0,
            max_iter,
            0.0,
            0.0,
            init.config.width as f32,
            init.config.height as f32,
            scale as f32,
        ];

        let uniform_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&param_data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group_layout =
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

        let bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let pipeline_layout = init
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
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
            pipeline,
            bind_group,
            uniform_buffer: uniform_buffer,
            start,
            time: 0.0,
            max_iter,
            mousex: 0.0,
            mousey: 0.0,
            mouse_pressed: false,
            prior_mouse_pos: Some(prior_mouse_pos),
            scale: scale as f32,
            mouse_control,
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
    
    pub fn handle_mouse_moved(&mut self, x: f64, y: f64) {
        let position = DVec2::new(x, y);
        if self.mouse_pressed && self.mouse_control {
            if let Some(prior) = self.prior_mouse_pos {
                let delta = position - prior;
                self.mousex = self.mousex + delta.x as f32;
                self.mousey = self.mousey + delta.y as f32;
            }
        }
        self.prior_mouse_pos = Some(position);
    }
    
    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Right => self.mouse_pressed = pressed,
            _ => {}
        }
    }
    
    pub fn handle_key_input(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        match (key, pressed) {
            (KeyCode::Escape, true) => {
                event_loop.exit();
            } 
            _ => {},
        }
    }

    pub fn update(&mut self) {
        // We don't have anything to update yet
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.time = self.start.elapsed().unwrap().as_secs_f32();
        let param_data = vec![
            self.time,
            self.max_iter,
            self.mousex,
            self.mousey,
            self.init.config.width as f32,
            self.init.config.height as f32,
            self.scale as f32,
        ];
        self.init
            .queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&param_data));

        let output = self.init.surface.get_current_texture()?;

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
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
