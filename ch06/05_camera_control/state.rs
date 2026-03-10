use std::sync::Arc;
use bytemuck::cast_slice;
use glam::{Mat4, DVec2};
use wgpu::util::DeviceExt;
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    event::MouseButton,
    window::Window,
};

use crate::camera;
use crate::vertex::{create_vertices, Vertex};
use wgpu_book::transforms;
use wgpu_book::wgpu_simplified as ws;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_mat: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_mat: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_project(&mut self, camera: &camera::Camera, project_mat: Mat4) {
        self.view_mat = (project_mat * camera.view_mat()).to_cols_array_2d()
    }
}

pub struct State {
    init: ws::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    // New objects
    camera: camera::Camera,
    projection: Mat4,
    camera_controller: camera::CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    mouse_pressed: bool,
    prior_mouse_pos: Option<DVec2>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let init = ws::InitWgpu::init_wgpu(window, 1).await;

        // Loading Shaders
        let shader = init
            .device
            .create_shader_module(wgpu::include_wgsl!("cube_face_color.wgsl"));

        // initial mouse position coordinates
        let prior_mouse_pos = DVec2::new(1.0, 1.0);

        // uniform data
        let camera = camera::Camera::new((2.0, 3.0, 5.0), -80.0, -30.0);
        let projection =
            transforms::create_projection(init.config.width as f32 / init.config.height as f32, true);
        let camera_controller = camera::CameraController::new(0.005);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_project(&camera, projection);

        let camera_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            init.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Uniform Bind Group Layout"),
            });

        let camera_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            immediate_size: 0,
        });

        let pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
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
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&create_vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            init,
            pipeline,
            vertex_buffer,

            camera,
            projection,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            mouse_pressed: false,
            prior_mouse_pos: Some(prior_mouse_pos),
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
        if self.mouse_pressed {
            if let Some(prior) = self.prior_mouse_pos {
                let delta = position - prior;
                self.camera_controller.mouse_move(delta.x, delta.y);
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
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform
            .update_view_project(&self.camera, self.projection);
        self.init.queue
            .write_buffer(&self.camera_buffer, 0, cast_slice(&[self.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.init.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .init
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            // Pipeline
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // Uniform bind Group
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.draw(0..36, 0..1);
        }

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}