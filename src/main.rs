mod canvas;
pub mod colors;
mod geometries;
mod matrix;
mod ray;
mod scenes;
mod transformations;
mod utils;
pub mod vectors;

fn main() {
    scenes::write_projectile_image();
    scenes::clock();
}
