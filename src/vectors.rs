use crate::utils::float_compare;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub}; // Import `fmt`

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:.2}, {:.2}, {:.2}, {:.2})",
            self.x, self.y, self.z, self.w
        )
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
            .sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w,
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        // if self.w == 1.0 || rhs.w == 1.0 {
        //     panic!("Cant dot point to point!")
        // }

        self.x * rhs.x
            + self.y * rhs.y
            + self.z * rhs.z
            + self.w * rhs.w
    }

    fn cross(&self, rhs: &Self) -> Self {
        if self.w == 1.0 && rhs.w == 1.0 {
            panic!("Cant add point to point!")
        }

        Tuple::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
            0.0,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_compare(self.x, other.x)
            && float_compare(self.y, other.y)
            && float_compare(self.z, other.z)
            && float_compare(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        if self.w == 1.0 && rhs.w == 1.0 {
            panic!("Cant add point to point!")
        }

        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        // if self.w == 1.0 && rhs.w == 1.0 {
        //     panic!("Cant add point to point!")
        // }

        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, self.w)
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        if rhs.w == 1.0 {
            panic!("Cant use scalar multiplication on a point!!")
        }

        Tuple::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
            rhs.w,
        )
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        if self.w == 1.0 {
            panic!("Cant use scalar multiplication on a point!!")
        }

        Tuple::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w,
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        if self.w == 1.0 {
            panic!("Cant use scalar multiplication on a point!!")
        }

        Tuple::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::float_compare;
    use crate::vectors::Tuple;

    #[test]
    fn test_point() {
        let point = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn test_compare() {
        let point_a = Tuple::point(1.0, 2.0, 3.0);
        let point_b = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(point_a, point_b);
    }

    #[test]
    fn test_add_vectors() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::vector(2.0, 3.0, 5.0);

        let vector_sum = Tuple::vector(3.0, 5.0, 8.0);

        assert_eq!(vector_a + vector_b, vector_sum);
    }

    #[test]
    fn test_add_vector_point() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::point(2.0, 3.0, 5.0);

        let vector_sum = Tuple::point(3.0, 5.0, 8.0);

        assert_eq!(vector_a + vector_b, vector_sum);
    }

    #[test]
    #[should_panic]
    fn test_add_point_point() {
        let point_a = Tuple::point(1.0, 2.0, 3.0);
        let point_b = Tuple::point(2.0, 3.0, 5.0);

        let point_sum = Tuple::point(3.0, 5.0, 8.0);

        assert_eq!(point_a + point_b, point_sum);
    }

    #[test]
    fn test_subtract_vectors() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::vector(2.0, 3.0, 5.0);

        let vector_sum = Tuple::vector(-1.0, -1.0, -2.0);

        assert_eq!(vector_a - vector_b, vector_sum);
    }

    #[test]
    fn test_neg_ident() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let neg = Tuple::vector(-1.0, -2.0, -3.0);

        assert_eq!(-vector_a, neg);
    }

    #[test]
    fn test_zero() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let zero = Tuple::vector(0.0, 0.0, 0.0);

        assert_eq!(-vector_a + vector_a, zero);
        assert_eq!(vector_a - vector_a, zero);
    }

    #[test]
    fn test_scalar_multiplication() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let result = Tuple::vector(3.0, 6.0, 9.0);

        assert_eq!(3.0 * vector_a, result);
        assert_eq!(vector_a * 3.0, result);
    }

    #[test]
    #[should_panic]
    fn test_scalar_point_mul() {
        let _point_a = Tuple::point(1.0, 2.0, 3.0) * 3.0;
    }

    #[test]
    #[should_panic]
    fn test_scalar_point_mul_reverse() {
        let _point_a = 3.0 * Tuple::point(1.0, 2.0, 3.0);
    }

    #[test]
    fn test_scalar_div() {
        let vector_a = Tuple::vector(3.0, 6.0, 9.0);
        let result = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(vector_a / 3.0, result);
    }

    #[test]
    fn test_magnitude() {
        let vector_a = Tuple::vector(3.0, 6.0, 9.0);

        assert!(float_compare(
            vector_a.magnitude(),
            11.224972
        ));
    }

    #[test]
    fn test_normalization() {
        let vector_a = Tuple::vector(3.0, 6.0, 9.0);
        assert_eq!(vector_a.normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_dot() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(vector_a.dot(&vector_b), 20.0);
    }

    #[test]
    fn test_cross() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::vector(2.0, 3.0, 4.0);
        let vector_c = Tuple::vector(-1.0, 2.0, -1.0);
        assert_eq!(vector_a.cross(&vector_b), vector_c);
    }
}
