use super::{hittable::HittableObject, Hit, Interval, Ray, Target};

#[derive(Debug, Clone, PartialEq)]
pub struct TargetList {
    pub list: Vec<Target>,
}

impl TargetList {
    pub fn new() -> Self {
        TargetList { list: Vec::new() }
    }

    pub fn add(&mut self, object: Target) {
        self.list.push(object);
    }
}

impl HittableObject for TargetList {
    fn hit(&self, ray: Ray, t: Interval) -> Option<Hit> {
        let mut closest_hit = None;
        let mut closest_so_far = t.max;

        for object in &self.list {
            if let Some(hit) = object.hit(ray, Interval::new(t.min, closest_so_far)) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
