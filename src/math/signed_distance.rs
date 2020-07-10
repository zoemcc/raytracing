use crate::math::math3::{Vec3, dot};
use crate::math::materials::{Material};
use crate::math::raytracing::{Ray, HitRecord, Hittable, face_normal_adjustment};
use crate::random_vec_in_unit_sphere;

pub enum SignedDistanceField {
    Sphere(Vec3, f64),
    SierpinskiTetrasphere(Vec3, usize),
}

impl SignedDistanceField {
    pub fn distance_estimate(&self, point: Vec3) -> f64 {
        match self {
            Self::Sphere(center, radius) => {
                let dif_to_center = point - *center;
                dif_to_center.length() - *radius
            },

            Self::SierpinskiTetrasphere(center, num_fractal_iterations) => {
                let mut cur_vec = point - *center;
                let offset = Vec3::one();
                for n in 0..(*num_fractal_iterations) {
                    if cur_vec.x() + cur_vec.y() < 0.0 {
                        cur_vec = Vec3::new(-cur_vec.y(), -cur_vec.x(), cur_vec.z());
                    }
                    if cur_vec.x() + cur_vec.z() < 0.0 {
                        cur_vec = Vec3::new(-cur_vec.z(), cur_vec.y(), -cur_vec.x());
                    }
                    if cur_vec.y() + cur_vec.z() < 0.0 {
                        cur_vec = Vec3::new(cur_vec.x(), -cur_vec.z(), -cur_vec.y());
                    }
                    cur_vec = cur_vec * 2.0 - offset;
                }
                (cur_vec.length() - 0.9) * 2.0_f64.powi(-(*num_fractal_iterations as i32))
            }
        }
    }

    pub fn normal_estimate(&self, point: Vec3) -> Vec3 {
        match self {
            Self::Sphere(center, radius) => {
                let dif_to_center = point - *center;
                dif_to_center.unit_vector()
            },

            Self::SierpinskiTetrasphere(center, num_fractal_iterations) => {
                // TODO: placeholder! DO NOT USE FOR specular reflections!
                Vec3::z_axis()
            }
        }
    }
}


