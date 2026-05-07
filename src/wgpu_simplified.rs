use std::sync::Arc;
use winit::window::Window;

// region: wgpu initialization
pub struct InitWgpu {
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub sample_count: u32,
    pub window: Arc<Window>,
}

impl InitWgpu {
    pub async fn init_wgpu(
        window: Arc<Window>,
        sample_count: u32,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // Surface
        let surface = instance.create_surface(window.clone()).unwrap();

        // Adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        // Logical Device and Queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats[0];

        let device = Arc::new(device);

        let size = window.inner_size();

        // Defines how a Surface creates a SurfaceTexture.
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Self {
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            sample_count,
            window: window,
        }
    }
}
// endregion: wgpu initialization

// region: pipelines
pub struct IRenderPipeline<'a> {
    pub shader: Option<&'a wgpu::ShaderModule>,
    pub vs_shader: Option<&'a wgpu::ShaderModule>,
    pub fs_shader: Option<&'a wgpu::ShaderModule>,
    pub vertex_buffer_layout: &'a [wgpu::VertexBufferLayout<'a>],
    pub pipeline_layout: Option<&'a wgpu::PipelineLayout>,
    pub topology: wgpu::PrimitiveTopology,
    pub strip_index_format: Option<wgpu::IndexFormat>,
    pub cull_mode: Option<wgpu::Face>,
    pub is_depth_stencil: bool,
    pub vs_entry: String,
    pub fs_entry: String,
}

impl Default for IRenderPipeline<'_> {
    fn default() -> Self {
        Self {
            shader: None,
            vs_shader: None,
            fs_shader: None,
            vertex_buffer_layout: &[],
            pipeline_layout: None,
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            cull_mode: None,
            is_depth_stencil: true,
            vs_entry: String::from("vs_main"),
            fs_entry: String::from("fs_main"),
        }
    }
}

impl IRenderPipeline<'_> {
    pub fn new(&mut self, init: &InitWgpu) -> wgpu::RenderPipeline {
        if self.shader.is_some() {
            self.vs_shader = self.shader;
            self.fs_shader = self.shader;
        }

        let mut depth_stencil: Option<wgpu::DepthStencilState> = None;
        if self.is_depth_stencil {
            depth_stencil = Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            });
        }

        init.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&self.pipeline_layout.unwrap()),
                vertex: wgpu::VertexState {
                    module: &self.vs_shader.as_ref().unwrap(),
                    entry_point: Some(&self.vs_entry),
                    buffers: &self.vertex_buffer_layout,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.fs_shader.as_ref().unwrap(),
                    entry_point: Some(&self.fs_entry),
                    targets: &[Some(init.config.format.into())],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: self.topology,
                    strip_index_format: self.strip_index_format,
                    ..Default::default()
                },
                depth_stencil,
                multisample: wgpu::MultisampleState {
                    count: init.sample_count,
                    ..Default::default()
                },
                multiview_mask: None,
                cache: None,
            })
    }
}
// endregion: pipelines
