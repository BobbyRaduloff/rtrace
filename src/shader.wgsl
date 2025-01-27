struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,

    albedo: vec3<f32>, // 0
    material_id: u32, // 0 = lambertian, 1 = metal, 2 = dielectric

    fuzz: f32, // 1
    refraction_index: f32, // 2
    _pad1: f32,
    _pad2: f32
};

@group(0) @binding(0) var<storage, read> input_rays: array<Ray>;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(2) var<storage, read_write> output_hits: array<vec4<f32>>;

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;

    if (id >= arrayLength(&output_hits)) {
        return;
    }

    let ray = input_rays[id];
    let sphere = spheres[0];

    let t = hit(ray.origin, ray.direction, sphere.center, sphere.radius);

    // if no hit, sky
    if (t < 0.0) {
        let direction = normalize(ray.direction);
        let lerp_t = 0.5 * (direction.y + 1.0);
        let color = (1.0 - lerp_t) * vec3(1.0, 1.0, 1.0) + lerp_t * vec3(0.5, 0.7, 1.0);
        output_hits[id] = vec4(color, 1.0);
        return;
    }

    output_hits[id] = vec4(sphere.albedo, 1.0);
}


fn hit(ro: vec3<f32>, rd: vec3<f32>, center: vec3<f32>, r: f32) -> f32 {
    let oc = ro - center;
    let a = dot(rd, rd);
    let half_b = dot(oc, rd);
    let c = dot(oc, oc) - r * r;
    let discriminant = half_b * half_b - a * c;
    if (discriminant < 0.0) {
        return -1.0;
    }
    let sqrtd = sqrt(discriminant);
    var root = (-half_b - sqrtd) / a;
    if root < 0.001 {
        root = (-half_b + sqrtd) / a;
        if root < 0.001 {
            return -1.0;
        }
    }
    return root;
}
