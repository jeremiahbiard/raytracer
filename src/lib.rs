pub mod renderer;
pub mod vec3;

use renderer::Color;
use vec3::*;

#[allow(dead_code)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = Vec3::dot_product(ray.direction, ray.direction);
    let b = 2.0 * Vec3::dot_product(oc, ray.direction);
    let c = Vec3::dot_product(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0. {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(r: Ray) -> Color {
        let mut t = hit_sphere(Point3::with_xyz(0.0, 0.0, -1.0), 0.5, &r);
        if t > 0. {
            let normal = Vec3::unit_vector(r.at(t) - Vec3::with_xyz(0., 0., -1.));
            return 0.5 * Color::with_rgb(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
        }
        let unit_direction = Vec3::unit_vector(r.direction);
        t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::with_rgb(1.0, 1.0, 1.0) + t * Color::with_rgb(0.5, 0.7, 1.0)
    }
}
