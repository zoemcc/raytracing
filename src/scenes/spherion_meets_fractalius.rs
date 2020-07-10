use crate::math::math3::{Vec3};
use crate::math::raytracing::{Hittable};
use crate::math::signed_distance::{SignedDistanceField};
use crate::math::materials::{Material};


pub fn spherion_meets_fractalius_scene() -> Hittable {
    Hittable::HittableList (
        vec![
            Hittable::Sphere(Vec3::new(0.0, -100.5, -1.0), 100.0,
                             Material::Metal(Vec3::new(0.1, 0.8, 0.4), 0.2)),

            // fractalius
            Hittable::Raymarcher(SignedDistanceField::
                                 SierpinskiTetrasphere(Vec3::new(0.0, 0.52, 0.75), 8),
                                 100, 0.000005, Material::Lambertian(Vec3::new(0.5, 0.4, 0.7))),
            Hittable::Sphere(Vec3::new(0.0, 0.52, 0.75), 0.4,
                             Material::Metal(Vec3::new(0.9, 0.2, 0.8), 0.01)),

            //spherion
            //body
            Hittable::Sphere(Vec3::new(0.0, -0.1, -2.0), 0.4,
                             Material::Lambertian(Vec3::new(0.5, 0.4, 0.7))),
            Hittable::Sphere(Vec3::new(0.5, 0.15, -2.0), 0.2,
                             Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.05)),
            Hittable::Sphere(Vec3::new(-0.5, 0.15, -2.0), 0.2,
                             Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.05)),
            Hittable::Sphere(Vec3::new(0.125, 0.05, -1.75), 0.15,
                             Material::Metal(Vec3::new(0.5, 0.9, 0.5), 0.01)),
            Hittable::Sphere(Vec3::new(-0.125, 0.05, -1.75), 0.15,
                             Material::Metal(Vec3::new(0.5, 0.9, 0.5), 0.01)),
            Hittable::Sphere(Vec3::new(0.0, -0.05, -1.7), 0.1,
                             Material::Lambertian(Vec3::new(0.8, 0.2, 0.2))),

            //pupil
            //Hittable::Sphere(Vec3::new(0.0, 0.45, 0.75), 0.5,
                             //Material::Metal(Vec3::new(0.2, 0.2, 0.2), 0.01)),
            Hittable::Sphere(Vec3::new(0.0, 0.52, 0.435), 0.175,
                             Material::Lambertian(Vec3::new(0.95, 0.95, 0.95))),
        ]
    )
}
