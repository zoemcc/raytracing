use crate::math::math3::{Vec3};
use crate::math::raytracing::{Hittable};
use crate::math::signed_distance::{SignedDistanceField};
use crate::math::materials::{Material};


pub fn first_fractal_scene() -> Hittable {
    Hittable::HittableList (
        vec![
            Hittable::Sphere(Vec3::new(0.0, -100.5, -1.0), 100.0,
                                 Material::Metal(Vec3::new(0.1, 0.8, 0.4), 0.01)),
            Hittable::Raymarcher(SignedDistanceField::
                SierpinskiTetrasphere(Vec3::new(0.0, 0.52, -0.0), 8),
                                     100, 0.000005, Material::Lambertian(Vec3::new(0.5, 0.4, 0.7))),
            Hittable::Sphere(Vec3::new(0.0, 0.52, 0.0), 0.4,
                             Material::Metal(Vec3::new(0.9, 0.2, 0.8), 0.01)),
]
)
}
