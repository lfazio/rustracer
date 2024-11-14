use std::ops::{Add, AddAssign, DivAssign, Mul};

use crate::interval::Interval;

#[derive(Debug, Clone, Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        let validity = Interval::new(0.0, 1.0);
        Color {
            r: validity.clamp(r),
            g: validity.clamp(g),
            b: validity.clamp(b),
        }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Color) -> Self {
        Color::new(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}

impl Add<&Color> for Color {
    type Output = Color;

    fn add(self, other: &Color) -> Self::Output {
        Color::new(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}

impl Add<Color> for &Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color::new(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}

impl Add<&Color> for &Color {
    type Output = Color;

    fn add(self, other: &Color) -> Self::Output {
        Color::new(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r();
        self.g += other.g();
        self.b += other.b();
    }
}

impl AddAssign<&Color> for Color {
    fn add_assign(&mut self, other: &Color) {
        self.r += other.r();
        self.g += other.g();
        self.b += other.b();
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Color::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}

impl Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}

impl Mul<Color> for &Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Color::new(self.r() * other, self.g() * other, self.b() * other)
    }
}

impl Mul<&f64> for Color {
    type Output = Self;

    fn mul(self, other: &f64) -> Self::Output {
        Color::new(self.r() * other, self.g() * other, self.b() * other)
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
        Color::new(self.r() * other, self.g() * other, self.b() * other)
    }
}

impl Mul<&f64> for &Color {
    type Output = Color;

    fn mul(self, other: &f64) -> Self::Output {
        Color::new(self.r() * other, self.g() * other, self.b() * other)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(other.r() * self, other.g() * self, other.b() * self)
    }
}

impl Mul<Color> for &f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(other.r() * self, other.g() * self, other.b() * self)
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color::new(other.r() * self, other.g() * self, other.b() * self)
    }
}

impl Mul<&Color> for &f64 {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color::new(other.r() * self, other.g() * self, other.b() * self)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, other: f64) {
        self.r /= other;
        self.g /= other;
        self.b /= other;
    }
}

impl DivAssign<&f64> for Color {
    fn div_assign(&mut self, other: &f64) {
        self.r /= other;
        self.g /= other;
        self.b /= other;
    }
}
