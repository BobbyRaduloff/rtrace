use std::sync::Arc;

use super::{Hittable, Material, Ray, Vector};

#[derive(Clone)]
pub struct Hit {
    pub p: Vector<3>,
    pub normal: Vector<3>,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl Hit {
    pub fn new(
        p: Vector<3>,
        t: f64,
        ray: Ray,
        outward_normal: Vector<3>,
        target: Arc<dyn Hittable>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            target,
        }
    }
}
