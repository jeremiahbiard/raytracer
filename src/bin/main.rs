use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use raytracer::ray::*;
use raytracer::vector3::*;

fn ray_color(r: &Ray) -> Color3 {
    let unit_direction = Vector3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + (t * Color3::new(0.5, 0.7, 1.0))
}
fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 1920;
    const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;
    const IMG_SIZE: usize = (IMG_HEIGHT * IMG_WIDTH * 3) as usize;

    const MN: f64 = 255.999;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

    // Render

    let mut data: [u8; IMG_SIZE] = [0; IMG_SIZE];
    let mut index: usize = 0;
    for j in (0..IMG_HEIGHT).rev() {
        println!("{} scanlines left...", j);
        for i in 0..IMG_WIDTH {
            let u = i as f64 / (IMG_WIDTH - 1) as f64;
            let v = j as f64 / (IMG_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            // let r = i as f64 / (IMG_WIDTH - 1) as f64;
            // let g = j as f64 / (IMG_HEIGHT - 1) as f64;
            // let b = 0.25;
            // println!("{:#?}", pixel_color);

            data[index] = (MN * pixel_color.x()) as u8; //(MN * r) as u8;
            index += 1;
            data[index] = (MN * pixel_color.y()) as u8; // (MN * g) as u8;
            index += 1;
            data[index] = (MN * pixel_color.z()) as u8; // (MN * b) as u8;
            index += 1;
        }
    }
    println!("Finished!");

    // Create png file
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}
