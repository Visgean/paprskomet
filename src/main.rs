mod canvas;
pub mod colors;
mod matrix;
mod scenes;
mod transformations;
mod utils;
pub mod vectors;
mod ray;

fn main() {
    scenes::write_projectile_image();
    scenes::clock();
}
