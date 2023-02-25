pub mod camera;
pub mod hittable;
mod ray;
pub mod sphere;
pub mod vector3;

pub mod prelude {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;
    pub const IMG_WIDTH: u32 = 100;
    pub const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;
    const IMG_SIZE: usize = (IMG_HEIGHT * IMG_WIDTH * 3) as usize;
    const MAX_DEPTH: u32 = 50;

    use super::camera::*;
    use super::hittable::*;
    use super::ray::*;
    use super::vector3::Color3;

    use rand::prelude::*;

    use rayon::prelude::*;

    const MN: f64 = 256.0;
    const SAMPLES_PER_PIXEL: u32 = 100;

    #[inline]
    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }

    fn write_color(pixels: &mut [u8], pixel_color: Color3, index: usize) {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        pixels[index] = (MN * clamp(r, 0.0, 0.999)) as u8; //(MN * r) as u8;
        pixels[index + 1] = (MN * clamp(g, 0.0, 0.999)) as u8; // (MN * g) as u8;
        pixels[index + 2] = (MN * clamp(b, 0.0, 0.999)) as u8; // (MN * b) as u8;
    }

    fn render_line(pixels: &mut [u8], camera: &Camera, scene: &HittableList, row: u32) {
        println!("{} scanlines remaining", row);
        for col in 0..IMG_WIDTH {
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                // let u_rand: f64 = rng.gen();
                // let v_rand: f64 = rng.gen();
                let u = (col as f64 + rng.gen::<f64>()) / (IMG_WIDTH - 1) as f64;
                let v = ((IMG_HEIGHT - row) as f64 + rng.gen::<f64>()) / (IMG_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, scene, MAX_DEPTH);
            }

            let index = ((row * IMG_WIDTH + (col * 3)) + row * IMG_WIDTH * 2) as usize;
            write_color(pixels, pixel_color, index);
        }
    }

    pub fn render(camera: &Camera, scene: &HittableList) {
        let mut pixels: [u8; IMG_SIZE] = [0; IMG_SIZE];
        for row in (0..IMG_HEIGHT).rev() {
            render_line(&mut pixels, camera, scene, row);
        }
        write_image(&pixels);
    }

    fn write_image(pixels: &[u8]) {
        let path = Path::new(r"image.png");
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);

        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(pixels).unwrap();
    }
}
