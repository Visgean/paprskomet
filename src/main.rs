extern crate core;

use crate::canvas::Canvas;
use crate::canvas::colors::Color;

mod canvas;

fn main() {

    let mut canvas = Canvas::new(1200, 800);
    canvas.pixels[1][1] = Color::red();

    canvas.write_ppm("./image.ppm");
    
    println!("Hello, world!");
}
