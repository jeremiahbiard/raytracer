#[allow(unused_imports)]

use raytracer::renderer::{Color, render};
use raytracer::renderer;
use raytracer::vec3::*;
use raytracer::Ray;


struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
}

struct Image {
    height: u16,
    width: u16,
}

impl Camera {
    pub fn new(origin: Point3, horizontal: Vec3, vertical: Vec3, lower_left: Vec3) -> Camera {
        Camera {
        origin,
        horizontal,
        vertical,
        lower_left,
        }
    }
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1600;
    let image_height = (image_width as f64 / aspect_ratio) as u16;

    let image = Image{width: image_width, height: image_height};

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    
    let origin = Point3::with_xyz(0.0, 0.0, 0.0);
    let horizontal = Vec3::with_xyz(viewport_width, 0.0, 0.0);
    let vertical = Vec3::with_xyz(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::with_xyz(0.0, 0.0, focal_length);

    let camera = Camera { origin, horizontal, vertical, lower_left};

    // Render
    print!("P3\n{} {}\n255\n", image.width, image.height);

    for j in (0..image.height - 1).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image.width {
            let u = i as f64 / (image.width - 1) as f64;
            let v = j as f64 / (image.width - 1) as f64;
            let r = Ray::new(origin, lower_left + u * horizontal + v * vertical - origin);
            let pixel_color = Ray::ray_color(r);
            renderer::write_pixel(pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
