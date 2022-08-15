
use crate::vectors::Vector3D;
use crate::canvas;

pub fn projectile(max_x: f32) -> Vec<Vector3D> {
    let gravity = Vector3D::vector(0.0, -0.587, 0.0);
    let wind = Vector3D::vector(-0.03, 0.0, 0.0);
    let mut position = Vector3D::point(0.0, 1.0, 0.0);

    let mut velocity = Vector3D::vector(1.0, 1.8, 0.0, ).normalize() * 30.0;


    let mut position_log: Vec<Vector3D> = vec![];


    while position.x < max_x {
        position_log.push(position);
        position = position + velocity;
        velocity = velocity + gravity + wind;
    }

    position_log
}