use crate::vector3::{Point3, Vector3};
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }

    pub fn origin(self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_at() {}
}
