extern crate fastrand;

use crate::{
    interval::Interval,
    objects::{Hittable, HittableList},
    ppm::image::Ppm,
    radian::Radian,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct Camera {
    position: Point3,
    lookat: Point3,
    vup: Vec3,
    aspect_ratio: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    focal_length: f64, // focal length
    img_w: u32,
    img_h: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    vfov: f64,

    viewport: Viewport,
}

#[derive(Debug, Default)]
pub struct Viewport {
    u: Vec3,
    v: Vec3,
    w: Vec3,
    du: Vec3,
    dv: Vec3,
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }

    0.0
}

fn color_get_level(v: f64) -> u8 {
    let c = linear_to_gamma(v);
    let intensity = Interval::new(0.000, 255.0);
    intensity.clamp(c * 255_f64) as u32 as u8
}

impl Camera {
    pub fn new(
        position: Point3,
        lookat: Point3,
        vup: Vec3,
        aspect_ratio: f64,
        img_w: u32,
        antialiasing: u32,
        max_depth: u32,
        vfov: f64,
    ) -> Camera {
        let img_h = if (f64::from(img_w) / aspect_ratio) < 1_f64 {
            1
        } else {
            (f64::from(img_w) / aspect_ratio) as u32
        };

        // Viewport dimensions
        let focal_length = (&position - &lookat).norm();
        let theta = Radian::from(vfov);
        let h = f64::tan(theta.value / 2.0);
        let vh = 2.0 * h * focal_length;
        let vw = vh * aspect_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (&position - &lookat).normalise();
        let u = Vec3::cross(&vup, &w).normalise();
        let v = Vec3::cross(&w, &u);

        Camera {
            position,
            lookat,
            vup,
            aspect_ratio,
            u: u.clone(),
            v: v.clone(),
            w: w.clone(),
            focal_length,
            img_w,
            img_h,
            samples_per_pixel: antialiasing,
            max_depth,
            vfov,
            viewport: Viewport::new(2.0, u, v, w, vw, vh, img_w, img_h),
        }
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn render(&self, world: &HittableList) {
        let mut img = Ppm::new(self.img_w, self.img_h, 256);

        for j in 0..self.img_h {
            for i in 0..self.img_w {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    color += self.ray_color(&self.get_ray(i, j), self.max_depth, world);
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

    fn ray_color(&self, ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            Some(rec) => match rec.mat.scatter(ray, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * self.ray_color(&scattered, depth - 1, world)
                }
                None => Color::new(0.0, 0.0, 0.0),
            },
            None => {
                let a = 0.5_f64 * (ray.direction().normalise().y() + 1.0_f64);

                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}

impl Viewport {
    pub fn new(scale: f64, u: Vec3, v: Vec3, w: Vec3, width: f64, height: f64, img_w: u32, img_h: u32) -> Viewport {
        let u_ = width * u; 
        let v_ = height * -v;

        Viewport {
            u: u_.clone(),
            v: v_.clone(),
            w,
            du: u_ / f64::from(img_w),
            dv: v_ / f64::from(img_h),
        }
    }

    pub fn du(&self) -> &Vec3 {
        &self.du
    }

    pub fn dv(&self) -> &Vec3 {
        &self.dv
    }

    pub fn origin(&self, c: &Camera) -> Point3 {
        let upper_left =
            &c.position - (c.focal_length() * &self.w) - (&self.u / 2.0) - (&self.v / 2.0);

        upper_left + (self.du() + self.dv()) / 2.0
    }
}
