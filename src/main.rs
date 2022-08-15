extern crate core;

use crate::canvas::Canvas;
use colors::Color;
use projectile::projectile;

mod canvas;
mod projectile;
pub mod vectors;
pub mod colors;

fn main() {

    let mut canvas = Canvas::new(1200, 800);

    let projectile_log = projectile(1200.0);

    for pos in projectile_log {
        let x = pos.x as usize;
        let y = pos.y as usize;

        // canvas.pixels[y][x] = Color::red();
        canvas.write(x, y, Color::red())

    }



    canvas.write_ppm("./image.ppm");

    println!("Hello, world!");
}
