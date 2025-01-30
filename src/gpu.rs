// main.rs
pub mod gpu_rtx;

use gpu_rtx::{samples_scene, Camera, Ray, Vector, RGB};
use std::fs::File;
use std::io::Write;
use wgpu::util::DeviceExt;

#[tokio::main]
async fn main() {
    // Scene setup: single sphere in the center
    let lookfrom = Vector::new([0.0, 0.0, -3.0]);
    let lookat = Vector::new([0.0, 0.0, 0.0]);
    let camera = Camera::new(
        480,
        320,
        1,
        50, // max_depth
        90.0,
        lookfrom,
        lookat,
        Vector::new([0.0, 1.0, 0.0]),
        0.0,
        (lookfrom - lookat).length(),
    );
    let rays = camera.generate_rays();
    let spheres = samples_scene::spheres();
    let max_depth = camera.max_depth;

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

    // Create two ray buffers for ping-pong
    let ray_buffer_size = (rays.len() * std::mem::size_of::<Ray>()) as wgpu::BufferAddress;

    let mut ray_buffer_a = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Ray Buffer A"),
        contents: bytemuck::cast_slice(&rays),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let mut ray_buffer_b = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Ray Buffer B"),
        size: ray_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let sphere_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Sphere Buffer"),
        contents: bytemuck::cast_slice(&spheres),
        usage: wgpu::BufferUsages::STORAGE,
    });

    // Create pipeline layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("RTX Group Layout"),
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
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("RTX Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    // Run the compute shader max_depth times, ping-ponging between buffers
    for _ in 0..max_depth {
        // Bind group: read from A, write to B
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: ray_buffer_a.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: sphere_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: ray_buffer_b.as_entire_binding(),
                },
            ],
            label: Some("RTX Bind Group"),
        });

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
            compute_pass.dispatch_workgroups(((rays.len() as u32) + 63) / 64, 1, 1);
        }
        queue.submit(Some(encoder.finish()));

        // Swap buffers
        std::mem::swap(&mut ray_buffer_a, &mut ray_buffer_b);
    }

    // ray_buffer_a now contains the final results
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: ray_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Copy Encoder"),
    });
    encoder.copy_buffer_to_buffer(&ray_buffer_a, 0, &staging_buffer, 0, ray_buffer_size);
    queue.submit(Some(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |result| {
        if let Err(e) = result {
            panic!("Failed to map buffer: {:?}", e);
        }
    });
    device.poll(wgpu::Maintain::Wait);

    let data = buffer_slice.get_mapped_range();
    let results: &[Ray] = bytemuck::cast_slice(&data);

    // Write results to PPM
    let mut file = File::create("output.ppm").expect("Failed to create PPM file");
    writeln!(
        file,
        "P3\n{} {}\n255",
        camera.image_width, camera.image_height
    )
    .unwrap();

    let mut pixel_index = 0;
    // We generated (width*height*samples_per_pixel) rays originally; each pixel
    // had 'samples_per_pixel' rays. We'll just use the first one for each pixel
    // in this example, or do averaging if needed. For now, let's be minimal:
    for y in 0..camera.image_height {
        for x in 0..camera.image_width {
            let color = results[pixel_index].color;
            pixel_index += camera.samples_per_pixel; // skip the other samples
            let rgb = RGB::new([color[0], color[1], color[2]]);
            writeln!(file, "{}", rgb).unwrap();
        }
    }

    println!("Render completed. Output saved to output.ppm");
}
