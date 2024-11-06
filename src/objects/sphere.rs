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
    fn hit(&self, ray: &Ray) -> bool {
        let oc = self.o() - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let b = -2.0 * ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.r() * self.r();

        (b * b - 4.0 * a * c) >= 0.0
    }
}
