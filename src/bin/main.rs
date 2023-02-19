use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    // Image

    const IMG_HEIGHT: u32 = 480;
    const IMG_WIDTH: u32 = 640;
    const MN: f32 = 255.999;
    const SIZE: usize = (IMG_HEIGHT * IMG_WIDTH * 3) as usize;

    // Create png file
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    // Render

    let mut data: [u8; SIZE] = [0; SIZE];
    let mut index: usize = 0;
    for j in (0..IMG_HEIGHT).rev() {
        println!("{} scanlines left...", j);
        for i in 0..IMG_WIDTH {
            let r = i as f32 / (IMG_WIDTH - 1) as f32;
            let g = j as f32 / (IMG_HEIGHT - 1) as f32;
            let b = 0.25;

            data[index] = (MN * r) as u8;
            index += 1;
            data[index] = (MN * g) as u8;
            index += 1;
            data[index] = (MN * b) as u8;
            index += 1;
        }
    }
    println!("Finished!");

    writer.write_image_data(&data).unwrap();
}
