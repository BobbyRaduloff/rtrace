use super::Sphere;

pub fn spheres() -> Vec<Sphere> {
    vec![
        // Ground sphere
        Sphere {
            center: [0.0, -1000.0, 0.0],
            radius: 1000.0,
        },
        // Small spheres
        Sphere {
            center: [-1.1, 0.2, -1.1],
            radius: 0.2,
        },
        Sphere {
            center: [1.2, 0.2, -1.2],
            radius: 0.2,
        },
        Sphere {
            center: [1.0, 0.2, 1.0],
            radius: 0.2,
        },
        // Main spheres
        Sphere {
            center: [0.0, 1.0, 0.0],
            radius: 1.0,
        },
        Sphere {
            center: [-4.0, 1.0, 0.0],
            radius: 1.0,
        },
        Sphere {
            center: [4.0, 1.0, 0.0],
            radius: 1.0,
        },
    ]
}
