use super::{Hit, Material, Ray, ScatterResult, Vector, RGB};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lambertian {
    pub albedo: RGB,
}

impl Lambertian {
    pub fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, incoming: Ray, hit: Hit) -> Option<super::ScatterResult> {
        let scattered = hit.normal + Vector::random_unit_vector();
        let outgoing = Ray::new(
            hit.p,
            if scattered.near_zero() {
                hit.normal
            } else {
                scattered
            },
        );

        return Some(ScatterResult::new(incoming, hit, self.albedo, outgoing));
    }
}
