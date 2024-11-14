use std::{cmp, fmt, ops};

use super::vector3::Vector3;

#[derive(Debug, Clone, Default)]
pub struct Point3 {
    coordinates: [f64; 3],
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {
            coordinates: [x, y, z],
        }
    }

    pub fn x(&self) -> f64 {
        self.coordinates[0]
    }

    pub fn y(&self) -> f64 {
        self.coordinates[1]
    }

    pub fn z(&self) -> f64 {
        self.coordinates[2]
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x(), self.y(), self.z())
    }
}

impl cmp::PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl ops::Neg for Point3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Neg for &Point3 {
    type Output = Point3;

    fn neg(self) -> Self::Output {
        Point3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add for Point3 {
    type Output = Self;

    fn add(self, other: Point3) -> Self {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<&Point3> for Point3 {
    type Output = Self;

    fn add(self, other: &Point3) -> Self {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<Point3> for &Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Self::Output {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<&Vector3> for Point3 {
    type Output = Self;

    fn add(self, other: &Vector3) -> Self {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<&Vector3> for &Point3 {
    type Output = Point3;

    fn add(self, other: &Vector3) -> Self::Output {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<Vector3> for Point3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<Vector3> for &Point3 {
    type Output = Point3;

    fn add(self, other: Vector3) -> Self::Output {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Add<&Point3> for &Point3 {
    type Output = Point3;

    fn add(self, other: &Point3) -> Self::Output {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::AddAssign for Point3 {
    fn add_assign(&mut self, other: Point3) {
        self.coordinates[0] += other.x();
        self.coordinates[1] += other.y();
        self.coordinates[2] += other.z();
    }
}

impl ops::AddAssign<&Point3> for Point3 {
    fn add_assign(&mut self, other: &Point3) {
        self.coordinates[0] += other.x();
        self.coordinates[1] += other.y();
        self.coordinates[2] += other.z();
    }
}

impl ops::Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Point3) -> Self {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<&Point3> for Point3 {
    type Output = Self;

    fn sub(self, other: &Point3) -> Self {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<Point3> for &Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Self::Output {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<&Point3> for &Point3 {
    type Output = Point3;

    fn sub(self, other: &Point3) -> Self::Output {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<Vector3> for Point3 {
    type Output = Self;

    fn sub(self, other: Vector3) -> Self {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<&Vector3> for Point3 {
    type Output = Self;

    fn sub(self, other: &Vector3) -> Self {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<Vector3> for &Point3 {
    type Output = Point3;

    fn sub(self, other: Vector3) -> Self::Output {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<&Vector3> for &Point3 {
    type Output = Point3;

    fn sub(self, other: &Vector3) -> Self::Output {
        Point3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::SubAssign for Point3 {
    fn sub_assign(&mut self, other: Point3) {
        self.coordinates[0] -= other.x();
        self.coordinates[1] -= other.y();
        self.coordinates[2] -= other.z();
    }
}

impl ops::SubAssign<&Point3> for Point3 {
    fn sub_assign(&mut self, other: &Point3) {
        self.coordinates[0] -= other.x();
        self.coordinates[1] -= other.y();
        self.coordinates[2] -= other.z();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let p0 = Point3::default();
        let p1 = Point3::new(0.0, 0.0, 0.0);

        assert_eq!(&p0, &p1);
        assert_eq!(p0, p1);
    }
}
