pub mod sphere;

use crate::ray::Ray;

pub trait View {
    fn hit(&self, ray: &Ray) -> bool;
}
