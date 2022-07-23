use std::ops::{Add, Neg, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
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

impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        if self.w == 1.0 && rhs.w == 1.0 {
            panic!("Cant add point to point!")
        }

        Vector3D::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w
        )
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.w == 1.0 && rhs.w == 1.0 {
            panic!("Cant add point to point!")
        }

        Vector3D::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w
        )
    }
}



impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D::new(
            -self.x,
            -self.y,
            -self.z,
            self.w
        )
    }
}


impl Mul<Vector3D> for f32 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        if rhs.w == 1.0 {
            panic!("Cant use scalar multiplication on a point!!")
        }

        Vector3D::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
            rhs.w
        )
    }
}


impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f32) -> Self::Output {
        if self.w == 1.0 {
            panic!("Cant use scalar multiplication on a point!!")
        }

        Vector3D::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w
        )
    }
}


impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f32) -> Self::Output {
        if self.w == 1.0 {
            panic!("Cant use scalar multiplication on a point!!")
        }

        Vector3D::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w
        )
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

    #[test]
    fn test_compare() {
        let point_a = Vector3D::point(1.0, 2.0, 3.0);
        let point_b = Vector3D::point(1.0, 2.0, 3.0);
        assert_eq!(point_a, point_b);
    }

    #[test]
    fn test_add_vectors() {
        let vector_a = Vector3D::vector(1.0, 2.0, 3.0);
        let vector_b = Vector3D::vector(2.0, 3.0, 5.0);

        let vector_sum = Vector3D::vector(3.0, 5.0, 8.0);

        assert_eq!(vector_a + vector_b, vector_sum);
    }

    #[test]
    fn test_add_vector_point() {
        let vector_a = Vector3D::vector(1.0, 2.0, 3.0);
        let vector_b = Vector3D::point(2.0, 3.0, 5.0);

        let vector_sum = Vector3D::point(3.0, 5.0, 8.0);

        assert_eq!(vector_a + vector_b, vector_sum);
    }


    #[test]
    #[should_panic]
    fn test_add_point_point() {
        let point_a = Vector3D::point(1.0, 2.0, 3.0);
        let point_b = Vector3D::point(2.0, 3.0, 5.0);

        let point_sum = Vector3D::point(3.0, 5.0, 8.0);

        assert_eq!(point_a + point_b, point_sum);
    }


    #[test]
    fn test_subtract_vectors() {
        let vector_a = Vector3D::vector(1.0, 2.0, 3.0);
        let vector_b = Vector3D::vector(2.0, 3.0, 5.0);

        let vector_sum = Vector3D::vector(-1.0, -1.0, -2.0);

        assert_eq!(vector_a - vector_b, vector_sum);
    }

    #[test]
    fn test_neg_ident() {
        let vector_a = Vector3D::vector(1.0, 2.0, 3.0);
        let neg = Vector3D::vector(-1.0, -2.0, -3.0);

        assert_eq!(-vector_a, neg);
    }

    #[test]
    fn test_zero() {
        let vector_a = Vector3D::vector(1.0, 2.0, 3.0);
        let zero = Vector3D::vector(0.0,0.0,0.0);

        assert_eq!(-vector_a + vector_a, zero);
        assert_eq!(vector_a - vector_a, zero);
    }



    #[test]
    fn test_scalar_multiplication() {
        let vector_a = Vector3D::vector(1.0, 2.0, 3.0);
        let result = Vector3D::vector(3.0, 6.0, 9.0);

        assert_eq!(3.0 * vector_a, result);
        assert_eq!(vector_a * 3.0, result);
    }

    #[test]
    #[should_panic]
    fn test_scalar_point_mul() {
        let _point_a = Vector3D::point(1.0, 2.0, 3.0) * 3.0;
    }

    #[test]
    #[should_panic]
    fn test_scalar_point_mul_reverse() {
        let _point_a = 3.0 * Vector3D::point(1.0, 2.0, 3.0);
    }

    #[test]
    fn test_scalar_div() {
        let vector_a = Vector3D::vector(3.0, 6.0, 9.0);
        let result = Vector3D::vector(1.0, 2.0, 3.0);

        assert_eq!(vector_a / 3.0, result);
    }

}