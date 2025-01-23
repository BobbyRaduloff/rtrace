use std::sync::Arc;

use super::{Hit, Hittable, Interval, Ray};

pub struct HittableList {
    pub list: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: Vec::new() }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.list.push(object);
    }
}

impl Hittable for HittableList {
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
