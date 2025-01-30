pub mod gpu_rtx;

use gpu_rtx::{samples_scene, Camera, Ray, SceneUniforms, Vector, RGB};
use std::fs::File;
use std::io::Write;
use wgpu::util::DeviceExt;

const TOTAL_SAMPLES: usize = 500;
const BATCH_SIZE: usize = 10; // We'll generate and trace 10 rays per pixel per batch
const WORKGROUP_SIZE: u32 = 64;

#[tokio::main]
async fn main() {
    // Setup camera and scene
    let lookfrom = Vector::new([0.0, 0.0, -3.0]);
    let lookat = Vector::new([0.0, 0.0, 0.0]);
    let camera = Camera::new(
        380,
        320,
        50, // max_depth
        60.0,
        lookfrom,
        lookat,
        Vector::new([0.0, 1.0, 0.0]),
        0.0,
        (lookfrom - lookat).length(),
    );
    let spheres = samples_scene::spheres();
    let max_depth = camera.max_depth;

    // We'll accumulate pixel colors in CPU memory
    let mut accum_colors = vec![[0.0_f32; 3]; camera.image_width * camera.image_height];

    // WGPU boilerplate
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find adapter");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("RTX Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let sphere_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Sphere Buffer"),
        contents: bytemuck::cast_slice(&spheres),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Group Layout"),
        entries: &[
            // binding 0: input rays
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
            // binding 1: spheres
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
            // binding 2: output rays
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
            // binding 3: uniforms
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
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
        cache: None,
        compilation_options: Default::default(),
    });

    // We'll do (TOTAL_SAMPLES / BATCH_SIZE) passes.
    // Each pass:
    //   1) Generate BATCH_SIZE rays per pixel (CPU side)
    //   2) Ping-pong them for up to max_depth bounces (GPU side)
    //   3) Read back the final color portion of each ray
    //   4) Accumulate into CPU array
    let passes = TOTAL_SAMPLES / BATCH_SIZE;

    for pass_idx in 0..passes {
        // 1) Generate BATCH_SIZE rays per pixel
        let rays = camera.generate_rays_count(BATCH_SIZE); // we'll define generate_rays_count
        let total_rays = rays.len();

        // Create GPU buffers for these rays
        let ray_buffer_size = (total_rays * std::mem::size_of::<Ray>()) as wgpu::BufferAddress;

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

        // 2) Ping-pong for up to max_depth bounces
        for bounce in 0..max_depth {
            let uniforms = SceneUniforms {
                pass_idx: pass_idx as u32,
                bounce_idx: bounce as u32,
            };
            let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scene Uniforms"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

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
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ],
                label: Some("Compute Bind Group"),
            });

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Encoder"),
            });
            {
                let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Compute Pass"),
                    timestamp_writes: None,
                });
                pass.set_pipeline(&compute_pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                pass.dispatch_workgroups(
                    ((total_rays as u32) + (WORKGROUP_SIZE - 1)) / WORKGROUP_SIZE,
                    1,
                    1,
                );
            }
            queue.submit(Some(encoder.finish()));

            std::mem::swap(&mut ray_buffer_a, &mut ray_buffer_b);
        }

        // After the loop, ray_buffer_a has the final traced rays for this batch
        // 3) Copy them back to CPU
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

        let slice = staging_buffer.slice(..);
        slice.map_async(wgpu::MapMode::Read, |r| {
            if let Err(e) = r {
                panic!("Failed to map buffer: {:?}", e);
            }
        });
        device.poll(wgpu::Maintain::Wait);

        let data = slice.get_mapped_range();
        let final_rays: &[Ray] = bytemuck::cast_slice(&data);

        // 4) Accumulate color in CPU array
        // each pixel had BATCH_SIZE rays, so total_rays = (width * height * BATCH_SIZE)
        let mut idx = 0;
        for pix in 0..(camera.image_width * camera.image_height) {
            // We'll just average these BATCH_SIZE final rays
            // Or we can sum them now and do final average at the end
            for _ in 0..BATCH_SIZE {
                let c = final_rays[idx].color;
                accum_colors[pix][0] += c[0];
                accum_colors[pix][1] += c[1];
                accum_colors[pix][2] += c[2];
                idx += 1;
            }
        }
    }

    // Now accum_colors contains the sum of all (TOTAL_SAMPLES) samples
    // We do final averaging and write to PPM
    let mut file = File::create("output.ppm").expect("Failed to create file");
    writeln!(
        file,
        "P3\n{} {}\n255",
        camera.image_width, camera.image_height
    )
    .unwrap();

    for pix in 0..(camera.image_width * camera.image_height) {
        let mut c = accum_colors[pix];
        c[0] /= TOTAL_SAMPLES as f32;
        c[1] /= TOTAL_SAMPLES as f32;
        c[2] /= TOTAL_SAMPLES as f32;
        let rgb = RGB::new([c[0], c[1], c[2]]);
        writeln!(file, "{}", rgb).unwrap();
    }

    println!("Render completed. Written to output.ppm");
}
