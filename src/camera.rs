use crate::types::{Point3, Vec3};

#[derive(Debug, Default)]
pub struct Camera {
    position: Point3,
    f: f64, // focal length
}

#[derive(Debug, Default)]
pub struct Viewport {
    u: Vec3,
    v: Vec3,
    du: Vec3,
    dv: Vec3,
}

impl Camera {
    pub fn new(position: Point3, f: f64) -> Camera {
        Camera { position, f }
    }

    pub fn focal_length(&self) -> f64 {
        self.f
    }

    pub fn position(&self) -> Point3 {
        self.position.clone()
    }
}

impl Viewport {
    pub fn new(scale: f64, w: u32, h: u32) -> Viewport {
        let vw = scale * f64::from(w) / f64::from(h);
        let vh = -scale;

        Viewport {
            u: Vec3::new(vw, 0.0, 0.0),
            v: Vec3::new(0.0, vh, 0.0),
            du: Vec3::new(vw / f64::from(w), 0.0, 0.0),
            dv: Vec3::new(0.0, vh / f64::from(h), 0.0),
        }
    }

    pub fn du(&self) -> Vec3 {
        self.du.clone()
    }

    pub fn dv(&self) -> Vec3 {
        self.dv.clone()
    }

    pub fn origin(&self, c: &Camera) -> Point3 {
        let u = self.u.clone();
        let v = self.v.clone();
        let upper_left =
            c.position() - Vec3::new(0.0, 0.0, c.focal_length()) - (u / 2.0) - (v / 2.0);

        upper_left + (self.du() + self.dv()) / 2.0
    }
}
