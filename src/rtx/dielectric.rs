use super::{Hit, Ray, ScatterResult, RGB};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DielectricData {
    pub refraction_index: f64,
}

impl DielectricData {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn scatter(self, incoming: Ray, hit: Hit) -> ScatterResult {
        let attenuation = RGB::new([1.0, 1.0, 1.0]);
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = incoming.direction.normalize();
        let cos_theta = (-1.0 * unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta) > fastrand::f64() {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, ri)
        };

        return ScatterResult::new(incoming, hit, attenuation, Ray::new(hit.p, direction));
    }

    fn reflectance(self, cosine: f64) -> f64 {
        let r0 = ((1.0 - self.refraction_index) / (1.0 + self.refraction_index)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
