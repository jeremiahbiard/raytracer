use crate::hittable::*;
use crate::ray::*;
use crate::vector3::*;
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hr: &mut HitRecord) -> bool {
        let oc = &mut (ray.origin() - self.center);
        let a = ray.direction().length_squared();
        let half_b = Vector3::dot(oc, &ray.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        hr.set_t(root);
        hr.set_p(ray.at(hr.t()));
        let outward_normal = (hr.p() - self.center) / self.radius;
        hr.set_face_normal(ray, outward_normal);
        true
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}
