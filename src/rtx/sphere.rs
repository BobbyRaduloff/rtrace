use super::{hittable::Hittable, Hit, Interval, Ray, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Vector<3>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vector<3>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t: Interval) -> Option<Hit> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = h * h - a * c;

        if disc < 0.0 {
            return None;
        }

        let sqrtd = disc.sqrt();
        let mut root = (h - sqrtd) / a;
        if !t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        Some(Hit::new(p, t, ray, outward_normal))
    }
}
