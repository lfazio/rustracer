use std::fmt;

use crate::{interval::Interval, material::color::Color};

#[derive(Debug, Clone, Default)]
pub struct PpmColor {
    r: u8,
    g: u8,
    b: u8,
}

impl PpmColor {
    pub fn new(r: u8, g: u8, b: u8) -> PpmColor {
        PpmColor { r, g, b }
    }

    pub fn set(&mut self, color: &PpmColor) {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }

    0.0
}

impl From<Color> for PpmColor {
    fn from(c: Color) -> PpmColor {
        let intensity = Interval::new(0.000, 255.0);

        PpmColor {
            r: intensity.clamp(linear_to_gamma(c.r()) * 255_f64) as u32 as u8,
            g: intensity.clamp(linear_to_gamma(c.g()) * 255_f64) as u32 as u8,
            b: intensity.clamp(linear_to_gamma(c.b()) * 255_f64) as u32 as u8,
        }
    }
}

impl fmt::Display for PpmColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
