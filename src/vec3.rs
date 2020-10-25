use std::ops::*;

// extern crate math;

#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

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

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
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
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
         Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
         }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3 {
        x: self.x * other.x,
        y: self.y * other.y,
        z: self.z * other.z,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
         Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
         }
    }
}


#[allow(dead_code)]
impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3::default()
    }

    /// Creates a new Vec3 with x, y, and z coords
    pub fn with_xyz(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot_product(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross_product(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
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
        let uut1 = Vec3::with_xyz(0.5, 0.5, 0.5);
        let uut2 = Vec3::with_xyz(1.0, 1.0, 1.0);
        assert_eq!(
            uut1 + uut2,
            Vec3 {
                x: 1.5,
                y: 1.5,
                z: 1.5,
            }
            )
        
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec3::with_xyz(1.0, 1.0, 1.0);
        let v2 = Vec3::with_xyz(1.0, 1.0, 1.0);
        v1 += v2;
        assert_eq!(v1, Vec3::with_xyz(2.0, 2.0, 2.0));
    }

    #[test]
    fn scalar_multiplication() {
        let mut v1 = Vec3::with_xyz(1.0, 1.0, 1.0);
        let v2 = Vec3::with_xyz(3.0, 3.0, 3.0);
        v1 *= 3.0;
        assert_eq!(v1, v2);
    }
    
    #[test]
    fn length() {
        let v1 = Vec3::with_xyz(1.0, 1.0, 1.0);
        assert_eq!(v1.length(), 1.7320508075688772);
        let v2 = Vec3::default();
        assert_eq!(v2.length(), 0.0);
    }

    #[test]
    fn length_squared() {
        let v1 = Vec3::with_xyz(1.0, 1.0, 1.0);
        assert_eq!(v1.length_squared(), 3.0);
    }

    // TODO: Write some unit tests for overloade operators
}
