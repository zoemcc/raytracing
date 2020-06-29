extern crate image;

use std::ops::{Neg, Add, Sub, Mul, Div};

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

struct Ray {
    orig: Vec3,
    dir: Vec3
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    fn at(&self, t: f64) -> Vec3 {
        self.orig + (t * self.dir)
    }
}


fn to_color(pixel_color: Vec3) -> image::Rgb<u8> {
    let r = (255.999 * pixel_color.x()).floor() as u8;
    let g = (255.999 * pixel_color.y()).floor() as u8;
    let b = (255.999 * pixel_color.z()).floor() as u8;

    image::Rgb([r, g, b])
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = (*ray).orig - center;
    let a = (*ray).dir.length_squared();
    let b = 2.0 * dot(oc, (*ray).dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 { -1.0 }
    else { (-b - discriminant.sqrt()) / (2.0 * a) }
}

fn ray_color(ray: Ray) -> image::Rgb<u8> {
    let t = hit_sphere(-Vec3::z_axis(), 0.5, &ray);
    if  t > 0.0 {
        let n = (ray.at(t) - (-Vec3::z_axis())).unit_vector();
        to_color(0.5 * (Vec3::new(n.x(), n.y(), n.z()) + Vec3::one()))
    }
    else {
        let unit_direction = ray.dir.unit_vector();
        let t2 = 0.5 * (unit_direction.y() + 1.0);
        to_color((1.0 - t2) * Vec3::one() + t2 * Vec3::new(0.5, 0.7, 1.0))
    }
}


fn main() -> std::io::Result<()> {
    println!("Configuring viewport and image buffer.");

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 384;
    let image_height: u32 = (image_width as f64 / aspect_ratio).floor() as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = viewport_width * Vec3::x_axis();
    let vertical = viewport_height * Vec3::y_axis();
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0
        - focal_length * Vec3::z_axis();

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    println!("Starting to render image.");

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = x as f64;
        let j = ((image_height - 1) - y) as f64;

        let u = i / (image_width - 1) as f64;
        let v = j / (image_height - 1) as f64;

        let ray = Ray::new(origin, lower_left_corner +
            u * horizontal + v * vertical - origin);

        *pixel = ray_color(ray);
    }

    println!("Finished rendering image.");

    imgbuf.save("./output/first_raytrace.png").unwrap();

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
