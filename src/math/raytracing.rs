use std::cmp::Ordering::{Equal, Less, Greater};

use crate::math::math3::{Vec3, dot};
use crate::math::materials::{Material};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            dir: direction.unit_vector()
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (t * self.dir)
    }
}

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
    t: f64,
    front_face: bool
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Vec3, normal: Vec3, material: &Material, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face
        }
    }
}

pub fn face_normal_adjustment(ray_direction: Vec3, outward_normal: Vec3) -> (Vec3, bool) {
    let front_face: bool = dot(ray_direction, outward_normal) < 0.0;
    let normal = if front_face {outward_normal} else {-1.0 * outward_normal};
    (normal, front_face)
}


pub enum Hittable {
    HittableList(Vec<Hittable>),
    Sphere(Vec3, f64, Material),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::HittableList(hittables) => {
                if let Some(min_hit_opt) = hittables.iter()
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
            },

            Hittable::Sphere(center, radius, mat) => {
                let oc = (*ray).origin - (*center);
                let a = (*ray).dir.length_squared();
                let half_b = dot(oc, (*ray).dir);
                let c = oc.length_squared() - (*radius) * (*radius);
                let discriminant = half_b * half_b - a * c;

                if discriminant > 0.0 {
                    let root = discriminant.sqrt();
                    for temp in [(-half_b - root) / a, (-half_b + root) / a].iter() {
                        let t = *temp;
                        if t < t_max && t > t_min {
                            let point = ray.at(t);
                            let outward_normal = (point - (*center)) / (*radius);
                            let (normal, front_face) =
                                face_normal_adjustment(ray.dir, outward_normal);
                            return Some(HitRecord::new(point, normal, &mat, t, front_face));
                        }
                    }
                }
                None
            }
        }
    }
}
