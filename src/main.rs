mod canvas;
pub mod colors;
mod matrix;
mod scenes;
mod utils;
pub mod vectors;
mod transformations;

fn main() {
    scenes::write_projectile_image();
    scenes::clock();
}
