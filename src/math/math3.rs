
use std::ops::{Neg, Add, Sub, Mul, Div};
use rand::Rng;
use rand::prelude::ThreadRng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn x_axis() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    pub fn y_axis() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub fn z_axis() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        dot(*self, *self)
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3::new(self.e[0].sqrt(), self.e[1].sqrt(), self.e[2].sqrt())
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random(rng_source: &mut ThreadRng) -> Vec3 {
        Vec3::new((*rng_source).gen_range(0.0, 1.0),
                  (*rng_source).gen_range(0.0, 1.0),
                  (*rng_source).gen_range(0.0, 1.0))
    }

    pub fn random_range(rng_source: &mut ThreadRng, min: f64, max: f64) -> Vec3 {
        Vec3::new((*rng_source).gen_range(min, max),
                  (*rng_source).gen_range(min, max),
                  (*rng_source).gen_range(min, max))
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.e[0], self.e[1], self.e[2])
    }

    pub fn print_string(&self) -> () {
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

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
        v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
        v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0]
    )
}

pub fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    vec - 2.0 * dot(vec, normal) * normal
}


#[allow(unused_assignments)]
pub fn random_vec_in_unit_sphere(rng_source: &mut ThreadRng) -> Vec3 {
    let mut random_vec: Vec3 = Vec3::zero();

    loop {
        random_vec = Vec3::random_range(rng_source, -1.0, 1.0);
        if random_vec.length_squared() <= 1.0 {
            break;
        }
    }
    random_vec
}

pub fn random_unit_vector(rng_source: &mut ThreadRng) -> Vec3 {
    let tau: f64 = 6.28318530717958647692528676655900577f64;
    let angle: f64 = rng_source.gen_range(0.0, tau);
    let height: f64 = rng_source.gen_range(-1.0, 1.0);
    let radius: f64 = (1.0 - height * height).sqrt();

    Vec3::new(radius * angle.cos(), radius * angle.sin(), height)
}

/* old verification stuff
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
