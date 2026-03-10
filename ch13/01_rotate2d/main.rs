use wgpu::util::DeviceExt;

async fn run(point: Vec<f32>, angle: f32) -> Option<Vec<f32>> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    // Adapter:
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            //compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            ..Default::default()
        })
        .await
        .unwrap();

    // Logical Device and Queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    // Loading shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("rotate2d.wgsl").into()),
    });

    let point_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Point Buffer"),
        contents: bytemuck::cast_slice(&point),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let angle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Angle Buffer"),
        contents: bytemuck::cast_slice(&[angle]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::STORAGE,
    });

    let result_buffer_size: u64 = 8;
    let result_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: result_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let read_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: result_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    ty: wgpu::BufferBindingType::Uniform,
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
        label: Some("Bind Group Layout"),
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: point_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: angle_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: result_buffer.as_entire_binding(),
            },
        ],
        label: Some("Uniform Bind Group"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        immediate_size: 0,
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        cache: None,
        compilation_options: wgpu::PipelineCompilationOptions::default(),
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.insert_debug_marker("compute collatz iterations");
        compute_pass.dispatch_workgroups(2, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    encoder.copy_buffer_to_buffer(&result_buffer, 0, &read_buffer, 0, result_buffer_size);
    queue.submit(Some(encoder.finish()));

    // read buffer
    let read_buffer_slice = read_buffer.slice(..);

    let (sender, receiver) = flume::bounded(1);
    read_buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    //device.poll(wgpu::Maintain::wait()).panic_on_timeout();
    // Wait for the GPU to finish working on the submitted work
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();


    if let Ok(Ok(())) = receiver.recv_async().await {
        // get buffer content
        let data = read_buffer_slice.get_mapped_range();

        let result = bytemuck::cast_slice(&data).to_vec();

        drop(data);

        read_buffer.unmap();

        println!("result = {:?}", result);

        Some(result)
    } else {
        panic!("failed to run compute on gpu!")
    }
}

fn main() {
    let mut point = "1.0,0.0";
    let mut angle = "45.0";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        point = &args[1];
    }
    if args.len() > 2 {
        angle = &args[2];
    }
    let pt = point
        .split(",")
        .filter_map(|s| s.parse::<f32>().ok())
        .collect::<Vec<_>>();
    let agl = angle.parse::<f32>();

    env_logger::init();
    pollster::block_on(run(pt, agl.unwrap()));
}
