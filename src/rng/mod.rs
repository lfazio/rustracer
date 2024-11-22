extern crate fastrand;

pub fn random() -> f64 {
    fastrand::f64()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

pub fn random_range_u32(min: u32, max: u32) -> u32 {
    ((min + (max - min)) as f64 * random()) as u32
}
