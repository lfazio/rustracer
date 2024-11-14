use crate::{
    objects::{vector3::Vector3, HitRecord},
    ray::Ray,
};

use super::{color::Color, Material};

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
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = &rec.normal + Vector3::new_random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        Some((
            Ray::new(rec.p.clone(), scatter_direction),
            self.albedo.clone(),
        ))
    }
}
