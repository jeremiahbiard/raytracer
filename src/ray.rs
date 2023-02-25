use crate::hittable::*;
use crate::vector3::{Color3, Point3, Vector3};

#[derive(Clone, Copy)]
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

pub fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color3 {
    let mut hit_record = HitRecord::new();

    if depth == 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, std::f64::INFINITY, &mut hit_record) {
        let target = hit_record.p + hit_record.normal + Vector3::random_unit_vector();
        // 0.5 * (hit_record.normal + Color3::new(1.0, 1.0, 1.0))
        0.5 * ray_color(
            Ray::new(hit_record.p, target - hit_record.p),
            world,
            depth - 1,
        )
    } else {
        let unit_direction = Vector3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + (t * Color3::new(0.5, 0.7, 1.0))
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test_at() {}
}
