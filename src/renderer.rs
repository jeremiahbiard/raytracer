#[allow(dead_code)]
use std::fmt;
use std::ops::{Mul, Add};

struct Image(u16, u16);
pub struct Color{
    red: f64, 
    green: f64, 
    blue: f64,
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color {
        red: self.red + rhs.red,
        green: self.green + rhs.green,
        blue: self.blue + rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

        fn mul(self, rhs: Color) -> Color {
            Color {
                red: rhs.red * self,
                green: rhs.green * self,
                blue: rhs.blue * self,
                
            }
        }
}

impl Add<f64> for Color {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Color {
            red: self.red + rhs,
            green: self.green + rhs,
            blue: self.blue + rhs,
        }
    }

}

impl Add<Color> for f64 {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            red: rhs.red + self,
            green: rhs.green + self,
            blue: rhs.blue + self,
        }
    }


}

const IMAGE_HEIGHT: u16 = 256;
const IMAGE_WIDTH: u16 = 256;

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Color {
    pub fn with_rgb(red: f64, green: f64, blue: f64) -> Color { 
        Color {
        red,
        green,
        blue,
        }
    }
}

// TODO(Jeremiah) add camera
pub fn render() {
    let image = Image(256, 256);

    print!("P3\n{} {}\n255\n", image.0, image.1);

    for j in (0..image.0 - 1).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image.1 {
            write_pixel(pixel_color(i, j));
        }
    }
    eprint!("\nDone.\n");
}

// TODO(Jeremiah) make these private after adding camera arg to render()
pub fn pixel_color(i: u16, j: u16) -> Color {
    Color::with_rgb(
        i as f64 / (IMAGE_WIDTH - 1) as f64,
        j as f64 / (IMAGE_HEIGHT - 1) as f64,
        0.25,
    )
}

pub fn write_pixel(color: Color) {
    let ir = (255.999 * color.red) as u16;
    let ig = (255.999 * color.green) as u16;
    let ib = (255.999 * color.blue) as u16;

    print!("{} {} {}\n", ir, ig, ib);
}
