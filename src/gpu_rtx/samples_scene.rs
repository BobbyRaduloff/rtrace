use super::Sphere;

pub fn spheres() -> Vec<Sphere> {
    vec![
        // Ground sphere
        Sphere {
            center: [0.0, -100.5, -1.0],
            radius: 100.0,

            albedo: [0.8, 0.8, 0.0], // Lambertian (diffuse)
            material_id: 0,          // 0 = Lambertian

            fuzz: 0.0,
            refraction_index: 0.0,
            _pad1: 0.0,
            _pad2: 0.0,
        },
        // Center sphere
        Sphere {
            center: [0.0, 0.0, -1.2],
            radius: 0.5,

            albedo: [0.1, 0.2, 0.5], // Lambertian (diffuse)
            material_id: 0,          // 0 = Lambertian

            fuzz: 0.0,
            refraction_index: 0.0,
            _pad1: 0.0,
            _pad2: 0.0,
        },
        // Left sphere (Glass)
        Sphere {
            center: [-1.0, 0.0, -1.0],
            radius: 0.5,

            albedo: [1.0, 1.0, 1.0], // Dielectric has no albedo effect
            material_id: 2,          // 2 = Dielectric

            fuzz: 0.0,
            refraction_index: 1.50,
            _pad1: 0.0,
            _pad2: 0.0,
        },
        // Left bubble (inverted glass sphere)
        Sphere {
            center: [-1.0, 0.0, -1.0],
            radius: 0.4,

            albedo: [1.0, 1.0, 1.0], // Dielectric
            material_id: 2,          // 2 = Dielectric

            fuzz: 0.0,
            refraction_index: 1.0 / 1.50, // Bubble effect
            _pad1: 0.0,
            _pad2: 0.0,
        },
        // Right sphere (Metal)
        Sphere {
            center: [1.0, 0.0, -1.0],
            radius: 0.5,

            albedo: [0.8, 0.6, 0.2], // Metal color
            material_id: 1,          // 1 = Metal

            fuzz: 0.0, // No fuzziness
            refraction_index: 0.0,
            _pad1: 0.0,
            _pad2: 0.0,
        },
    ]
}
