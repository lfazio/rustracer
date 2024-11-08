use crate::{
    interval::Interval,
    objects::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

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
        let mut reflected = Vec3::reflect(ray.direction(), rec.normal.clone());

        reflected = reflected.normalise() + (self.fuzz * Vec3::random_unit());
        let scattered = Ray::new(rec.p.clone(), reflected);
        if scattered.direction().dot(&rec.normal) > 0.0 {
            return Some((scattered, self.albedo.clone()));
        }

        None
    }
}
