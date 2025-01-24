use std::ops::{Add, Div, Mul, Sub};

use super::Interval;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    pub components: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(components: [f64; N]) -> Self {
        Self { components }
    }

    pub fn zero() -> Self {
        Self {
            components: [0.0; N],
        }
    }

    pub fn length(&self) -> f64 {
        self.components.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
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

    pub fn dot(&self, other: Self) -> f64 {
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
}

// Operator Overloads

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

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self.components[i] * scalar;
        }
        Self::new(result)
    }
}

impl<const N: usize> Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, vector: Vector<N>) -> Self::Output {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = vector.components[i] * self;
        }
        Vector::new(result)
    }
}

impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
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
            components[i] = fastrand::f64();
        }
        Self::new(components)
    }

    pub fn random_range(range: Interval) -> Self {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = fastrand::f64() * (range.max - range.min) + range.min;
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
}
