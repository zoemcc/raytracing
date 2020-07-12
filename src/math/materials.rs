use rand::prelude::ThreadRng;

use crate::math::math3::{Vec3, random_unit_vector, reflect, dot, random_vec_in_unit_sphere};
use crate::math::raytracing::{Ray, HitRecord};


pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Absorb,
}

impl Material {
    pub fn scatter(&self, rng_source: &mut ThreadRng, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Metal(albedo, fuzz) => {
                let reflected = reflect(ray.dir.unit_vector(), hit_record.normal);
                let scattered = Ray::new(hit_record.point, reflected + (*fuzz) * random_vec_in_unit_sphere(rng_source));
                if dot(scattered.dir, hit_record.normal) > 0.0 {
                    Some((scattered, (*albedo)))
                }
                else {
                    None
                }
            },

            Material::Lambertian(albedo) => {
                let scatter_direction: Vec3 = hit_record.normal + random_unit_vector(rng_source);
                Some((Ray::new(hit_record.point, scatter_direction), (*albedo)))
            },

            Material::Absorb => {
                None
            }
        }
    }
}
