use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use rand::prelude::*;
use raytracer::camera::*;
use raytracer::hittable::*;
use raytracer::ray::*;
use raytracer::sphere::*;
use raytracer::vector3::*;

fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color3 {
    let mut hit_record = HitRecord::new();

    if depth <= 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.0, std::f64::INFINITY, &mut hit_record) {
        let target = hit_record.p + hit_record.normal + Vector3::random_in_unit_sphere();
        // 0.5 * (hit_record.normal + Color3::new(1.0, 1.0, 1.0))
        0.5 * ray_color(
            Ray::new(hit_record.p, target - hit_record.p),
            world,
            depth - 1,
        )
    } else {
        let unit_direction = Vector3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + (t * Color3::new(0.5, 0.7, 1.0))
    }
}
/*
fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = Vector3::dot(&oc, &r.direction());
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
*/
fn main() {
    let mut rng = rand::thread_rng();
    // Image
    const IMG_WIDTH: u32 = 640;
    const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;
    const IMG_SIZE: usize = (IMG_HEIGHT * IMG_WIDTH * 3) as usize;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    const MN: f64 = 256.0;

    // World
    let mut world: HittableList = HittableList::default();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    // let x: f64 = rand::random();

    let mut data: [u8; IMG_SIZE] = [0; IMG_SIZE];
    let mut index: usize = 0;
    for j in (0..IMG_HEIGHT).rev() {
        println!("{} scanlines left...", j);
        for i in 0..IMG_WIDTH {
            let mut pixel_color = Color3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u_rand: f64 = rng.gen();
                let v_rand: f64 = rng.gen();
                let u = (i as f64 + u_rand) / (IMG_WIDTH - 1) as f64;
                let v = (j as f64 + v_rand) / (IMG_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, MAX_DEPTH);
            }

            // write_color
            let mut r = pixel_color.x();
            let mut g = pixel_color.y();
            let mut b = pixel_color.z();

            let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
            r *= scale;
            g *= scale;
            b *= scale;

            data[index] = (MN * raytracer::clamp(r, 0.0, 0.999)) as u8; //(MN * r) as u8;
            index += 1;
            data[index] = (MN * raytracer::clamp(g, 0.0, 0.999)) as u8; // (MN * g) as u8;
            index += 1;
            data[index] = (MN * raytracer::clamp(b, 0.0, 0.999)) as u8; // (MN * b) as u8;
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
