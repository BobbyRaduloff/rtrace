// struct Ray {
//     origin: vec3<f32>,
//     direction: vec3<f32>,
// };

// struct Sphere {
//     center: vec3<f32>,
//     radius: f32,
// };

@group(0) @binding(0) var<storage, read_write> output_hits: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // let id = global_id.x;

    // if (id >= arrayLength(&input_rays)) {
    //     return;
    // }

    // let ray = input_rays[id];
    // let sphere = spheres[0];

    // let oc = ray.origin - sphere.center;
    // let a = dot(ray.direction, ray.direction);
    // let half_b = dot(oc, ray.direction);
    // let c = dot(oc, oc) - sphere.radius * sphere.radius;
    // let discriminant = half_b * half_b - a * c;

    // output_hits[id] = select(2u, 1u, discriminant > 0.0);
    output_hits[id] = 1;
}
