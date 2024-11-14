pub mod color;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use super::material::color::Color;
use crate::{objects::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

#[derive(Debug, Default, Clone)]
pub struct DefaultMaterial;

impl Material for DefaultMaterial {}
