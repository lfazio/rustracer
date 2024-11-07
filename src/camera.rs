use crate::{
    interval::Interval,
    objects::{Hittable, HittableList},
    ppm::image::Ppm,
    ray::Ray,
    types::{Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct Camera {
    position: Point3,
    f: f64, // focal length
    ratio: f64,
    img_w: usize,
}

#[derive(Debug, Default)]
pub struct Viewport {
    u: Vec3,
    v: Vec3,
    du: Vec3,
    dv: Vec3,
}

fn f64_convert(x: f64) -> u32 {
    x.round().rem_euclid(2f64.powi(32)) as u32
}

fn color_get_level(v: f64) -> u8 {
    f64_convert(v * 255_f64) as u8
}

impl Camera {
    pub fn new(position: Point3, f: f64, ratio: f64, img_w: usize) -> Camera {
        Camera {
            position,
            f,
            ratio,
            img_w,
        }
    }

    pub fn focal_length(&self) -> f64 {
        self.f
    }

    pub fn position(&self) -> Point3 {
        self.position.clone()
    }

    pub fn render(&self, world: &HittableList) {
        let img_h = if f64::from(self.img_w as u32) / self.ratio < 1_f64 {
            1_usize
        } else {
            f64_convert(f64::from(self.img_w as u32) / self.ratio) as usize
        };
        let mut img = Ppm::new(self.img_w, img_h, 256);
        let viewport = Viewport::new(2.0, self.img_w as u32, img_h as u32);

        for j in 0..img_h {
            for i in 0..self.img_w {
                let pixel = viewport.origin(self)
                    + (f64::from(i as u32) * viewport.du())
                    + (f64::from(j as u32) * viewport.dv());
                let ray_direction = pixel.clone() - self.position();
                let ray = Ray::new(pixel, ray_direction);

                let (r, g, b) = self.ray_color(&ray, world);
                img.set(i, j, r, g, b);
            }
        }

        println!("{}", img);
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> (u8, u8, u8) {
        let rayt = Interval::new(0.0, f64::INFINITY);
        let r: f64;
        let g: f64;
        let b: f64;

        match world.hit(&ray, &rayt) {
            Some(rec) => {
                let normal = &rec.normal;
                r = (normal.x() + 1.0) / 2.0;
                g = (normal.y() + 1.0) / 2.0;
                b = (normal.z() + 1.0) / 2.0;
            }
            None => {
                r = ray.color().x();
                g = ray.color().y();
                b = ray.color().z();
            }
        }

        (color_get_level(r), color_get_level(g), color_get_level(b))
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
