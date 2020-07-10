use crate::math::math3::{Vec3};
use crate::math::raytracing::{Hittable};
use crate::math::materials::{Material};


pub fn spherion_scene() -> Hittable {
    Hittable::HittableList (
        vec![
            Hittable::Sphere(Vec3::new(0.0, -100.5, -1.0), 100.0,
                                 Material::Lambertian(Vec3::new(0.1, 0.8, 0.4))),
            Hittable::Sphere(Vec3::new(0.0, -0.1, -1.0), 0.4,
                                 Material::Lambertian(Vec3::new(0.5, 0.4, 0.7))),
            Hittable::Sphere(Vec3::new(0.5, 0.15, -1.0), 0.2,
                                 Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.05)),
            Hittable::Sphere(Vec3::new(-0.5, 0.15, -1.0), 0.2,
                                 Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.05)),
            Hittable::Sphere(Vec3::new(0.125, 0.05, -0.75), 0.15,
                                 Material::Metal(Vec3::new(0.5, 0.9, 0.5), 0.01)),
            Hittable::Sphere(Vec3::new(-0.125, 0.05, -0.75), 0.15,
                                 Material::Metal(Vec3::new(0.5, 0.9, 0.5), 0.01)),
            Hittable::Sphere(Vec3::new(0.0, -0.05, -0.7), 0.1,
                                 Material::Lambertian(Vec3::new(0.8, 0.2, 0.2))),
            Hittable::Sphere(Vec3::new(0.0, 0.45, 0.75), 0.5,
                                 Material::Metal(Vec3::new(0.2, 0.2, 0.2), 0.01)),
            Hittable::Sphere(Vec3::new(0.0, 0.45, 0.335), 0.175,
                                 Material::Lambertian(Vec3::new(0.95, 0.95, 0.95))),
        ]
    )
}
