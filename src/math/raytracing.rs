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
            dir: direction
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (t * self.dir)
    }
}

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Material>,
    pub t: f64,
    front_face: bool
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Vec3, normal: Vec3, material: &'a Box<dyn Material>, t: f64, front_face: bool) -> HitRecord {
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


