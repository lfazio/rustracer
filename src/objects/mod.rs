use std::rc::Rc;

pub mod point3;
pub mod sphere;
pub mod vector3;

use crate::interval::Interval;
use crate::material::{DefaultMaterial, Material};
use crate::objects::point3::Point3;
use crate::objects::vector3::Vector3;
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::default(),
            normal: Vector3::default(),
            t: 0.0,
            front_face: false,
            mat: Rc::new(DefaultMaterial::default()),
        }
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = Vector3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, rayt: &Interval) -> Option<HitRecord>;
}

pub struct HittableList<'t> {
    pub objects: Vec<Rc<&'t dyn Hittable>>,
}

impl<'t> HittableList<'t> {
    pub fn new() -> HittableList<'t> {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, o: &'t dyn Hittable) {
        self.objects.push(Rc::<&dyn Hittable>::new(o));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'t> Hittable for HittableList<'t> {
    fn hit(&self, ray: &Ray, rayt: &Interval) -> Option<HitRecord> {
        let mut hit_anything: bool = false;
        let mut closest_so_far = rayt.max();
        let mut temp_rec = HitRecord::new();

        for o in &self.objects {
            let interval = Interval::new(rayt.min(), closest_so_far);
            if let Some(hr) = o.hit(ray, &interval) {
                hit_anything = true;
                closest_so_far = hr.t;
                temp_rec = hr;
            }
        }

        if hit_anything {
            return Some(temp_rec);
        }

        None
    }
}
