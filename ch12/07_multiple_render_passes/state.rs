use bytemuck::cast_slice;
use glam::Mat4;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{
    event_loop::{ActiveEventLoop, OwnedDisplayHandle},
    keyboard::KeyCode,
    window::Window,
};

use wgpu_book::transforms;
use wgpu_book::wgpu_simplified as ws;

use crate::vertex_mesh::{Vertex2, create_vertices2};
use crate::vertex_surface::{Vertex, create_vertices};

use crate::light::light;

const ANIMATION_SPEED: f32 = 1.0;
const IS_PERSPECTIVE: bool = true;

// for mesh

pub struct State {
    init: ws::InitWgpu,
    view_mat: Mat4,
    project_mat: Mat4,

    // for surface
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    vertex_uniform_buffer: wgpu::Buffer,
    num_vertices: u32,

    // for mesh
    pipeline2: wgpu::RenderPipeline,
    vertex_buffer2: wgpu::Buffer,
    num_vertices2: u32,
    color: Vec<f32>,
}

impl State {
    pub async fn new(
        display: OwnedDisplayHandle,
        window: Arc<Window>,
        colormap_name: &str,
        color: Vec<f32>,
    ) -> Self {
        let init = ws::InitWgpu::init_wgpu(display, window, 1).await;

        let light_data = light([1.0, 1.0, 0.0], 0.1, 0.8, 0.4, 30.0, 1);
        let data = create_vertices(colormap_name);
        let data2 = create_vertices2();

        // Loading Shaders
        let shader = init
            .device
            .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // uniform data
        let camera_position = (3.5, 1.75, 3.5).into();
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = (0.0, 1.0, 0.0).into();

        let (view_mat, project_mat, _view_project_mat) = transforms::create_view_projection(
            camera_position,
            look_direction,
            up_direction,
            init.config.width as f32 / init.config.height as f32,
            IS_PERSPECTIVE,
        );

        // create vertex uniform buffer
        // model_mat and view_projection_mat will be stored in vertex_uniform_buffer inside the update function
        let vertex_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Uniform Buffer"),
            size: 208,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // create fragment uniform buffer. here we set eye_position = camera_position and light_position = eye_position
        let fragment_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Fragment Uniform Buffer"),
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light and eye positions
        let light_position: &[f32; 3] = camera_position.as_ref();
        let eye_position: &[f32; 3] = camera_position.as_ref();
        init.queue.write_buffer(
            &fragment_uniform_buffer,
            0,
            bytemuck::cast_slice(light_position),
        );
        init.queue.write_buffer(
            &fragment_uniform_buffer,
            16,
            bytemuck::cast_slice(eye_position),
        );

        // create light uniform buffer
        let light_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Light Uniform Buffer"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light parameters
        init.queue.write_buffer(
            &light_uniform_buffer,
            0,
            bytemuck::cast_slice(&[light_data]),
        );

        let uniform_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: Some("Uniform Bind Group Layout"),
                });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fragment_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform_buffer.as_entire_binding(),
                },
            ],
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
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24Plus,
                    depth_write_enabled: Some(true),
                    depth_compare: Some(wgpu::CompareFunction::LessEqual),
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        let vertex_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let num_vertices = data.len() as u32;

        // for mesh

        // Loading Shaders
        let shader2 = init
            .device
            .create_shader_module(wgpu::include_wgsl!("mesh.wgsl"));

        let pipeline2 = init
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline 2"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader2,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex2::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader2,
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
                    topology: wgpu::PrimitiveTopology::LineList,
                    strip_index_format: None,
                    ..Default::default()
                },
                depth_stencil: None, // change it to "None" for multiple render passes
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        let vertex_buffer2 = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer 2"),
                contents: cast_slice(&data2),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let num_vertices2 = data2.len() as u32;

        Self {
            init,
            view_mat,
            project_mat,

            // for surface
            pipeline,
            vertex_buffer,
            uniform_bind_group,
            vertex_uniform_buffer,
            num_vertices,

            // for mesh
            pipeline2,
            vertex_buffer2,
            num_vertices2,
            color,
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

            self.project_mat =
                transforms::create_projection(width as f32 / height as f32, IS_PERSPECTIVE);
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

    pub fn update(&mut self, dt: std::time::Duration) {
        // update uniform buffer
        let dt = ANIMATION_SPEED * dt.as_secs_f32();
        let model_mat = transforms::create_transforms(
            [0.0, 0.0, 0.0],
            [dt.sin(), dt.cos(), 0.0],
            [1.0, 1.0, 1.0],
        );
        let view_project_mat = self.project_mat * self.view_mat;

        let normal_mat = (model_mat.inverse()).transpose();

        let model_ref: &[f32; 16] = model_mat.as_ref();
        let view_projection_ref: &[f32; 16] = view_project_mat.as_ref();
        let normal_ref: &[f32; 16] = normal_mat.as_ref();
        let color: [f32; 4] = [self.color[0], self.color[1], self.color[2], 1.0];

        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            0,
            bytemuck::cast_slice(model_ref),
        );
        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            64,
            bytemuck::cast_slice(view_projection_ref),
        );
        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            128,
            bytemuck::cast_slice(normal_ref),
        );
        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            192,
            bytemuck::cast_slice(&color),
        );
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

        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            // draw surface
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass 2"),
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
                        store: wgpu::StoreOp::Store, // true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard, // false,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        {
            // draw mesh
            let mut render_pass2 = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store, // true,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass2.set_pipeline(&self.pipeline2);
            render_pass2.set_vertex_buffer(0, self.vertex_buffer2.slice(..));
            render_pass2.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass2.draw(0..self.num_vertices2, 0..1);
        }

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Some(())
    }
}
