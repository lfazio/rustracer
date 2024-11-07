mod camera;
mod interval;
mod objects;
mod ppm;
mod ray;
mod types;

use camera::Camera;
use objects::{sphere, HittableList};
use types::{Color, Point3};

fn main() {
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), 1.0, 16_f64 / 9_f64, 640);

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

    camera.render(&world);
}
