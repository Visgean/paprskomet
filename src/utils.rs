pub fn float_compare(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.0001
}
