

#[derive(Debug)]
struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}


impl Vector3D {
    pub fn new (x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    pub fn point (x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 1.0
        }
    }
    pub fn vector (x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 0.0
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::types::Vector3D;

    #[test]
    fn test_point() {
        let point = Vector3D::point(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.w, 1.0);
    }
}