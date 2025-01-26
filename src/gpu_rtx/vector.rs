use std::ops::{Add, Div, Mul, Sub};

use super::Interval;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    pub components: [f32; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(components: [f32; N]) -> Self {
        Self { components }
    }

    pub fn zero() -> Self {
        Self {
            components: [0.0; N],
        }
    }

    pub fn length(&self) -> f32 {
        self.components.iter().map(|&x| x * x).sum::<f32>().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.components.iter().map(|&x| x * x).sum()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.components
            .iter()
            .filter(|&x| x.abs() < s)
            .collect::<Vec<_>>()
            .len()
            == self.components.len()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            return Self::zero();
        }
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self.components[i] / len;
        }
        Self::new(result)
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Self, etai_over_etat: f32) -> Self {
        let cos_theta = f32::min((-1.0 * *self).dot(normal), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self.components[i] + other.components[i];
        }
        Self::new(result)
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self.components[i] - other.components[i];
        }
        Self::new(result)
    }
}

impl<const N: usize> Mul<f32> for Vector<N> {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self.components[i] * scalar;
        }
        Self::new(result)
    }
}

impl<const N: usize> Mul<Vector<N>> for f32 {
    type Output = Vector<N>;

    fn mul(self, vector: Vector<N>) -> Self::Output {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = vector.components[i] * self;
        }
        Vector::new(result)
    }
}

impl<const N: usize> Div<f32> for Vector<N> {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self.components[i] / scalar;
        }
        Self::new(result)
    }
}

impl<const N: usize> Vector<N> {
    pub fn random() -> Self {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = fastrand::f32();
        }
        Self::new(components)
    }

    pub fn random_range(range: Interval) -> Self {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = fastrand::f32() * (range.max - range.min) + range.min;
        }

        Self::new(components)
    }
}

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, vector: Vector<N>) -> Self::Output {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = vector.components[i] * self.components[i];
        }
        Vector::new(result)
    }
}

impl Vector<3> {
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vector::<3>::random_range(Interval::new(-1.0, 1.0));
            let len_squared = p.length_squared();
            if 1e-160 < len_squared && len_squared <= 1.0 {
                return p / len_squared.sqrt();
            }
        }
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -1.0 * on_unit_sphere
        }
    }

    pub fn cross(&self, other: Self) -> Self {
        let [x1, y1, z1] = self.components;
        let [x2, y2, z2] = other.components;

        Vector::new([y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2])
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new([
                fastrand::f32() * 2.0 - 1.0,
                fastrand::f32() * 2.0 - 1.0,
                0.0,
            ]);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}
