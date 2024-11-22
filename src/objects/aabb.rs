use crate::interval::Interval;
use crate::objects::point3::Point3;
use crate::ray::Ray;

#[derive(Debug, Clone, Default)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb {
            x,
            y,
            z,
        }
    }

    pub fn from(a: &Point3, b: &Point3) -> Aabb {
        Aabb {
            x: if a.x() <= b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) },
            y: if a.y() <= b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) },
            z: if a.z() <= b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) },
        }
    }

    pub fn new_enclosing(box1: &Aabb, box2: &Aabb) -> Aabb {
        Aabb {
            x: Interval::new_enclosing(&box1.x, &box2.x),
            y: Interval::new_enclosing(&box1.y, &box2.y),
            z: Interval::new_enclosing(&box1.z, &box2.z),
        }
    }

    fn get(&self, n: usize) -> Option<&Interval> {
        match n {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None,
        }
    }

    pub fn hit(&self, ray: &Ray, rayt: &Interval) -> Option<Interval> {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();
        let mut rt = rayt.clone();

        for axis in 0..2 {
            let ax = self.get(axis)?;
            let adinv = 1.0 / match axis { 0 => ray_direction.x(), 1 => ray_direction.y(), 2 => ray_direction.z(), _ => return None, };
            
            let ray_origin_axis = match axis { 0 => ray_origin.x(), 1 => ray_origin.y(), 2 => ray_origin.z(), _ => return None, };

            let t0 = (ax.min() - ray_origin_axis) * adinv;
            let t1 = (ax.max() - ray_origin_axis) * adinv;

            if t0 < t1 {
                let min = if t0 > rt.min() {
                    t0
                } else { 
                    rt.min()
                };
                let max = if t1 < rt.max() {
                    t1
                } else { 
                    rt.max()
                };

                rt.set_min(min);
                rt.set_max(max);
            } else {
                let min = if t1 > rt.min() {
                    t1
                } else { 
                    rt.min()
                };
                let max = if t0 < rt.max() {
                    t0
                } else { 
                    rt.max()
                };

                if max <= min {
                    return None;
                }

                rt.set_min(min);
                rt.set_max(max);
            }
        }

        Some(rt)
    }
}
