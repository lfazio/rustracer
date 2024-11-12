extern crate fastrand;

use crate::{
    interval::Interval,
    objects::{Hittable, HittableList},
    ppm::image::Ppm,
    ray::Ray,
    rng,
    vec3::{Color, Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct Camera {
    position: Point3,         // Point camera is looking from
    lookat: Point3,           // Point camera is looking at
    vup: Vec3,                // Camera-relative "up" direction
    aspect_ratio: f64,        // Ratio of image width over height
    img_w: u32,               // Rendered image width in pixel count
    img_h: u32,               // Rendered image height in pixel count
    samples_per_pixel: usize, // Count of random samples for each pixel
    max_depth: usize,
    vfov: f64,            // Vertical view angle (field of view)
    focus_dist: f64,      // Distance from camera lookfrom point to plane of perfect focus
    defocus_angle: f64,   // Variation angle of rays through each pixel
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius

    viewport: Viewport,
}

#[derive(Debug, Default)]
pub struct Viewport {
    origin: Point3,
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
    pub fn set_antialiasing(&mut self, sample_per_pixel: usize) -> &mut Self {
        self.samples_per_pixel = sample_per_pixel;

        self
    }

    pub fn set_maximum_depth(&mut self, max_depth: usize) -> &mut Self {
        self.max_depth = max_depth;

        self
    }

    pub fn new(
        position: Point3,
        lookat: Point3,
        vup: Vec3,
        aspect_ratio: f64,
        img_w: u32,
        vfov: f64,
        focus_dist: f64,
        defocus_angle: f64,
    ) -> Camera {
        let img_h = if (f64::from(img_w) / aspect_ratio) < 1_f64 {
            1
        } else {
            (f64::from(img_w) / aspect_ratio) as u32
        };

        // Viewport dimensions
        let theta = f64::to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let vh = 2.0 * h * focus_dist;
        let vw = vh * aspect_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (&position - &lookat).normalise();
        let u = vup.cross(&w).normalise();
        let v = w.cross(&u);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * f64::tan(f64::to_radians(defocus_angle / 4.0));
        let defocus_disk_u = &u * defocus_radius;
        let defocus_disk_v = &v * defocus_radius;

        Camera {
            position: position.clone(),
            lookat,
            vup,
            aspect_ratio,
            img_w,
            img_h,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov,
            viewport: Viewport::new(&position, focus_dist, &u, &v, &w, (vw, vh), (img_w, img_h)),
            focus_dist,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let mut img = Ppm::new(self.img_w, self.img_h, 256);

        eprintln!("Rendering...");
        for j in 0..self.img_h {
            for i in 0..self.img_w {
                eprint!(
                    "\rLine {}/{} - {:02.2}%",
                    j,
                    self.img_h,
                    100.0 * f64::from(i) / f64::from(self.img_w)
                );
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    color += self.ray_color(&self.get_ray(i, j), self.max_depth, world);
                }
                color /= f64::from(self.samples_per_pixel as u32);

                img.set(
                    i,
                    j,
                    color_get_level(color.x()),
                    color_get_level(color.y()),
                    color_get_level(color.z()),
                );
            }
        }

        eprintln!("\x33[2K\rDone!");
        println!("{}", img);
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rng::random_range(-0.5, 0.5),
            rng::random_range(-0.5, 0.5),
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        &self.position + (p.x() * &self.defocus_disk_u) + (p.y() * &self.defocus_disk_v)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.viewport.origin()
            + ((f64::from(i) + offset.x()) * self.viewport.du())
            + ((f64::from(j) + offset.y()) * self.viewport.dv());

        let ray_origin: Point3 = if self.defocus_angle <= 0.0 {
            self.position.clone()
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(ray_origin.clone(), &pixel_sample - &ray_origin)
    }

    fn ray_color(&self, ray: &Ray, depth: usize, world: &HittableList) -> Color {
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
    pub fn new(
        camera_position: &Point3,
        camera_focus_dist: f64,
        u: &Vec3,
        v: &Vec3,
        w: &Vec3,
        vsize: (f64, f64),
        imgsize: (u32, u32),
    ) -> Viewport {
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let vu = vsize.0 * u; // Vector across viewport horizontal edge
        let vv = vsize.1 * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors to the next pixel.
        let du = &vu / f64::from(imgsize.0);
        let dv = &vv / f64::from(imgsize.1);

        // Calculate the location of the upper left pixel.
        let upper_left = camera_position - (camera_focus_dist * w) - 0.5 * (&vu + &vv);

        Viewport {
            origin: upper_left + 0.5 * (&du + &dv),
            du,
            dv,
        }
    }

    pub fn du(&self) -> &Vec3 {
        &self.du
    }

    pub fn dv(&self) -> &Vec3 {
        &self.dv
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
}
