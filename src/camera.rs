extern crate fastrand;

use crate::{
    interval::Interval,
    objects::{Hittable, HittableList},
    ppm::image::Ppm,
    ray::Ray,
    types::{Color, Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct Camera {
    position: Point3,
    f: f64, // focal length
    img_w: u32,
    img_h: u32,
    samples_per_pixel: u32,
    viewport: Viewport,
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
    let intensity = Interval::new(0.0, 1.0);
    f64_convert(intensity.clamp(v) * 255_f64) as u8
}

impl Camera {
    pub fn new(position: Point3, f: f64, ratio: f64, img_w: u32, antialiasing: u32) -> Camera {
        let img_h = if (f64::from(img_w) / ratio) < 1_f64 {
            1
        } else {
            f64_convert(f64::from(img_w) / ratio)
        };

        Camera {
            position,
            f,
            img_w,
            img_h,
            samples_per_pixel: antialiasing,
            viewport: Viewport::new(2.0, img_w, img_h),
        }
    }

    pub fn focal_length(&self) -> f64 {
        self.f
    }

    pub fn render(&self, world: &HittableList) {
        let mut img = Ppm::new(self.img_w, self.img_h, 256);

        for j in 0..self.img_h {
            for i in 0..self.img_w {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    color += self.ray_color(&self.get_ray(i, j), world);
                }
                color /= f64::from(self.samples_per_pixel);

                img.set(
                    i,
                    j,
                    color_get_level(color.x()),
                    color_get_level(color.y()),
                    color_get_level(color.z()),
                );
            }
        }

        println!("{}", img);
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, 0.0)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.viewport.origin(self)
            + ((f64::from(i) + offset.x()) * self.viewport.du())
            + ((f64::from(j) + offset.y()) * self.viewport.dv());

        Ray::new(self.position.clone(), pixel_sample - self.position.clone())
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        let r: f64;
        let g: f64;
        let b: f64;

        match world.hit(ray, &Interval::new(0.0, f64::INFINITY)) {
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

        Color::new(r, g, b)
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
            c.position.clone() - Vec3::new(0.0, 0.0, c.focal_length()) - (u / 2.0) - (v / 2.0);

        upper_left + (self.du() + self.dv()) / 2.0
    }
}
