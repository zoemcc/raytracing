use crate::math::math3::{Vec3};
use crate::math::raytracing::{Hittable, HittableList};
use crate::math::hittables::{Sphere};
use crate::math::materials::{Lambertian, Metal};


pub fn spherion_scene() -> Box<dyn Hittable> {
    Box::new(HittableList {
        hittables: vec![
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0,
                                 Box::new(Lambertian::new(Vec3::new(0.1, 0.8, 0.4))))),
            Box::new(Sphere::new(Vec3::new(0.0, -0.1, -1.0), 0.4,
                                 Box::new(Lambertian::new(Vec3::new(0.5, 0.4, 0.7))))),
            Box::new(Sphere::new(Vec3::new(0.5, 0.15, -1.0), 0.2,
                                 Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.05)))),
            Box::new(Sphere::new(Vec3::new(-0.5, 0.15, -1.0), 0.2,
                                 Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.05)))),
            Box::new(Sphere::new(Vec3::new(0.125, 0.05, -0.75), 0.15,
                                 Box::new(Metal::new(Vec3::new(0.5, 0.9, 0.5), 0.01)))),
            Box::new(Sphere::new(Vec3::new(-0.125, 0.05, -0.75), 0.15,
                                 Box::new(Metal::new(Vec3::new(0.5, 0.9, 0.5), 0.01)))),
            Box::new(Sphere::new(Vec3::new(0.0, -0.05, -0.7), 0.1,
                                 Box::new(Lambertian::new(Vec3::new(0.8, 0.2, 0.2))))),
            Box::new(Sphere::new(Vec3::new(0.0, 0.45, 0.75), 0.5,
                                 Box::new(Metal::new(Vec3::new(0.2, 0.2, 0.2), 0.01)))),
            Box::new(Sphere::new(Vec3::new(0.0, 0.45, 0.335), 0.175,
                                 Box::new(Lambertian::new(Vec3::new(0.95, 0.95, 0.95))))),
        ]
    })
}
