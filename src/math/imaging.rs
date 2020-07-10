use rand::Rng;
use rand::prelude::ThreadRng;

use crate::math::math3::{Vec3, cross};
use crate::math::raytracing::{Ray, Hittable};
use crate::math::utils::{degrees_to_radians};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = cross(vup, w).unit_vector();
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - w - horizontal / 2.0 - vertical / 2.0
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner - self.origin
            + u * self.horizontal + v * self.vertical)
    }
}


pub fn to_color(pixel_color: Vec3, samples_per_pixel: i32) -> image::Rgb<u8> {
    let pixel_color_scaled_gamma_corrected = (pixel_color / (samples_per_pixel as f64)).sqrt();

    let r = (256.0 * clamp(pixel_color_scaled_gamma_corrected.x(), 0.0, 0.999)).floor() as u8;
    let g = (256.0 * clamp(pixel_color_scaled_gamma_corrected.y(), 0.0, 0.999)).floor() as u8;
    let b = (256.0 * clamp(pixel_color_scaled_gamma_corrected.z(), 0.0, 0.999)).floor() as u8;

    image::Rgb([r, g, b])
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {min} else if x > max {max} else {x}
}

pub fn ray_color(rng_source: &mut ThreadRng, ray: Ray, hittable: &Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3::zero()
    }
    else {
        // TODO: replace this hard coded t_max limit with a better way to terminate raymarching
        // was before f64::INFINITY
        if let Some(hit_record) = (*hittable).hit(&ray, 0.001, 100.0) {
            if let Some((scattered, attenuation)) =
            (*hit_record.material).scatter(rng_source, &ray, hit_record) {
                attenuation * ray_color(rng_source, scattered, hittable, depth - 1)
            }
            else {
                Vec3::zero()
            }
        } else {
            let unit_ray_dir = ray.dir.unit_vector();
            let t = 0.5 * (unit_ray_dir.y() + 1.0);
            (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

