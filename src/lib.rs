pub mod vec3;

#[allow(dead_code)]
pub mod renderer {
    use std::fmt;

    struct Image(u16, u16);
    struct Color(f64, f64, f64);

    const IMAGE_HEIGHT: i32 = 256;
    const IMAGE_WIDTH: i32 = 256;

    impl fmt::Display for Image {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    pub fn render() {
        let image = Image(256, 256);

        print!("P3\n{} {}\n255\n", image.0, image.1);

        for j in (0..image.0 - 1).rev() {
            eprintln!("\rScanlines remaining: {}", j);
            for i in 0..image.1 {
                let r = i as f32 / (image.0 as f32 - 1.0);
                let g = j as f32 / (image.1 as f32 - 1.0);
                let b = 0.25;

                let ir = (255.999 * r) as u16;
                let ig = (255.999 * g) as u16;
                let ib = (255.999 * b) as u16;

                print!("{} {} {}\n", ir, ig, ib);
            }
        }
        eprint!("\nDone.\n");
    }

    fn pixel_color(i: i32, j: i32) -> Color {
        Color(
            i as f64 / (IMAGE_WIDTH - 1) as f64,
            j as f64 / (IMAGE_HEIGHT - 1) as f64,
            0.25,
        )
    }

    fn write_pixel(color: Color) {
        print!("{} {} {}\n", color.0, color.1, color.2);
    }
}
