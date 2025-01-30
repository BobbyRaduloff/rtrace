struct Ray {
    origin: vec3<f32>,
    bounces_left: u32,
    direction: vec3<f32>,
    color:vec3<f32>,
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
@group(0) @binding(2) var<storage, read_write> output_rays: array<Ray>;

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;

    if (id >= arrayLength(&output_rays)) {
        return;
    }

    let ray = input_rays[id];
    if (ray.bounces_left <= 0) {
        return;
    }

    var closest_t = 1e20;
    var hit_normal = vec3<f32>(0.0, 0.0, 0.0);
    var hit_point = vec3<f32>(0.0, 0.0, 0.0);
    var hit_sphere: Sphere;

    var new_ray = ray;
    for (var i: u32 = 0; i < arrayLength(&spheres); i += 1) {
        let sphere = spheres[i];
        let t = hit(ray.origin, ray.direction, sphere.center, sphere.radius);

        if (t > 0 && t < closest_t) {
            closest_t = t;
            hit_point = ray.origin + ray.direction * t;
            hit_normal = normalize(hit_point - sphere.center);
            hit_sphere = sphere;
        }
    }

    new_ray.origin = hit_point;
    new_ray.bounces_left = ray.bounces_left - 1;

    if (closest_t < 0  || closest_t >= 1e20 - 1) {
        let direction = normalize(ray.direction);
        let lerp_t = 0.5 * (direction.y + 1.0);
        let color = (1.0 - lerp_t) * vec3(1.0, 1.0, 1.0) + lerp_t * vec3(0.5, 0.7, 1.0);

        new_ray.bounces_left = 0;
        new_ray.color = ray.color * color;
        output_rays[id] = new_ray;
        return;
    }

    if (hit_sphere.material_id == 0u) {
        // lambertian
        let scatter_dir = hit_normal + random_unit_vector(global_id);
        let scattered_direction = select(hit_normal, scatter_dir, length(scatter_dir) > 0.001);

        new_ray.direction = normalize(scattered_direction);
        new_ray.color = ray.color * hit_sphere.albedo;
    } else if (hit_sphere.material_id == 1u) {
        // metal
        let reflected = reflect(normalize(ray.direction), hit_normal);
        let scattered_direction = reflected + hit_sphere.fuzz * random_unit_vector(global_id);

        new_ray.direction = normalize(scattered_direction);
        new_ray.color = ray.color * hit_sphere.albedo;
    } else if (hit_sphere.material_id == 2u) {
        let unit_direction = normalize(ray.direction);
        let refraction_ratio = select(1.0 / hit_sphere.refraction_index, hit_sphere.refraction_index, dot(unit_direction, hit_normal) < 0.0);
        let cos_theta = min(dot(-unit_direction, hit_normal), 1.0);
        let sin_theta = sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = select(
            reflect(unit_direction, hit_normal),
            refract(unit_direction, hit_normal, refraction_ratio),
            cannot_refract
        );

        new_ray.direction = direction;
    }


    output_rays[id] = new_ray;
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

fn random_unit_vector(global_id: vec3<u32>) -> vec3<f32> {
    let a = random_f32(global_id) * 2.0 * 3.14159265359;
    let z = random_f32(global_id) * 2.0 - 1.0;
    let r = sqrt(1.0 - z * z);
    return vec3<f32>(r * cos(a), r * sin(a), z);
}

fn random_f32(global_id: vec3<u32>) -> f32 {
    return fract(sin(dot(vec2<f32>(f32(global_id.x), f32(global_id.y)), vec2<f32>(12.9898, 78.233))) * 43758.5453);
}
