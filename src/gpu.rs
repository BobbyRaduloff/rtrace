pub mod gpu_rtx;

use gpu_rtx::{Camera, Ray, Sphere, Vector};
use std::fs::File;
use std::io::Write;
use wgpu::util::DeviceExt;

#[tokio::main]
async fn main() {
    // Scene setup: single sphere in the center
    let sphere = Sphere {
        center: [0.0, 0.0, -5.0], // Sphere at z = -5.0
        radius: 1.0,
    };
    let camera = Camera::new(
        480, // image_width
        320, // image_height
        1,   // samples_per_pixel (only 1 sample here)
        1,   // max_depth (unused)
        90.0,
        Vector::new([0.0, 0.0, 0.0]),  // Look from
        Vector::new([0.0, 0.0, -1.0]), // Look at
        Vector::new([0.0, 1.0, 0.0]),  // Up
        0.0,
        1.0,
    );
    let rays: Vec<Ray> = camera.generate_rays(camera.image_width, camera.image_height);

    // Initialize WGPU
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find a suitable GPU adapter.");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device.");

    // Load the shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("RTX Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let ray_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Ray Buffer"),
        contents: bytemuck::cast_slice(&rays),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let sphere_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Sphere Buffer"),
        contents: bytemuck::cast_slice(&[sphere]),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer_size =
        (camera.image_width * camera.image_height * std::mem::size_of::<u32>()) as u64;
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    // Bind group and compute pipeline setup
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Hit Bind Group Layout"),
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
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: ray_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: sphere_buffer.as_entire_binding(),
            },
        ],
        label: Some("Hit Bind Group"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Hit Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        cache: None,
        compilation_options: Default::default(),
    });

    // Dispatch compute pass
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Command Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(
            (camera.image_width as u32 + 63) / 64,
            camera.image_height as u32,
            1,
        );
    }

    queue.submit(Some(encoder.finish()));

    // Read output buffer
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Copy Encoder"),
    });
    encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_buffer_size);
    queue.submit(Some(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |result| {
        if let Err(e) = result {
            panic!("Failed to map buffer: {:?}", e);
        }
    });
    device.poll(wgpu::Maintain::Wait);

    let data = buffer_slice.get_mapped_range();
    let results: &[u32] = bytemuck::cast_slice(&data);
    println!("{:?}", results);

    // Write results to PPM
    let mut file = File::create("output.ppm").expect("Failed to create PPM file");
    writeln!(
        file,
        "P3\n{} {}\n255",
        camera.image_width, camera.image_height
    )
    .unwrap();

    for y in 0..camera.image_height {
        for x in 0..camera.image_width {
            let idx = y * camera.image_width + x;
            let value = results[idx] * 255; // 1 = hit, 0 = miss
            writeln!(file, "{} {} {}", value, value, value).unwrap();
        }
    }

    println!("Render completed. Output saved to output.ppm");
}
