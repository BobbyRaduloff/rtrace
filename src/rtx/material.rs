use super::{DielectricData, Hit, LambertianData, MetalData, Ray, ScatterResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Lambertian(LambertianData),
    Metal(MetalData),
    Dielectric(DielectricData),
}

impl Material {
    pub fn scatter(self, incoming: Ray, hit: Hit) -> ScatterResult {
        match self {
            Material::Lambertian(data) => data.scatter(incoming, hit),
            Material::Metal(data) => data.scatter(incoming, hit),
            Material::Dielectric(data) => data.scatter(incoming, hit),
        }
    }
}
