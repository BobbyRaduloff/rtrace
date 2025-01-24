use super::{Hit, LambertianData, Ray, ScatterResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Lambertian(LambertianData),
}

impl Material {
    pub fn scatter(self, incoming: Ray, hit: Hit) -> ScatterResult {
        match self {
            Material::Lambertian(data) => data.scatter(incoming, hit),
        }
    }
}
