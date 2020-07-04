extern crate image;

use std::time::{Duration, SystemTime};
use rand::Rng;
use rand::prelude::ThreadRng;
use rayon::prelude::*;
use image::{RgbImage, ImageFormat};

mod math;
pub use crate::math::math3::{Vec3, dot, random_vec_in_unit_sphere, reflect, random_unit_vector};
pub use crate::math::raytracing::{Ray, HitRecord, Hittable, HittableList};
pub use crate::math::imaging::{Camera, to_color, ray_color};
pub use crate::math::materials::{Material, Metal, Lambertian};
pub use crate::math::hittables::{Sphere};


fn main() -> std::io::Result<()> {
    println!("Configuring viewport and image buffer.");


    let aspect_ratio = 16.0 / 9.0;

    let print_every_n_rows: u32 = 20;
    let image_width: u32 = 200;
    let image_height: u32 = (image_width as f64 / aspect_ratio).floor() as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("Image width: {}, Image Height: {}, Samples Per Pixel: {}, Status print every {} rows",
             image_width, image_height, samples_per_pixel, print_every_n_rows);


    let cam: Camera = Camera::new();

    //let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    println!("Starting to render image.");

    let now_render = SystemTime::now();
    let result_vec: Vec<(u32, u32, Vec3)> = (0..image_width * image_height).into_par_iter().map(|index| {
        let mut rng = rand::thread_rng();
        /*
        let world: Box<dyn Hittable> = Box::new(HittableList {
            hittables: vec![
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0,
                                     Box::new(Lambertian::new(Vec3::new(0.1, 0.8, 0.4))))),
                Box::new(Sphere::new(-Vec3::z_axis(), 0.5,
                                     Box::new(Lambertian::new(Vec3::new(0.5, 0.4, 0.7))))),
                Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5,
                                     Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.7)))),
                Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5,
                                     Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.2))))
            ]
        });
         */
        let world: Box<dyn Hittable> = Box::new(HittableList {
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
        });


        let x = index as u32 % image_width;
        let y = (index as u32 - x) / image_width;
        let i = x as f64;
        let j = ((image_height - 1) - y) as f64;

        if index as u32 % (print_every_n_rows * image_width) == 0 {
            println!("Pixel (x, y): ({}, {}), Rows remaining: {}", x, y, image_height - y);
        }

        let pixel_color: Vec3 = (0..samples_per_pixel).map(|_| {
            let u = (i + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f64;
            let v = (j + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f64;
            let ray = cam.get_ray(u, v);
            ray_color(&mut rng, ray, &world, max_depth)
        }).fold(Vec3::zero(), |x, y| x + y);
        (x, y, pixel_color)
    }).collect();

    match now_render.elapsed() {
        Ok(elapsed) => {
            println!("Rendering complete! took {} seconds", elapsed.as_secs());
        }
        Err(e) => {
            println!("Rendering error! {:?}", e);
        }
    }

    println!("Finished rendering image.");

    let now_save = SystemTime::now();

    let mut img = RgbImage::new(image_width, image_height);
    for (x, y, pixel_color) in result_vec {
        img.put_pixel(x, y, to_color(pixel_color, samples_per_pixel));
    }

    img.save("./output/spherion_test.png").unwrap();

    match now_save.elapsed() {
        Ok(elapsed) => {
            println!("Saving complete! took {} seconds", elapsed.as_secs());
        }
        Err(e) => {
            println!("Rendering error! {:?}", e);
        }
    }


    println!("Finished saving image.");

    Ok(())
}


/* loading/reencoding code
let imgres = image::open("./output/spherion_the_terrible.png");
match imgres {
    Ok(img2) => {
        println!("loading success! starting to encode as jpg");
        img2.save_with_format("./output/spherion_the_terrible.jpg",
                              ImageFormat::Jpeg);
    }
    _ => {println!("image reading failed!");}
}

 */


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
