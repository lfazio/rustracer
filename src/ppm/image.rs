use std::fmt;

use crate::{material::color::Color, ppm::color::PpmColor};

#[derive(Debug)]
pub struct Ppm {
    magic: u8,
    w: u32,
    h: u32,
    depth: u32,
    body: Vec<PpmColor>,
}

impl Ppm {
    pub fn new(w: u32, h: u32, depth: u32) -> Ppm {
        Ppm {
            magic: 3,
            w,
            h,
            depth: depth - 1,
            body: vec![PpmColor::new(0, 0, 0); (w * h) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: &Color) {
        self.body[(y * self.w + x) as usize].set(&PpmColor::from(color))
    }
}

impl fmt::Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(
            f,
            "P{}\n{} {}\n{}\n",
            self.magic, self.w, self.h, self.depth
        );

        for color in &self.body {
            _ = writeln!(f, "{}", color);
        }

        writeln!(f)
    }
}
