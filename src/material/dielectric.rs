use crate::{
    objects::{vector3::Vector3, HitRecord},
    ray::Ray,
    rng,
};

use super::{color::Color, Material};

#[derive(Debug, Default, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = f64::powi((1.0 - refraction_index) / (1.0 + refraction_index), 2);

        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = Vector3::normalise(ray.direction());
        let cos_theta = f64::min((-&unit_direction).dot(&rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = (ri * sin_theta) > 1.0;

        let direction = if cannot_refract || (self.reflectance(cos_theta, ri) > rng::random()) {
            Vector3::reflect(&unit_direction, &rec.normal)
        } else {
            Vector3::refract(&unit_direction, &rec.normal, ri)
        };

        Some((
            Ray::new(rec.p.clone(), direction),
            Color::new(1.0, 1.0, 1.0),
        ))
    }
}
