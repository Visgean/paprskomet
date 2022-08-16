extern crate core;

mod canvas;
mod projectile;
pub mod vectors;
pub mod colors;

fn main() {
    projectile::write_projectile_image();

    println!("Hello, world!");
}
