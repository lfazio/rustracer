use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone)]
pub struct Sphere {
    o: Point3,
    r: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(o: Point3, r: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere { o, r, material }
    }

    pub fn o(&self) -> Point3 {
        self.o.clone()
    }

    pub fn r(&self) -> f64 {
        self.r
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, rayt: &Interval) -> Option<HitRecord> {
        let oc = self.o() - ray.origin();
        let a = ray.direction().dot(&ray.direction());
        let h = ray.direction().dot(&oc);
        let c = oc.dot(&oc) - self.r() * self.r();
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;
        if !rayt.surrounds(root) {
            root = (h + sqrtd) / a;
            if !rayt.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = ray.at(root);
        rec.mat = self.material.clone();
        let outward_normal = (ray.at(root) - self.o()) / self.r();
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
