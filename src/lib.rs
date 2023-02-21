pub mod camera;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vector3;

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
