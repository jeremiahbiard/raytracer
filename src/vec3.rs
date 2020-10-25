use std::ops::{Add, AddAssign, MulAssign};

extern crate math;

#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, s: f64) {
        *self = Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3::default()
    }

    /// Creates a new Vec3 with x, y, and z coords
    pub fn vec_with_xyz(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        let test_vec = Vec3::new();
        assert_eq!(test_vec.x, 0.0);
        assert_eq!(test_vec.y, 0.0);
        assert_eq!(test_vec.z, 0.0);
    }

    #[test]
    fn add_two_vectors() {
        let uut1 = Vec3::vec_with_xyz(0.5, 0.5, 0.5);
        let uut2 = Vec3::vec_with_xyz(1.0, 1.0, 1.0);
        assert_eq!(
            uut1 + uut2,
            Vec3 {
                x: 1.5,
                y: 1.5,
                z: 1.5
            }
        );
    }

    #[test]
    fn add_assign() {
        let mut uut1 = Vec3::vec_with_xyz(1.0, 1.0, 1.0);
        let uut2 = Vec3::vec_with_xyz(1.0, 1.0, 1.0);
        uut1 += uut2;
        assert_eq!(uut1, Vec3::vec_with_xyz(2.0, 2.0, 2.0));
    }

    #[test]
    fn scalar_multiplication() {
        let mut v1 = Vec3::vec_with_xyz(1.0, 1.0, 1.0);
        let v2 = Vec3::vec_with_xyz(3.0, 3.0, 3.0);
        v1 *= 3.0;
        assert_eq!(v1, v2);
    }
}
