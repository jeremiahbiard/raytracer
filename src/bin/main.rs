use png::{ColorType, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    // Image

    const IMG_HEIGHT: u32 = 256;
    const IMG_WIDTH: u32 = 256;
    const MN: f32 = 255.999;
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    // Render
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);

    let mut data: Vec<u8> = Vec::new();
    for j in (0..IMG_HEIGHT - 1).rev() {
        for i in 0..IMG_WIDTH {
            let r = i as f32 / (IMG_WIDTH - 1) as f32;
            let g = j as f32 / (IMG_HEIGHT - 1) as f32;
            let b = 0.25;

            let ir = (MN * r) as u8;
            let ig = (MN * g) as u8;
            let ib = (MN * b) as u8;

            data.push(ir);
            data.push(ig);
            data.push(ib);
            // println!("{} {} {}", ir, ig, ib);
        }
    }
    let array: [u8; (IMG_HEIGHT * IMG_WIDTH) as usize] =
        match data.try_into().unwrap_or_else(|error| {
            panic!("Problem: {:?}", error);
        }) {};

    writer.write_image_data(&array).unwrap();
}
