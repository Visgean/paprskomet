use crate::vectors::Vector3D;

use crate::canvas::Canvas;
use crate::colors::Color;

pub fn write_projectile_image() {
    let mut canvas = Canvas::new(1200, 800);

    let projectile_log = projectile(1200.0);

    for pos in projectile_log {
        let x = pos.x as usize;
        let y = pos.y as usize;

        // canvas.pixels[y][x] = Color::red();
        canvas.write(x, y, Color::red())
    }

    canvas.write_ppm("./image.ppm");
}

pub fn projectile(max_x: f64) -> Vec<Vector3D> {
    let gravity = Vector3D::vector(0.0, -0.587, 0.0);
    let wind = Vector3D::vector(-0.03, 0.0, 0.0);
    let mut position = Vector3D::point(0.0, 1.0, 0.0);

    let mut velocity = Vector3D::vector(1.0, 1.8, 0.0).normalize() * 30.0;

    let mut position_log: Vec<Vector3D> = vec![];

    while position.x < max_x {
        position_log.push(position);
        position = position + velocity;
        velocity = velocity + gravity + wind;
    }

    position_log
}
