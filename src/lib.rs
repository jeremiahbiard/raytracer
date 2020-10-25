pub mod renderer;
pub mod vec3;

use vec3::*;
use renderer::Color;

#[allow(dead_code)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(r: Ray) -> Color {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        let c1 = Color::with_rgb(1.0, 1.0, 1.0);
        let c2 = Color::with_rgb(0.5, 0.7, 1.0);
        (1.0 - t) * c1 + t * c2
    }
}
