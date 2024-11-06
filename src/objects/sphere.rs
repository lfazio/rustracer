use crate::objects::View;
use crate::ray::Ray;
use crate::types::{Color, Point3};

#[derive(Debug)]
pub struct Sphere {
    o: Point3,
    r: f64,
    color: Color,
}

impl Sphere {
    pub fn new(o: Point3, r: f64, color: Point3) -> Sphere {
        Sphere { o, r, color }
    }

    pub fn o(&self) -> Point3 {
        self.o.clone()
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }
}

impl View for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = self.o() - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let h = ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.r() * self.r();
        let discriminant = h * h - (a * c);

        if discriminant < 0.0 {
            None
        } else {
            Some((h - f64::sqrt(discriminant)) / a)
        }
    }
}
