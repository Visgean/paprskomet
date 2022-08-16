use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}


fn f_pixel(n: f32) -> usize{
    // we store pixels in f32 but for ppm format we need them as integers..
    let x = n * 255.0;
    let mut int_x = x as usize;

    if int_x > 255 {
        int_x = 255
    }
    int_x
}


impl Color {
    pub fn new (r: f32, g: f32, b: f32) -> Self {
        Self {r, g, b}
    }

    pub fn black() -> Self {
        Self {r:0.0, g:0.0, b:0.0}
    }

    pub fn red() -> Self {
        Self {r:1.0, g:0.0, b:0.0}
    }

    pub fn to_str(&self) -> String {
        format!("{} {} {}\n", f_pixel(self.r), f_pixel(self.g), f_pixel(self.b))
    }



    pub fn magnitude(&self) -> f32 {
        (
            self.r.powi(2) + self.g.powi(2) + self.b.powi(2)
        ).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        Self {
            r: self.r / m,
            g: self.g / m,
            b: self.b / m,
        }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.r * rhs.r +
        self.g * rhs.g +
        self.b * rhs.b
    }
}



impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
        )
    }
}



impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self * rhs.r,
            self * rhs.g,
            self * rhs.b,
        )
    }
}


impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(
            self.r * rhs,
            self.g * rhs,
            self.b * rhs,
        )
    }
}


impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color::new(
            self.r / rhs,
            self.g / rhs,
            self.b / rhs,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::colors::Color;

    #[test]
    fn test_colors() {
        let color = Color::new(1.0, 2.0, 3.0);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.b, 3.0);
    }

    #[test]
    fn test_compare() {
        let color_a = Color::new(1.0, 2.0, 3.0);
        let color_b = Color::new(1.0, 2.0, 3.0);
        assert_eq!(color_a, color_b);
    }

    #[test]
    fn test_add_colors() {
        let color_a = Color::new(1.0, 2.0, 3.0);
        let color_b = Color::new(2.0, 3.0, 5.0);

        let color_sum = Color::new(3.0, 5.0, 8.0);

        assert_eq!(color_a + color_b, color_sum);
    }


    #[test]
    fn test_subtract_vectors() {
        let color_a = Color::new(1.0, 2.0, 3.0);
        let color_b = Color::new(2.0, 3.0, 5.0);

        let color_sum = Color::new(-1.0, -1.0, -2.0);

        assert_eq!(color_a - color_b, color_sum);
    }


    #[test]
    fn test_scalar_multiplication() {
        let color_a = Color::new(1.0, 2.0, 3.0);
        let result = Color::new(3.0, 6.0, 9.0);

        assert_eq!(3.0 * color_a, result);
        assert_eq!(color_a * 3.0, result);
    }


    #[test]
    fn test_scalar_div() {
        let color_a = Color::new(3.0, 6.0, 9.0);
        let result = Color::new(1.0, 2.0, 3.0);

        assert_eq!(color_a / 3.0, result);
    }


    #[test]
    fn test_magnitude() {
        let color_a = Color::new(3.0, 6.0, 9.0);

        assert_eq!(color_a.magnitude(), 11.224972);
    }

    #[test]
    fn test_normalization() {
        let color_a = Color::new(3.0, 6.0, 9.0);
        assert_eq!(color_a.normalize().magnitude(), 1.0);
    }


    #[test]
    fn test_dot() {
        let color_a = Color::new(1.0, 2.0, 3.0);
        let color_b = Color::new(2.0, 3.0, 4.0);
        assert_eq!(color_a.dot(&color_b), 20.0);
    }

    #[test]
    fn test_color_product() {
        let color_a = Color::new(1.0, 0.2, 1.0);
        let color_b = Color::new(0.9, 1.0, 2.0);

        let color_c = Color::new(0.9, 0.2, 2.0);

        assert_eq!(color_a * color_b, color_c);
    }




}