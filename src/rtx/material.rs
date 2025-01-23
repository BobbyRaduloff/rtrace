use super::{Hit, Ray, RGB};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScatterResult {
    pub incoming: Ray,
    pub hit: Hit,
    pub attenuation: RGB,
    pub scattered: Ray,
}

impl ScatterResult {
    pub fn new(incoming: Ray, hit: Hit, attenuation: RGB, scattered: Ray) -> Self {
        Self {
            incoming,
            hit,
            attenuation,
            scattered,
        }
    }
}

pub enum Material {
    Lambertian(RGB),
}

impl Material {
    fn scatter(&self, incoming: Ray, hit: Hit) -> Option<ScatterResult> {}
}
