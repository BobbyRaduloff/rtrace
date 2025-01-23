use super::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vector<3>,
    pub direction: Vector<3>,
}

impl Ray {
    pub fn new(origin: Vector<3>, direction: Vector<3>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector<3> {
        return self.origin + t * self.direction;
    }
}
