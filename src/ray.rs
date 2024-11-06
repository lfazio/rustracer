use crate::types::{Color, Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.direction.clone()
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self) -> Color {
        let unit = self.direction().normalise();
        let a = 0.5_f64 * (unit.y() + 1.0_f64);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
