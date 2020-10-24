#[derive(Default, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3::default()
    }

    pub fn vec_with_xyz(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_default() {
        let test_vec = Vec3::new();
        assert_eq!(test_vec.x, 0.0);
        assert_eq!(test_vec.y, 0.0);
        assert_eq!(test_vec.z, 0.0);
    }
}
