pub mod lambertian;
pub mod metal;

use crate::{objects::HitRecord, ray::Ray, vec3::Color};

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

#[derive(Debug, Default, Clone)]
pub struct DefaultMaterial;

impl Material for DefaultMaterial {}
