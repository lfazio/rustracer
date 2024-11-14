use std::{cmp, fmt, ops};

use crate::objects::point3::Point3;
use crate::rng;

#[derive(Debug, Clone, Default)]
pub struct Vector3 {
    components: [f64; 3],
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            components: [x, y, z],
        }
    }

    pub fn new_random_unit() -> Vector3 {
        loop {
            let v = Vector3::new(
                rng::random_range(-1.0, 1.0),
                rng::random_range(-1.0, 1.0),
                rng::random_range(-1.0, 1.0),
            );
            let lensq = v.dot(&v);
            if 1e-160 < lensq && lensq < 1.0 {
                return v.normalise();
            }
        }
    }

    pub fn new_random_in_unit_disk() -> Vector3 {
        loop {
            let p = Vector3::new(
                rng::random_range(-1.0, 1.0),
                rng::random_range(-1.0, 1.0),
                0.0,
            );
            if p.dot(&p) < 1.0 {
                break p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.x()) < s && f64::abs(self.y()) < s && f64::abs(self.z()) < s
    }

    pub fn x(&self) -> f64 {
        self.components[0]
    }

    pub fn y(&self) -> f64 {
        self.components[1]
    }

    pub fn z(&self) -> f64 {
        self.components[2]
    }

    pub fn norm(&self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    pub fn normalise(&self) -> Vector3 {
        self.clone() / self.norm()
    }

    pub fn dot(&self, rhs: &Vector3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
        v - 2.0 * v.dot(n) * n
    }

    pub fn refract(&self, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = f64::min(Vector3::dot(&(-self), n), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.dot(&r_out_perp))) * n;

        &r_out_perp + &r_out_parallel
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x(), self.y(), self.z())
    }
}

impl cmp::PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl From<Point3> for Vector3 {
    fn from(p: Point3) -> Vector3 {
        Vector3::new(p.x(), p.y(), p.z())
    }
}

impl From<&Point3> for Vector3 {
    fn from(p: &Point3) -> Vector3 {
        Vector3::new(p.x(), p.y(), p.z())
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self {
        Vector3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<&Vector3> for Vector3 {
    type Output = Self;

    fn add(self, other: &Vector3) -> Self {
        Vector3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Self::Output {
        Vector3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.components[0] += other.x();
        self.components[1] += other.y();
        self.components[2] += other.z();
    }
}

impl ops::AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, other: &Vector3) {
        self.components[0] += other.x();
        self.components[1] += other.y();
        self.components[2] += other.z();
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Vector3) -> Self {
        Vector3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<&Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, other: &Vector3) -> Self {
        Vector3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Self::Output {
        Vector3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Vector3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.components[0] -= other.x();
        self.components[1] -= other.y();
        self.components[2] -= other.z();
    }
}

impl ops::SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, other: &Vector3) {
        self.components[0] -= other.x();
        self.components[1] -= other.y();
        self.components[2] -= other.z();
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.components[0] *= rhs;
        self.components[1] *= rhs;
        self.components[2] *= rhs;
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Mul<&f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &f64) -> Vector3 {
        Vector3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<&f64> for Vector3 {
    fn mul_assign(&mut self, rhs: &f64) {
        self.components[0] *= rhs;
        self.components[1] *= rhs;
        self.components[2] *= rhs;
    }
}

impl ops::Mul<Vector3> for &f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Mul<&Vector3> for &f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl ops::Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl ops::Div<&f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: &f64) -> Vector3 {
        Vector3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl ops::Div<&f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: &f64) -> Vector3 {
        Vector3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self.components[0] /= rhs;
        self.components[1] /= rhs;
        self.components[2] /= rhs;
    }
}

impl ops::DivAssign<&f64> for Vector3 {
    fn div_assign(&mut self, rhs: &f64) {
        self.components[0] /= rhs;
        self.components[1] /= rhs;
        self.components[2] /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::default();
        let v3 = Vector3::new(0.0, 0.0, 1.0);

        assert_eq!(v1, v2);
        assert_ne!(v2, v3);
    }

    #[test]
    fn test_add() {
        let v1: Vector3 = Vector3::new(1.0, 0.0, -1.0);
        let v2: Vector3 = Vector3::new(1.0, 1.0, 0.0);

        assert_eq!(v1.clone() + v2.clone(), Vector3::new(2.0, 1.0, -1.0));
        assert_eq!(&v1 + &v2, Vector3::new(2.0, 1.0, -1.0));
        assert_eq!(v1.clone() + &v2, Vector3::new(2.0, 1.0, -1.0));
        assert_eq!(&v1 + v2, Vector3::new(2.0, 1.0, -1.0));

        let mut v1: Vector3 = Vector3::new(1.0, 0.0, -1.0);
        let v2: Vector3 = Vector3::new(1.0, 1.0, 0.0);
        v1 += v2.clone();

        assert_eq!(v1, Vector3::new(2.0, 1.0, -1.0));

        v1 += &v2;
        assert_eq!(v1, Vector3::new(3.0, 2.0, -1.0));
    }

    #[test]
    fn test_sub() {
        let mut v1: Vector3 = Vector3::new(1.0, 0.0, -1.0);
        let v2: Vector3 = Vector3::new(1.0, 1.0, 0.0);

        assert_eq!(v1.clone() - v2.clone(), Vector3::new(0.0, -1.0, -1.0));

        v1 -= v2;
        assert_eq!(v1, Vector3::new(0.0, -1.0, -1.0));
    }

    #[test]
    fn test_mul() {
        let mut v1: Vector3 = Vector3::new(1.0, 0.0, -1.0);

        assert_eq!(v1.clone() * 2.0, Vector3::new(2.0, 0.0, -2.0));
        assert_eq!(2.0 * v1.clone(), Vector3::new(2.0, 0.0, -2.0));

        v1 *= 2.0;
        assert_eq!(v1, Vector3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_div() {
        let mut v1: Vector3 = Vector3::new(2.0, 0.0, -2.0);

        assert_eq!(v1.clone() / 2.0, Vector3::new(1.0, 0.0, -1.0));
        assert_eq!(0.5 * v1.clone(), Vector3::new(1.0, 0.0, -1.0));

        v1 /= 2.0;
        assert_eq!(v1, Vector3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_dot() {
        let v1: Vector3 = Vector3::new(1.0, 0.0, 0.0);
        let z: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        assert_eq!(v1.dot(&z), 0.0);
        assert_eq!(z.dot(&v1), 0.0);

        let v2: Vector3 = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 1.0);
        assert_eq!(v2.dot(&v1), 1.0);

        let v2: Vector3 = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v2.dot(&v1), 0.0);

        let v2: Vector3 = Vector3::new(0.0, 0.0, 1.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v2.dot(&v1), 0.0);

        let v1: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        let v2: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 4.0);
        assert_eq!(v2.dot(&v1), 4.0);
    }

    #[test]
    fn test_norm() {
        let v: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        assert_eq!(v.norm(), 0.0);

        let v: Vector3 = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(v.norm(), 1.0);

        let v: Vector3 = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(v.norm(), 1.0);

        let v: Vector3 = Vector3::new(0.0, 0.0, 1.0);
        assert_eq!(v.norm(), 1.0);

        let v: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        assert_eq!(v.norm(), 2.0);

        let v: Vector3 = Vector3::new(2.0, 2.0, 0.0);
        assert_eq!(v.norm(), f64::sqrt(8.0));

        let v: Vector3 = Vector3::new(2.0, 2.0, 2.0);
        assert_eq!(v.norm(), f64::sqrt(12.0));

        let v: Vector3 = Vector3::new(0.0, 2.0, 2.0);
        assert_eq!(v.norm(), f64::sqrt(8.0));
    }

    #[test]
    fn test_normalise() {
        let v: Vector3 = Vector3::new(2.0, 0.0, 0.0);
        let unit_v = v.normalise();
        assert_eq!(unit_v.dot(&unit_v), 1.0);

        let v: Vector3 = Vector3::new(20.0, 20.0, 20.0);
        let unit_v = v.normalise();
        assert_eq!(unit_v.dot(&unit_v), 1.0);
    }

    #[test]
    fn test_cross() {
        let v1: Vector3 = Vector3::new(2.0, 3.0, 4.0);
        let v2: Vector3 = Vector3::new(5.0, 6.0, 7.0);
        let v: Vector3 = v1.cross(&v2);

        assert_eq!(v, Vector3::new(-3.0, 6.0, -3.0));
    }
}
