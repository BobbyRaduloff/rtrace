use super::{HittableObject, Interval, Ray, SphereData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    Sphere(SphereData),
}

impl HittableObject for Target {
    fn hit(&self, ray: Ray, t: Interval) -> Option<super::Hit> {
        match self {
            Target::Sphere(data) => data.hit(ray, t),
        }
    }
}
