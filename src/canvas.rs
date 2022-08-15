use std::fmt::format;

use crate::canvas::colors::Color;
use std::fs;

pub mod vectors;
pub mod colors;




#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,

    pub pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width: width,
            height: height,
            pixels: vec![vec![Color::black(); width]; height],
        }
    }

    pub fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255", self.width, self.height);

        let pixel_array: Vec<String> = self.pixels.iter().map(
            |x| x.iter().map(|p| p.to_str()).collect()
        ).collect();

        let string_value = pixel_array.join("\n");

        format!("{} \n{}", header, string_value)
    }

    pub fn write_ppm(&self, filename: &str) {
        fs::write(filename, self.to_ppm()).expect("Unable to write file");

    }

}



#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::canvas::colors::Color;

    #[test]
    fn test_canvas() {
        let canvas = Canvas::new(30, 30);
        assert_eq!(Color::black(), canvas.pixels[0][0])
    }

    #[test]
    fn test_canvas_write() {
        let mut canvas = Canvas::new(30, 30);
        canvas.pixels[1][1] = Color::red();

        assert_eq!(Color::red(), canvas.pixels[1][1])
    }


    //
    // #[test]
    // fn test_ppm() {
    //     let mut canvas = Canvas::new(4, 4);
    //     canvas.pixels[0][0] = Color::red();
    //
    //     assert_eq!("potato", canvas.to_ppm())
    // }


}

