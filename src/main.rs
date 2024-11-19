mod camera;
mod interval;
mod material;
mod objects;
mod ppm;
mod ray;
mod rng;

use std::rc::Rc;
use std::vec::Vec;

use camera::Camera;
use material::{color::Color, dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use objects::point3::Point3;
use objects::vector3::Vector3;
use objects::{sphere, Hittable, HittableList};

fn main() {
    let mut camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::default(),
        Vector3::new(0.0, 1.0, 0.0),
        16.0 / 9.0,
        1200,
        20.0,
        10.0,
        0.6,
    );
    camera.set_antialiasing(500).set_maximum_depth(50);

    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let binding = sphere::Sphere::new(Point3::new(0.0, -1000.0, -1.0), 1000.0, ground_material);
    world.add(&binding);

    let p = Point3::new(4.0, 0.2, 0.0);
    let mut sphere = Vec::<sphere::Sphere>::new();
    for a in 0..22_usize {
        for b in 0..22_usize {
            let choose_mat = rng::random();
            let center = Point3::new(
                f64::from(a as i32 - 11) + 0.9 * rng::random(),
                0.2,
                f64::from(b as i32 - 11) + 0.9 * rng::random(),
            );
            let motion: Option<Point3>;

            if Vector3::from(&center - &p).norm() > 0.9 {
                let sphere_material: Rc<dyn material::Material> = match choose_mat {
                    0.0..0.8 => {
                        // diffuse
                        let albedo = Color::new(
                            rng::random() * rng::random(),
                            rng::random() * rng::random(),
                            rng::random() * rng::random(),
                        );
                        motion =
                            Some(&center + Vector3::new(0.0, rng::random_range(0.0, 0.5), 0.0));
                        Rc::new(Lambertian::new(albedo))
                    }
                    0.8..0.95 => {
                        // metal
                        let albedo = Color::new(
                            rng::random_range(0.5, 1.0),
                            rng::random_range(0.5, 1.0),
                            rng::random_range(0.5, 1.0),
                        );
                        let fuzz = rng::random_range(0.0, 0.5);
                        motion = None;
                        Rc::new(Metal::new(albedo, fuzz))
                    }
                    _ => {
                        // glass
                        motion = None;
                        Rc::new(Dielectric::new(1.50))
                    }
                };

                let binding = match motion {
                    Some(c2) => sphere::Sphere::with_motion(center, c2, 0.2, sphere_material),
                    None => sphere::Sphere::new(center, 0.2, sphere_material),
                };
                sphere.push(binding);
            }
        }
    }

    for s in sphere.iter() {
        world.add(s as &dyn Hittable);
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    let binding = sphere::Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1);
    world.add(&binding);

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let binding = sphere::Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2);
    world.add(&binding);

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let binding = sphere::Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3);
    world.add(&binding);

    camera.render(&world);
}
