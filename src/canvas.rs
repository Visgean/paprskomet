use crate::colors::Color;
use crate::vectors::Tuple;
use std::fs;

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,

    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width: width,
            height: height,
            pixels: vec![
                vec![Color::black(); width];
                height
            ],
        }
    }

    pub fn write(
        &mut self,
        x: usize,
        y: usize,
        color: Color,
    ) {
        // so the problem with just writing to canvas is that it will be rendered from upper left..
        // we could change the .to_ppm method to render it upside down sort of, but it makes more
        // sense to hide the vector implementation...

        if x > self.width || y > self.height {
            // i could panick but why? lets just silently ignore this!
            return;
            // panic!("Out of bounds write to canvas!")
        }

        let row = self.height - y - 1;
        self.pixels[row][x] = color;
    }

    pub fn write_origin(
        &mut self,
        x: isize,
        y: isize,
        color: Color,
    ) {
        // writes pixel referenced by midpoint of the canvas...

        let mid_x = self.width as isize / 2;
        let mid_y = self.height as isize / 2;

        self.write(
            (mid_x + x) as usize,
            (mid_y + y) as usize,
            color,
        );
    }

    pub fn write_point(&mut self, p: Tuple, color: Color) {
        let x = p.x as isize;
        let y = p.y as isize;

        self.write_origin(x, y, color);
    }

    pub fn to_ppm(&self) -> String {
        let header = format!(
            "P3\n{} {}\n255",
            self.width, self.height
        );

        let pixel_array: Vec<String> = self
            .pixels
            .iter()
            .map(|x| {
                x.iter().map(|p| p.to_str()).collect()
            })
            .collect();

        let string_value = pixel_array.join("\n");

        format!("{} \n{}\n", header, string_value)
    }

    pub fn write_ppm(&self, filename: &str) {
        fs::write(filename, self.to_ppm())
            .expect("Unable to write file");
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::colors::Color;

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
