struct Ray {
    origin: vec3<f32>,
    bounces_left: u32,
    direction: vec3<f32>,
    color: vec3<f32>,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,

    albedo: vec3<f32>, // 0
    material_id: u32,  // 0 = lambertian, 1 = metal, 2 = dielectric

    fuzz: f32, // 1
    refraction_index: f32, // 2
    _pad1: f32,
    _pad2: f32
};

struct SceneUniforms {
    pass_idx: u32,
    bounce_idx: u32,
};

@group(0) @binding(0) var<storage, read> input_rays: array<Ray>;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(2) var<storage, read_write> output_rays: array<Ray>;
@group(0) @binding(3) var<uniform> scene: SceneUniforms;

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

    // Find closest sphere hit
    var closest_t = 1e20;
    var hit_normal = vec3<f32>(0.0, 0.0, 0.0);
    var hit_point = vec3<f32>(0.0, 0.0, 0.0);
    var hit_sphere: Sphere;

    var new_ray = ray;
    for (var i: u32 = 0; i < arrayLength(&spheres); i += 1) {
        let sphere = spheres[i];
        let t = hit(ray.origin, ray.direction, sphere.center, sphere.radius);
        if (t > 0.0 && t < closest_t) {
            closest_t = t;
            hit_point = ray.origin + ray.direction * t;
            hit_normal = normalize(hit_point - sphere.center);
            hit_sphere = sphere;
        }
    }

    // Update ray origin for next bounce
    new_ray.origin = hit_point;
    new_ray.bounces_left = ray.bounces_left - 1;

    // If we missed or it's beyond max distance, shade with sky
    if (closest_t < 0.001 || closest_t >= 1e19) {
        let direction = normalize(ray.direction);
        let lerp_t = 0.5 * (direction.y + 1.0);
        let color = (1.0 - lerp_t) * vec3(1.0, 1.0, 1.0) + lerp_t * vec3(0.5, 0.7, 1.0);

        new_ray.bounces_left = 0;
        new_ray.color = ray.color * color;
        output_rays[id] = new_ray;
        return;
    }

    // Handle materials
    if (hit_sphere.material_id == 0u) {
        // Lambertian (diffuse)
        let scatter_dir = hit_normal + random_unit_vector(global_id);
        // Avoid degenerate scatter directions
        let scattered_direction = select(hit_normal, scatter_dir, length(scatter_dir) > 0.001);

        new_ray.direction = normalize(scattered_direction);
        new_ray.color = ray.color * hit_sphere.albedo;

    } else if (hit_sphere.material_id == 1u) {
        // Metal
        let reflected = reflect(normalize(ray.direction), hit_normal);
        let scattered_direction = reflected + hit_sphere.fuzz * random_unit_vector(global_id);

        new_ray.direction = normalize(scattered_direction);
        new_ray.color = ray.color * hit_sphere.albedo;

    } else if (hit_sphere.material_id == 2u) {
        // Dielectric (glass).  We use a front_face check and Schlick reflectance.

        let unit_direction = normalize(ray.direction);
        // If dot > 0, we're inside the sphere, so flip the normal
        let front_face = dot(unit_direction, hit_normal) < 0.0;

        var out_normal = hit_normal;
        var refraction_ratio = 1.0 / hit_sphere.refraction_index;
        if (!front_face) {
            out_normal = -out_normal;
            refraction_ratio = hit_sphere.refraction_index;
        }

        let cos_theta = min(dot(-unit_direction, out_normal), 1.0);
        let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let reflect_prob = reflectance(cos_theta, hit_sphere.refraction_index);

        // Decide reflection vs. refraction
        let rand_val = random_f32_with_seed(global_id);
        if (cannot_refract || reflect_prob > rand_val) {
            // Reflect
            new_ray.direction = reflect(unit_direction, out_normal);
        } else {
            // Refract
            new_ray.direction = refract(unit_direction, out_normal, refraction_ratio);
        }

        // Keep color as is (glass is usually colorless)
        new_ray.color = ray.color;
    }

    output_rays[id] = new_ray;
}

// Sphere intersection
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

// Random direction on unit sphere
fn random_unit_vector(global_id: vec3<u32>) -> vec3<f32> {
    let a = random_f32_with_seed(global_id) * 2.0 * 3.14159265359;
    let z = random_f32_with_seed(global_id) * 2.0 - 1.0;
    let r = sqrt(1.0 - z * z);
    return vec3<f32>(r * cos(a), r * sin(a), z);
}

// Pseudo-random function with pass/bounce seeds
fn random_f32_with_seed(global_id: vec3<u32>) -> f32 {
    let seed_x = global_id.x ^ (scene.pass_idx * 374761393u) ^ (scene.bounce_idx * 668265263u);
    let seed_y = global_id.y ^ (scene.pass_idx * 977123579u) ^ (scene.bounce_idx * 265443578u);

    let t = vec2<f32>(f32(seed_x), f32(seed_y));
    return fract(sin(dot(t, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// Reflect a vector around a normal
fn reflect(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - 2.0 * dot(v, n) * n;
}

// Refract a vector given a normal and refraction ratio
fn refract(uv: vec3<f32>, n: vec3<f32>, etai_over_etat: f32) -> vec3<f32> {
    let cos_theta = dot(-uv, n);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -sqrt(abs(1.0 - dot(r_out_perp, r_out_perp))) * n;
    return r_out_perp + r_out_parallel;
}

// Schlick reflectance approximation
fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Schlick's approximation
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0_sqr = r0 * r0;
    return r0_sqr + (1.0 - r0_sqr) * pow(1.0 - cosine, 5.0);
}
