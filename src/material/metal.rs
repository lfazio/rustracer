use crate::{interval::Interval, objects::vector3::Vector3, objects::HitRecord, ray::Ray};

use super::color::Color;
use super::Material;

#[derive(Debug, Default, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: Interval::new(0.0, 1.0).clamp(fuzz),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = Vector3::reflect(ray.direction(), &rec.normal);

        reflected = reflected.normalise() + (self.fuzz * Vector3::new_random_unit());
        let scattered = Ray::new(rec.p.clone(), reflected);
        if scattered.direction().dot(&rec.normal) > 0.0 {
            return Some((scattered, self.albedo.clone()));
        }

        None
    }
}
