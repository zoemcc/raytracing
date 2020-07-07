use crate::math::math3::{Vec3, dot};
use crate::math::materials::{Material};
use crate::math::raytracing::{Ray, HitRecord, Hittable, face_normal_adjustment};


pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = (*ray).origin - self.center;
        let a = (*ray).dir.length_squared();
        let half_b = dot(oc, (*ray).dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for temp in [(-half_b - root) / a, (-half_b + root) / a].iter() {
                let t = *temp;
                if t < t_max && t > t_min {
                    let point = ray.at(t);
                    let outward_normal = (point - self.center) / self.radius;
                    let (normal, front_face) =
                        face_normal_adjustment(ray.dir, outward_normal);
                    return Some(HitRecord::new(point, normal, &self.material, t, front_face));
                }
            }
        }
        None
    }
}

