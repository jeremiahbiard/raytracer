fn main() {
    // Image

    const IMG_HEIGHT: i32 = 256;
    const IMG_WIDTH: i32 = 256;
    const MN: f32 = 255.999;
    // Render
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT - 1).rev() {
        eprintln!("{} Scanlines remaining...", j);
        for i in 0..IMG_WIDTH {
            let r = i as f32 / (IMG_WIDTH - 1) as f32;
            let g = j as f32 / (IMG_HEIGHT - 1) as f32;
            let b = 0.25;

            let ir = (MN * r) as i32;
            let ig = (MN * g) as i32;
            let ib = (MN * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Finished!");
}
