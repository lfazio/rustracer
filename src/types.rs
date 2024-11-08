extern crate fastrand;

use std::cmp;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            x: e0,
            y: e1,
            z: e2,
        }
    }

    fn random(&self) -> Vec3 {
        Vec3 {
            x: fastrand::f64(),
            y: fastrand::f64(),
            z: fastrand::f64(),
        }
    }

    fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: min + fastrand::f64() * (max - min),
            y: min + fastrand::f64() * (max - min),
            z: min + fastrand::f64() * (max - min),
        }
    }

    pub fn random_unit() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.dot(&p);
            if 1e-160 < lensq && lensq < 1.0 {
                return p / lensq;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -1.0 * on_unit_sphere
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn norm(&self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    pub fn normalise(&self) -> Vec3 {
        self.clone() / self.norm()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x(), self.y(), self.z())
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x();
        self.y += rhs.y();
        self.z += rhs.z();
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x();
        self.y -= rhs.y();
        self.z -= rhs.z();
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut v1: Vec3 = Vec3::new(1.0, 0.0, -1.0);
        let v2: Vec3 = Vec3::new(1.0, 1.0, 0.0);

        assert_eq!(v1.clone() + v2.clone(), Vec3::new(2.0, 1.0, -1.0));

        v1 += v2;
        assert_eq!(v1, Vec3::new(2.0, 1.0, -1.0));
    }

    #[test]
    fn test_sub() {
        let mut v1: Vec3 = Vec3::new(1.0, 0.0, -1.0);
        let v2: Vec3 = Vec3::new(1.0, 1.0, 0.0);

        assert_eq!(v1.clone() - v2.clone(), Vec3::new(0.0, -1.0, -1.0));

        v1 -= v2;
        assert_eq!(v1, Vec3::new(0.0, -1.0, -1.0));
    }

    #[test]
    fn test_mul() {
        let mut v1: Vec3 = Vec3::new(1.0, 0.0, -1.0);

        assert_eq!(v1.clone() * 2.0, Vec3::new(2.0, 0.0, -2.0));
        assert_eq!(2.0 * v1.clone(), Vec3::new(2.0, 0.0, -2.0));

        v1 *= 2.0;
        assert_eq!(v1, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_div() {
        let mut v1: Vec3 = Vec3::new(2.0, 0.0, -2.0);

        assert_eq!(v1.clone() / 2.0, Vec3::new(1.0, 0.0, -1.0));
        assert_eq!(0.5 * v1.clone(), Vec3::new(1.0, 0.0, -1.0));

        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_dot() {
        let v: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let z: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v.dot(&z), 0.0);
        assert_eq!(z.dot(&v), 0.0);

        let v1: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let v2: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 1.0);
        assert_eq!(v2.dot(&v1), 1.0);

        let v1: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let v2: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v2.dot(&v1), 0.0);

        let v1: Vec3 = Vec3::new(0.0, 0.0, 1.0);
        let v2: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v2.dot(&v1), 0.0);

        let v1: Vec3 = Vec3::new(2.0, 0.0, 0.0);
        let v2: Vec3 = Vec3::new(2.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 4.0);
        assert_eq!(v2.dot(&v1), 4.0);
    }

    #[test]
    fn test_norm() {
        let v: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v.norm(), 0.0);

        let v: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(v.norm(), 1.0);

        let v: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v.norm(), 1.0);

        let v: Vec3 = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(v.norm(), 1.0);

        let v: Vec3 = Vec3::new(2.0, 0.0, 0.0);
        assert_eq!(v.norm(), 2.0);

        let v: Vec3 = Vec3::new(2.0, 2.0, 0.0);
        assert_eq!(v.norm(), f64::sqrt(8.0));

        let v: Vec3 = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.norm(), f64::sqrt(12.0));

        let v: Vec3 = Vec3::new(0.0, 2.0, 2.0);
        assert_eq!(v.norm(), f64::sqrt(8.0));
    }

    #[test]
    fn test_normalise() {
        let v: Vec3 = Vec3::new(2.0, 0.0, 0.0);
        let unit_v = v.normalise();
        assert_eq!(unit_v.dot(&unit_v), 1.0);

        let v: Vec3 = Vec3::new(20.0, 20.0, 20.0);
        let unit_v = v.normalise();
        assert_eq!(unit_v.dot(&unit_v), 1.0);
    }

    #[test]
    fn test_cross() {
        let v1: Vec3 = Vec3::new(2.0, 3.0, 4.0);
        let v2: Vec3 = Vec3::new(5.0, 6.0, 7.0);
        let v: Vec3 = v1.cross(&v2);

        assert_eq!(v, Vec3::new(-3.0, 6.0, -3.0));
    }
}
