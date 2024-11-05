use std::fmt;

#[derive(Debug,Clone,Default)]
pub struct PpmColor {
	r: u8,
	g: u8,
	b: u8,
}

#[derive(Debug)]
pub struct Ppm {
	magic: u8,
	w: usize,
	h: usize,
	depth: usize,
	body: Vec<PpmColor>,
}

impl PpmColor {
	pub fn new(r: u8, g: u8, b: u8) -> PpmColor {
		PpmColor { 
			r,
			g,
			b,
		}
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

impl Ppm {
	pub fn new(w: usize, h: usize, depth: usize) -> Ppm {
		Ppm {
			magic: 3,
			w,
			h,
			depth: depth - 1,
			body: vec![PpmColor::new(0, 0, 0); w * h],
		}
	}

	pub fn set(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
		self.body[y * self.w + x].set(r, g, b)

	}
}

impl fmt::Display for Ppm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let _ = write!(f, "P{}\n{} {}\n{}\n", self.magic, self.w, self.h, self.depth);

		for color in &self.body {
			_ = writeln!(f, "{}", color);
		}

		writeln!(f)
	}
}