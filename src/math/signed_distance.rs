use crate::math::math3::{Vec3, dot};
use crate::math::materials::{Material};
use crate::math::raytracing::{Ray, HitRecord, Hittable, face_normal_adjustment};
use crate::random_vec_in_unit_sphere;

pub trait SignedDistance {
    fn distance_estimate(&self, point: Vec3) -> f64;
    fn normal_estimate(&self, point: Vec3) -> Vec3;
}

pub struct SierpinskiTetrahedron {
    center: Vec3,
    num_fractal_iterations: usize,
}

impl SierpinskiTetrahedron {
    pub fn new(center: Vec3, num_fractal_iterations: usize) -> SierpinskiTetrahedron {
        SierpinskiTetrahedron {
            center,
            num_fractal_iterations,
        }
    }
}

impl SignedDistance for SierpinskiTetrahedron {
    fn distance_estimate(&self, point: Vec3) -> f64 {
        let mut cur_vec = point - self.center;
        let offset = Vec3::one();
        for n in 0..self.num_fractal_iterations {
            if cur_vec.x() + cur_vec.y() < 0.0 {cur_vec = Vec3::new(-cur_vec.y(), -cur_vec.x(), cur_vec.z());}
            if cur_vec.x() + cur_vec.z() < 0.0 {cur_vec = Vec3::new(-cur_vec.z(), cur_vec.y(), -cur_vec.x());}
            if cur_vec.y() + cur_vec.z() < 0.0 {cur_vec = Vec3::new(cur_vec.x(), -cur_vec.z(), -cur_vec.y());}
            cur_vec = cur_vec * 2.0 - offset;
        }
        let distance = cur_vec.length() * 2.0_f64.powi(-(self.num_fractal_iterations as i32));
        //println!("final distance: {}", distance);
        distance
    }

    fn normal_estimate(&self, point: Vec3) -> Vec3 {
        Vec3::z_axis()
    }
}


pub struct SphereSignedDistance {
    center: Vec3,
    radius: f64,
}

impl SphereSignedDistance {
    pub fn new(center: Vec3, radius: f64) -> SphereSignedDistance {
        SphereSignedDistance {
            center,
            radius,
        }
    }
}

impl SignedDistance for SphereSignedDistance {
    fn distance_estimate(&self, point: Vec3) -> f64 {
        let dif_to_center = point - self.center;
        dif_to_center.length() - self.radius
    }

    fn normal_estimate(&self, point: Vec3) -> Vec3 {
        let dif_to_center = point - self.center;
        dif_to_center.unit_vector()
    }
}

