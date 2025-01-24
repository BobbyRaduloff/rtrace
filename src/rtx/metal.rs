use super::{Hit, Ray, ScatterResult, Vector, RGB};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetalData {
    pub albedo: RGB,
    pub fuzz: f64,
}

impl MetalData {
    pub fn new(albedo: RGB, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.max(1.0),
        }
    }

    pub fn scatter(self, incoming: Ray, hit: Hit) -> ScatterResult {
        let reflected = incoming.direction.reflect(hit.normal);
        let fuzzed = reflected.normalize() + (self.fuzz * Vector::<3>::random_unit_vector());
        let outgoing = Ray::new(hit.p, fuzzed);

        return ScatterResult::new(incoming, hit, self.albedo, outgoing);
    }
}
