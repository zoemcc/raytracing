extern crate image;

use std::ops::{Neg, Add, Sub, Mul, Div};
use std::cmp::Ordering::{Equal, Less, Greater};
use rand::Rng;
use rand::seq::index::sample;


#[derive(Debug, Copy, Clone)]
struct Vec3 {
    e: [f64;3]
}

#[allow(dead_code)]
impl Vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn one() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    fn x_axis() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    fn y_axis() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    fn z_axis() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    fn x(&self) -> f64 {
        self.e[0]
    }

    fn y(&self) -> f64 {
        self.e[1]
    }

    fn z(&self) -> f64 {
        self.e[2]
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        dot(*self, *self)
    }

    fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    fn to_string(&self) -> String {
        format!("{} {} {}", self.e[0], self.e[1], self.e[2])
    }

    fn print_string(&self) -> () {
        println!("{}", self.to_string());
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self + (-other)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::zero();
        let horizontal = viewport_width * Vec3::x_axis();
        let vertical = viewport_height * Vec3::y_axis();

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - focal_length * Vec3::z_axis()
                - horizontal / 2.0 - vertical / 2.0
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner - self.origin
            + u * self.horizontal + v * self.vertical)
    }
}

struct Ray {
    origin: Vec3,
    dir: Vec3
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            dir: direction
        }
    }

    fn at(&self, t: f64) -> Vec3 {
        self.origin + (t * self.dir)
    }
}

struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool
}

impl HitRecord {
    fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            point,
            normal,
            t,
            front_face
        }
    }
}

fn face_normal_adjustment(ray_direction: Vec3, outward_normal: Vec3) -> (Vec3, bool) {
    let front_face: bool = dot(ray_direction, outward_normal) < 0.0;
    let normal = if front_face {outward_normal} else {-1.0 * outward_normal};
    (normal, front_face)
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct HittableList {
    hittables: Vec<Box<dyn Hittable>>
}

impl HittableList {
    fn new() -> HittableList {
        HittableList {
            hittables: Vec::new()
        }
    }

    fn add(&mut self, to_add: Box<dyn Hittable>) -> () {
        self.hittables.push(to_add);
    }

    fn clear(&mut self) -> () {
        self.hittables.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(min_hit_opt) = self.hittables.iter()
            .map(|x| x.hit(ray, t_min, t_max))
            .min_by(|x, y| {
                match (x, y) {
                    (Some(x_hit_record), Some(y_hit_record)) => {
                        if x_hit_record.t < y_hit_record.t {Less} else {Greater}
                    },
                    (Some(_), None) => Less,
                    (None, Some(_)) => Greater,
                    (None, None) => Equal
                }
            })
        { min_hit_opt } else { None }
    }
}

struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
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
                    return Some(HitRecord::new(point, normal, t, front_face));
                }
            }
        }
        None
    }
}

fn to_color(pixel_color: Vec3, samples_per_pixel: i32) -> image::Rgb<u8> {
    let scaled_pixel_color = pixel_color / (samples_per_pixel as f64);
    let r = (256.0 * clamp(scaled_pixel_color.x(), 0.0, 0.999)).floor() as u8;
    let g = (256.0 * clamp(scaled_pixel_color.y(), 0.0, 0.999)).floor() as u8;
    let b = (256.0 * clamp(scaled_pixel_color.z(), 0.0, 0.999)).floor() as u8;

    image::Rgb([r, g, b])
}

fn to_color_single(pixel_color: Vec3) -> image::Rgb<u8> {
    let r = (255.999 * pixel_color.x()).floor() as u8;
    let g = (255.999 * pixel_color.y()).floor() as u8;
    let b = (255.999 * pixel_color.z()).floor() as u8;

    image::Rgb([r, g, b])
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {min} else if x > max {max} else {x}
}

fn ray_color(ray: Ray, hittable: &Box<dyn Hittable>) -> Vec3 {
    if let Some(hit_record) = (*hittable).hit(&ray, 0.0, f64::INFINITY) {
        0.5 * (hit_record.normal + Vec3::one())
    }
    else {
        let unit_ray_dir = ray.dir.unit_vector();
        let t = 0.5 * (unit_ray_dir.y() + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}


fn main() -> std::io::Result<()> {
    println!("Configuring viewport and image buffer.");

    let mut rng = rand::thread_rng();

    let aspect_ratio = 16.0 / 9.0;

    let print_every_n_rows: u32 = 20;
    let image_width: u32 = 3840;
    let image_height: u32 = (image_width as f64 / aspect_ratio).floor() as u32;
    let samples_per_pixel = 100;

    println!("Image width: {}, Image Height: {}, Samples Per Pixel: {}, Status print every {} rows",
             image_width, image_height, samples_per_pixel, print_every_n_rows);

    let world: Box<dyn Hittable> = Box::new(HittableList {
        hittables: vec![
            Box::new(Sphere::new(-Vec3::z_axis(), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
        ]
    });

    let cam: Camera = Camera::new();

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    println!("Starting to render image.");

    for (index, (x, y, pixel)) in imgbuf.enumerate_pixels_mut().enumerate() {
        if index as u32 % (print_every_n_rows * image_width) == 0 {
            println!("Pixel (x, y): ({}, {}), Rows remaining: {}", x, y, image_height - y);
        }
        let i = x as f64;
        let j = ((image_height - 1) - y) as f64;

        let pixel_color: Vec3 = (0..samples_per_pixel).map(|_| {
            let u = (i + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f64;
            let v = (j + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f64;
            let ray = cam.get_ray(u, v);
            ray_color(ray, &world)
        }).fold(Vec3::zero(), |x, y| x + y);

        *pixel = to_color(pixel_color, samples_per_pixel);
    }

    println!("Finished rendering image.");

    imgbuf.save("./output/lorgest_sphere_sphere.png").unwrap();

    println!("Finished saving image.");

    Ok(())
}




/* TODO: old testing stuff, move this to vec trait testing
let x_vec = Vec3::new(1.0, 0.5, 0.25);
x_vec.print_string();
let y_vec = Vec3::new(-1.0, 0.75, -0.75);
y_vec.print_string();
let neg_x_vec = -x_vec;
neg_x_vec.print_string();
let z_vec = x_vec + y_vec;
z_vec.print_string();
let w_vec = x_vec * y_vec;
w_vec.print_string();
let x_scal_vec = x_vec * (0.5 as f64);
x_scal_vec.print_string();
let scal_x_vec = (0.5 as f64) * x_vec ;
scal_x_vec.print_string();
let sub_vec = x_vec - y_vec;
sub_vec.print_string();
let x_div = x_vec / (0.5 as f64);
x_div.print_string();
let x_unit = x_vec.unit_vector();
x_unit.print_string();
println!("{}", x_unit.length());
*/
