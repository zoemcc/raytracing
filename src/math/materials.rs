use rand::Rng;
use rand::prelude::ThreadRng;

use crate::math::math3::{Vec3, random_unit_vector, reflect, dot, random_vec_in_unit_sphere};
use crate::math::raytracing::{Ray, HitRecord};

pub trait Material {
    fn scatter(&self, rng_source: &mut ThreadRng, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rng_source: &mut ThreadRng, _: &Ray, hit_record: HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_direction: Vec3 = hit_record.normal + random_unit_vector(rng_source);
        Some((Ray::new(hit_record.point, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 {fuzz} else {1.0}
        }
    }
}

impl Material for Metal {
    fn scatter(&self, rng_source: &mut ThreadRng, ray: &Ray, hit_record: HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray.dir.unit_vector(), hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected + self.fuzz * random_vec_in_unit_sphere(rng_source));
        if dot(scattered.dir, hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        }
        else {
            None
        }
    }
}

