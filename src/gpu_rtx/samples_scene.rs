use super::Sphere;

pub fn spheres() -> Vec<Sphere> {
    vec![
        // Ground sphere
        Sphere {
            radius: 2.0,
            _pad2: [0.0, 0.0, 0.0],
            center: [0.0, 0.0, 0.0],
            _pad1: 0.0,
        },
    ]
}
