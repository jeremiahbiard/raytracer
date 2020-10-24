#[derive(Default, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn vec_with_xyz(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
}
