use crate::{
    objects::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

#[derive(Debug, Default, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let hr = rec.clone();
        let mut scatter_direction = hr.normal.clone() + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hr.normal;
        }

        Some((Ray::new(hr.p, scatter_direction), self.albedo.clone()))
    }
}
