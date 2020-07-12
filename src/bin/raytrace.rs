extern crate image;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};
use rand::Rng;
use rand::prelude::ThreadRng;
use rayon::prelude::*;
use image::{RgbImage, ImageFormat};


//mod math;
//pub use raytracing::math;
//mod math;
//mod super::math;
//pub use crate::math;
use raytracing::math::math3::{Vec3, dot, random_vec_in_unit_sphere, reflect, random_unit_vector};
use raytracing::math::raytracing::{Ray, HitRecord, Hittable};

use raytracing::math::imaging::{Camera, to_color, ray_color};
use raytracing::math::materials::{Material};

//use crate::scenes;
//mod super::scenes;
use raytracing::scenes::spherion::{spherion_scene};
use raytracing::scenes::three_spheres::{three_spheres_scene};
use raytracing::scenes::first_fractal::{first_fractal_scene};
use raytracing::scenes::spherion_meets_fractalius::{spherion_meets_fractalius_scene};


fn main() -> std::io::Result<()> {

    println!("Configuring viewport and image buffer.");


    let aspect_ratio = 16.0 / 9.0;

    let print_every_n_rows: u32 = 20;
    let image_width: u32 = 100;
    let image_height: u32 = (image_width as f64 / aspect_ratio).floor() as u32;
    let samples_per_pixel = 100;
    let max_depth = 5;

    println!("Image width: {}, Image Height: {}, Samples Per Pixel: {}, Status print every {} rows",
             image_width, image_height, samples_per_pixel, print_every_n_rows);


    let vfov: f64 = 45.0;
    let aspect_ratio: f64 = (image_width as f64) / (image_height as f64);
    let lookat: Vec3 = Vec3::new(-0.0, 0.0, 0.0);
    let lookfrom: Vec3 = Vec3::new(-3.3, 2.0, 1.75);
    let vup: Vec3 = Vec3::y_axis();
    let cam: Camera = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio);

    //let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    println!("Starting to render image.");

    let thread_counter = Arc::new(AtomicUsize::new(0));

    let world = spherion_meets_fractalius_scene();
    let now_render = SystemTime::now();
    let result_vec: Vec<(u32, u32, Vec3)> = (0..image_width * image_height).into_par_iter().map(|index| {
        let mut rng = rand::thread_rng();


        let x = index as u32 % image_width;
        let y = (index as u32 - x) / image_width;
        let i = x as f64;
        let j = ((image_height - 1) - y) as f64;


        let pixel_color: Vec3 = (0..samples_per_pixel).map(|_| {
            let u = (i + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f64;
            let v = (j + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f64;
            let ray = cam.get_ray(u, v);
            ray_color(&mut rng, ray, &world, max_depth)
        }).fold(Vec3::zero(), |x, y| x + y);
        let count = thread_counter.fetch_add(1, Ordering::SeqCst);
        if count as u32 % (print_every_n_rows * image_width) == 0 {
            let rows_remaining = image_height - ((count as u32) / image_width);
            println!("Rows remaining: {}, Percent left to go: {}", rows_remaining, 100.0 * rows_remaining as f64 / image_height as f64);
        }
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

    let savepath =  "./output/throwaway";
    img.save(format!("{}.{}", savepath, "png")).unwrap();
    img.save(format!("{}.{}", savepath, "jpg")).unwrap();

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


