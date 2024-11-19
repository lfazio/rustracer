use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Material;
use crate::objects::point3::Point3;
use crate::objects::vector3::Vector3;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Clone)]
pub struct Sphere {
    o: Point3,
    r: f64,
    material: Rc<dyn Material>,
    center: Ray,
}

impl Sphere {
    pub fn new(o: Point3, r: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            o: o.clone(),
            r,
            material,
            center: Ray::new(o, Vector3::default()),
        }
    }

    pub fn with_motion(o: Point3, o2: Point3, r: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            o: o.clone(),
            r,
            material,
            center: Ray::new(o.clone(), Vector3::from(&o2 - &o)),
        }
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
        let current_center = self.center.at(ray.time());
        let oc = Vector3::from(&current_center - ray.origin());
        let a = ray.direction().dot(ray.direction());
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
        let outward_normal = Vector3::from(&rec.p - &current_center) / self.r();
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
