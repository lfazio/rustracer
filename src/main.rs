mod camera;
mod interval;
mod material;
mod objects;
mod ppm;
mod ray;
mod rng;
mod vec3;

use std::rc::Rc;

use camera::Camera;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use objects::{sphere, HittableList};
use vec3::{Color, Point3, Vec3};

fn main() {
    let mut camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        400,
        20.0,
        3.4,
        10.0,
    );
    camera.set_antialiasing(100).set_maximum_depth(50);

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let binding = sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100_f64, material_ground);
    world.add(&binding);

    let binding = sphere::Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5_f64, material_center);
    world.add(&binding);

    let binding = sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5_f64, material_left);
    world.add(&binding);

    let binding = sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4_f64, material_bubble);
    world.add(&binding);

    let binding = sphere::Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5_f64, material_right);
    world.add(&binding);

    camera.render(&world);
}
