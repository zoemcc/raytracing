extern crate image;

use std::time::{Duration, SystemTime};
use rand::Rng;
use rand::prelude::ThreadRng;
use rayon::prelude::*;
use image::{RgbImage, ImageFormat};

mod math;
pub use crate::math::math3::{Vec3, dot, random_vec_in_unit_sphere, reflect, random_unit_vector};
pub use crate::math::raytracing::{Ray, HitRecord, Hittable};
pub use crate::math::imaging::{Camera, to_color, ray_color};
pub use crate::math::materials::{Material, Metal, Lambertian};
pub use crate::math::hittables::{Sphere};

mod scenes;
pub use crate::scenes::spherion::{spherion_scene};
pub use crate::scenes::three_spheres::{three_spheres_scene};
pub use crate::scenes::first_fractal::{first_fractal_scene};


fn main() -> std::io::Result<()> {
    println!("Configuring viewport and image buffer.");


    let aspect_ratio = 16.0 / 9.0;
    //let aspect_ratio = 1.0;

    let print_every_n_rows: u32 = 20;
    let image_width: u32 = 2000;
    let image_height: u32 = (image_width as f64 / aspect_ratio).floor() as u32;
    let samples_per_pixel = 100;
    let max_depth = 4;

    println!("Image width: {}, Image Height: {}, Samples Per Pixel: {}, Status print every {} rows",
             image_width, image_height, samples_per_pixel, print_every_n_rows);


    let cam: Camera = Camera::new();

    //let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    println!("Starting to render image.");

    let now_render = SystemTime::now();
    let result_vec: Vec<(u32, u32, Vec3)> = (0..image_width * image_height).into_par_iter().map(|index| {
        let mut rng = rand::thread_rng();

        let world = first_fractal_scene();

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

    img.save("./output/signed_distance_sierpinski_second.png").unwrap();

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


