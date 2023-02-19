use core::fmt;
use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{:}, {:}, {:}}}", self.x, self.y, self.z)
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, _rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f64) -> Vector3 {
        self * (1. / _rhs)
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    #[inline]
    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        (u.x * v.x) + (u.y * v.y) + (u.z * v.z)
    }

    #[inline]
    pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
        Vector3 {
            x: (u.y * v.z) - (u.z * v.y),
            y: (u.z * v.x) - (u.x * v.z),
            z: (u.x * v.y) - (u.y * v.x),
        }
    }

    #[inline]
    pub fn unit_vector(v: Vector3) -> Vector3 {
        v / v.length()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_squared() {
        let test_vec = Vector3::new(1., 1., 1.);
        assert_eq!(test_vec.length_squared(), 3.0)
    }

    #[test]
    fn test_length() {
        let test_vec = Vector3::new(1., 1., 1.);
        assert_eq!(test_vec.length(), 1.7320508075688772)
    }

    #[test]
    fn test_cross() {
        let test_vec1 = Vector3::new(-3., 0., -2.);
        let test_vec2: Vector3 = Vector3::new(0.5, -1., 2.);
        let test_vec3: Vector3 = Vector3::new(-2., 5., 3.);
        assert_eq!(Vector3::cross(&test_vec1, &test_vec2), test_vec3)
    }

    #[test]
    fn test_dot() {
        let test_vec1 = Vector3::new(-3., 0., -2.);
        let test_vec2: Vector3 = Vector3::new(0.5, -1., 2.);
        assert_eq!(Vector3::dot(&test_vec1, &test_vec2), -5.5)
    }

    #[test]
    fn test_multiply_vector3_by_f64() {
        let test_vec: Vector3 = Vector3::new(-1., -1., -1.);
        let scalar = -2.;
        let result = Vector3 {
            x: 2.,
            y: 2.,
            z: 2.,
        };
        assert_eq!(test_vec * scalar, result)
    }

    #[test]
    fn test_divide_vector3_by_f64() {
        let test_vec: Vector3 = Vector3::new(-1., -1., -1.);
        let scalar = -2.;
        let result = Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        };
        assert_eq!(test_vec / scalar, result)
    }

    #[test]
    fn test_unit_vector() {
        let test_vec: Vector3 = Vector3::new(12., 4., 3.);
        let result = Vector3 {
            x: 0.9230769230769231,
            y: 0.3076923076923077,
            z: 0.23076923076923078,
        };
        assert_eq!(Vector3::unit_vector(test_vec), result)
    }
}
