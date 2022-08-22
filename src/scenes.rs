use std::f64::consts::PI;
use crate::vectors::Tuple;

use crate::canvas::Canvas;
use crate::colors::Color;
use crate::transformations::{rotation_x, rotation_y, rotation_z, translation};

pub fn write_projectile_image() {
    let mut canvas = Canvas::new(1200, 800);

    let projectile_log = projectile(1200.0);

    for pos in projectile_log {
        let x = pos.x as usize;
        let y = pos.y as usize;

        // canvas.pixels[y][x] = Color::red();
        canvas.write(x, y, Color::red())
    }

    canvas.write_ppm("./projectile.ppm");
}

pub fn projectile(max_x: f64) -> Vec<Tuple> {
    let gravity = Tuple::vector(0.0, -0.587, 0.0);
    let wind = Tuple::vector(-0.03, 0.0, 0.0);
    let mut position = Tuple::point(0.0, 1.0, 0.0);

    let mut velocity = Tuple::vector(1.0, 1.8, 0.0).normalize() * 30.0;

    let mut position_log: Vec<Tuple> = vec![];

    while position.x < max_x {
        position_log.push(position);
        position = position + velocity;
        velocity = velocity + gravity + wind;
    }

    position_log
}


pub fn clock() {
    let mut canvas = Canvas::new(1200, 800);

    let mut p = Tuple::vector(0., 300., 0.);
    let d = rotation_z((2. * PI) / 12.);

    for i in 0..12 {
        p = d.clone() * p;
        canvas.write_point(p, Color::red());
    }


    canvas.write_ppm("./clock.ppm");
}


























