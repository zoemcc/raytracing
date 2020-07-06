use crate::math::math3::{Vec3};
use crate::math::raytracing::{Hittable};
use crate::math::hittables::{Sphere, HittableList, Raymarcher};
use crate::math::signed_distance::{SierpinskiTetrahedron, SphereSignedDistance};
use crate::math::materials::{Lambertian, Metal, AbsorbRay};


pub fn first_fractal_scene() -> Box<dyn Hittable> {
    Box::new(HittableList {
        hittables: vec![
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0,
                                 Box::new(Lambertian::new(Vec3::new(0.1, 0.8, 0.4))))),
            /*
            Box::new(Raymarcher::new(Box::new(
                SierpinskiTetrahedron::new(Vec3::new(0.0, 0.0, -1.5), 1)),
                                     10, 0.2, Box::new(AbsorbRay::new(Vec3::new(0.5, 0.4, 0.7))))),
             */
            Box::new(Raymarcher::new(Box::new(SphereSignedDistance::new(
                Vec3::new(0.0, -0.1, -1.0), 0.4
            )), 100, 0.005, Box::new(Lambertian::new(Vec3::new(0.5, 0.4, 0.7))))),
            //Box::new(Sphere::new(Vec3::new(0.0, -0.1, -1.0), 0.4,
                                 //Box::new(Lambertian::new(Vec3::new(0.5, 0.4, 0.7))))),
        ]
    })
}
