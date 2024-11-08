use crate::{
    objects::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

#[derive(Debug, Default, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray.direction(), rec.normal.clone());

        Some((Ray::new(rec.p.clone(), reflected), self.albedo.clone()))
    }
}
