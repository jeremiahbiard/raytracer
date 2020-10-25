pub mod vec3;

pub mod create_ppm {

    use std::fmt;

    struct Image(u16, u16);

    impl fmt::Display for Image {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    /// Creates a simple test image
    pub fn test_image() {
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
}
