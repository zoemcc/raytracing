use std::cmp::Ordering::{Equal, Less, Greater};
use crate::math::math3::{Vec3, dot};
use crate::math::materials::{Material};
use crate::math::raytracing::{Ray, HitRecord, Hittable, face_normal_adjustment};
use crate::math::signed_distance::{SignedDistance};


pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittables: Vec::new()
        }
    }

    pub fn add(&mut self, to_add: Box<dyn Hittable>) -> () {
        self.hittables.push(to_add);
    }

    pub fn clear(&mut self) -> () {
        self.hittables.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(min_hit_opt) = self.hittables.iter()
            .map(|x| x.hit(ray, t_min, t_max))
            .min_by(|x, y| {
                match (x, y) {
                    (Some(x_hit_record), Some(y_hit_record)) => {
                        if x_hit_record.t < y_hit_record.t {Less} else {Greater}
                    },
                    (Some(_), None) => Less,
                    (None, Some(_)) => Greater,
                    (None, None) => Equal
                }
            })
        { min_hit_opt } else { None }
    }
}

pub struct Raymarcher {
    distance_field: Box<dyn SignedDistance>,
    max_march_steps: usize,
    min_distance: f64,
    material: Box<dyn Material>,
}

impl Raymarcher {
    pub fn new(distance_field: Box<dyn SignedDistance>, max_march_steps: usize,
               min_distance: f64, material: Box<dyn Material>) -> Raymarcher {
        Raymarcher {
            distance_field,
            max_march_steps,
            min_distance,
            material
        }
    }
}

impl Hittable for Raymarcher {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_cur = t_min;
        for step in 0..self.max_march_steps {
            let cur_point = ray.at(t_cur);
            let cur_distance = self.distance_field.distance_estimate(cur_point);
            if cur_distance < self.min_distance {
                let normal: Vec3 = self.distance_field.normal_estimate(cur_point);
                //let normal: Vec3 = -ray.dir;
                return Some(HitRecord::new(cur_point, normal, &self.material, t_cur, true))
            }
            else {
                t_cur += cur_distance;
                if t_cur > t_max {return None}
            }
        }

        None
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = (*ray).origin - self.center;
        let a = (*ray).dir.length_squared();
        let half_b = dot(oc, (*ray).dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for temp in [(-half_b - root) / a, (-half_b + root) / a].iter() {
                let t = *temp;
                if t < t_max && t > t_min {
                    let point = ray.at(t);
                    let outward_normal = (point - self.center) / self.radius;
                    let (normal, front_face) =
                        face_normal_adjustment(ray.dir, outward_normal);
                    return Some(HitRecord::new(point, normal, &self.material, t, front_face));
                }
            }
        }
        None
    }
}


