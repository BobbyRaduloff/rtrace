use super::{Hit, Interval, Ray};

pub trait Hittable {
    fn hit(&self, ray: Ray, t: Interval) -> Option<Hit>;
}
