use super::{Hit, Ray, ScatterResult, Vector, RGB};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LambertianData {
    pub albedo: RGB,
}

impl LambertianData {
    pub fn new(albedo: RGB) -> Self {
        Self { albedo }
    }

    pub fn scatter(self, incoming: Ray, hit: Hit) -> ScatterResult {
        let scattered = hit.normal + Vector::random_unit_vector();
        let outgoing = Ray::new(
            hit.p,
            if scattered.near_zero() {
                hit.normal
            } else {
                scattered
            },
        );

        return ScatterResult::new(incoming, hit, self.albedo, outgoing);
    }
}
