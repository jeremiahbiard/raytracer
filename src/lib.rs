pub mod renderer;
pub mod vec3;

use renderer::Color;
use vec3::*;

#[allow(dead_code)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = Vec3::dot_product(ray.direction, ray.direction);
    let b = 2.0 * Vec3::dot_product(oc, ray.direction);
    let c = Vec3::dot_product(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(r: Ray) -> Color {
        let result: Color;

        if hit_sphere(Point3::with_xyz(0.0, 0.0, -1.0), 0.5, &r) {
            result = Color::with_rgb(1.0, 0.0, 0.0);
        } else {
            let unit_direction = Vec3::unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            let c1 = Color::with_rgb(1.0, 1.0, 1.0);
            let c2 = Color::with_rgb(0.5, 0.7, 1.0);
            result = (1.0 - t) * c1 + t * c2;
        }
        result
    }
}
