use super::Sphere;

pub fn spheres() -> Vec<Sphere> {
    vec![
        // Ground sphere
        Sphere {
            center: [0.0, -999.9, 0.0],
            radius: 1000.0,

            albedo: [0.5, 0.5, 0.5],
            material_id: 0,

            fuzz: 0.0,
            refraction_index: 0.0,
            _pad1: 0.0,
            _pad2: 0.0,
        },
        Sphere {
            center: [0.0, 0.0, 0.0],
            radius: 2.0,

            albedo: [0.5, 0.5, 0.5],
            material_id: 0,

            fuzz: 0.0,
            refraction_index: 0.0,
            _pad1: 0.0,
            _pad2: 0.0,
        },
    ]
}
