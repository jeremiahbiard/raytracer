use crate::ray::*;
use crate::vector3::*;
#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vector3,
    t: f64,
    front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vector3) {
        self.front_face = Vector3::dot(&r.direction(), &outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }

    pub fn set_front_face(&mut self, v: bool) {
        self.front_face = v
    }

    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }
    pub fn t(self) -> f64 {
        self.t
    }

    pub fn set_p(&mut self, p: Vector3) {
        self.p = p;
    }

    pub fn p(self) -> Vector3 {
        self.p
    }
}
