use crate::math::math3::{Vec3};
use crate::math::raytracing::{Hittable, HittableList};
use crate::math::hittables::{Sphere};
use crate::math::materials::{Material};


pub fn three_spheres_scene() -> Box<dyn Hittable> {
    Box::new(HittableList {
        hittables: vec![
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0,
                                 Material::Lambertian(Vec3::new(0.1, 0.8, 0.4)))),
            Box::new(Sphere::new(-Vec3::z_axis(), 0.5,
                                 Material::Lambertian(Vec3::new(0.5, 0.4, 0.7)))),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5,
                                 Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.7))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5,
                                 Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.2)))
        ]
    })
}
