mod canvas;
pub mod colors;
mod matrix;
mod projectile;
mod utils;
pub mod vectors;
mod transformations;

fn main() {
    projectile::write_projectile_image();

    println!("Hello, world!");
}
