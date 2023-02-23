pub mod camera;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vector3;

use vector3::Color3;

const MN: f64 = 256.0;
pub const SAMPLES_PER_PIXEL: u32 = 100;

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

pub fn write_color(data: &mut [u8], index: usize, pixel_color: Color3) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    data[index] = (MN * clamp(r, 0.0, 0.999)) as u8; //(MN * r) as u8;
    data[index + 1] = (MN * clamp(g, 0.0, 0.999)) as u8; // (MN * g) as u8;
    data[index + 2] = (MN * clamp(b, 0.0, 0.999)) as u8; // (MN * b) as u8;
}
