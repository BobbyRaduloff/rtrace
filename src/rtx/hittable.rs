use std::sync::Arc;

use super::{Hit, Interval, Ray, Target, TargetList};

#[derive(Debug, Clone, PartialEq)]
pub enum Hittable {
    Single(Target),
    Multiple(TargetList),
    MultiplePtr(Arc<TargetList>),
}

pub trait HittableObject {
    fn hit(&self, ray: Ray, t: Interval) -> Option<Hit>;
}

impl HittableObject for Hittable {
    fn hit(&self, ray: Ray, t: Interval) -> Option<Hit> {
        match self {
            Hittable::Single(target) => target.hit(ray, t),
            Hittable::Multiple(list) => list.hit(ray, t),
            Hittable::MultiplePtr(ptr) => ptr.hit(ray, t),
        }
    }
}
