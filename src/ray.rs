use crate::objects::point3::Point3;
use crate::objects::vector3::Vector3;

#[derive(Debug, Default, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn with_motion(origin: Point3, direction: Vector3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + &(self.direction() * t)
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
