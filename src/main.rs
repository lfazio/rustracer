mod camera;
mod interval;
mod objects;
mod ppm;
mod ray;
mod types;

use camera::{Camera, Viewport};
use interval::Interval;
use objects::{sphere, HitRecord, Hittable, HittableList};
use ppm::image::Ppm;
use ray::Ray;
use types::{Color, Point3};

fn f64_convert(x: f64) -> u32 {
    x.round().rem_euclid(2f64.powi(32)) as u32
}

fn color_get_level(v: f64) -> u8 {
    f64_convert(v * 255_f64) as u8
}

fn main() {
    let aspect_ratio = 16_f64 / 9_f64;
    let img_w = 400_usize;

    let img_h = if f64::from(img_w as u32) / aspect_ratio < 1_f64 {
        1_usize
    } else {
        f64_convert(f64::from(img_w as u32) / aspect_ratio) as usize
    };
    let mut img = Ppm::new(img_w, img_h, 256);

    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), 1.0);
    let viewport = Viewport::new(2.0, img_w as u32, img_h as u32);

    let mut world = HittableList::new();
    let binding = sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5_f64,
        Color::new(1.0, 0.0, 0.0),
    );
    world.add(&binding);

    let binding = sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100_f64,
        Color::new(0.0, 1.0, 0.0),
    );
    world.add(&binding);

    for j in 0..img_h {
        for i in 0..img_w {
            let pixel = viewport.origin(&camera)
                + (f64::from(i as u32) * viewport.du())
                + (f64::from(j as u32) * viewport.dv());
            let ray_direction = pixel.clone() - camera.position();
            let ray = Ray::new(pixel, ray_direction);
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

            img.set(
                i,
                j,
                color_get_level(r),
                color_get_level(g),
                color_get_level(b),
            );
        }
    }
    println!("{}", img);
}
