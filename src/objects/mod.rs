use std::rc::Rc;

pub mod sphere;

use crate::interval::Interval;
use crate::material::{DefaultMaterial, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Rc::new(DefaultMaterial::default()),
        }
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -1.0 * outward_normal.clone()
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
        let mut closest_so_far = rayt.max;
        let mut temp_rec = HitRecord::new();

        for o in &self.objects {
            let interval = Interval::new(rayt.min, closest_so_far);
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
