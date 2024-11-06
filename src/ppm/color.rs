use std::fmt;

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

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}

impl fmt::Display for PpmColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
