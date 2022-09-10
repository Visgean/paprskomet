use crate::utils::float_compare;
use std::iter::zip;

use crate::vectors::Tuple;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct M {
    data: Vec<Vec<f64>>,
    pub rows: usize,
    pub columns: usize,
}

impl M {
    pub fn new(data: Vec<Vec<f64>>) -> Result<M, String> {
        // verify that every row has the same number of elements
        let rows = data.len();

        if rows < 1 {
            return Err("Cant have empty matrix!".into());
        }

        let columns = data[0].len();
        if columns < 1 {
            return Err("Cant have empty matrix!".into());
        }

        if columns > 4 || rows > 4 {
            return Err(
                "Right now only 4x4 matrices are supported, sorry!".into()
            );
        }

        for row in &data {
            if row.len() != columns {
                return Err("Illegal matrix size".into());
            }
        }

        Ok(M {
            data,
            columns,
            rows,
        })
    }

    pub fn empty_matrix(m: usize, n: usize) -> M {
        M {
            data: vec![vec![0.0; n]; m],
            columns: n,
            rows: m,
        }
    }

    pub fn ident(n: usize) -> M {
        let mut data = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    data[i][j] = 1.0
                }
            }
        }

        M {
            data,
            columns: n,
            rows: n,
        }
    }

    pub fn column(&self, n: usize) -> Vec<f64> {
        let mut result = vec![0.0; self.rows];

        for i in 0..self.rows {
            result[i] = self.data[i][n];
        }

        result
    }

    pub fn row(&self, m: usize) -> Vec<f64> {
        self.data[m].clone()
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, x: f64) {
        self.data[i][j] = x;
    }

    pub fn transpose(&self) -> M {
        let mut result = M::empty_matrix(self.rows, self.columns);

        for i in 0..self.rows {
            for j in 0..self.columns {
                result.set(j, i, self.data[i][j])
            }
        }
        result
    }

    pub fn det(&self) -> f64 {
        // base case:

        if self.columns == 2 && self.rows == 2 {
            return self.get(0, 0) * self.get(1, 1)
                - self.get(0, 1) * self.get(1, 0);
        }

        let mut det = 0.0;
        for j in 0..self.columns {
            det += self.get(0, j) * self.cofactor(0, j);
        }
        det
    }

    pub fn invertible(&self) -> bool {
        !float_compare(self.det(), 0.0)
    }

    pub fn submatrix(&self, row: usize, col: usize) -> M {
        let mut m = self.clone();

        for i in 0..self.rows {
            m.data[i].remove(col);
        }

        m.data.remove(row);
        m.rows = m.rows - 1;
        m.columns = m.columns - 1;
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        return self.submatrix(row, col).det();
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let r = self.minor(row, col);

        if (row + col) % 2 == 1 {
            return -r;
        }

        r
    }

    pub fn inverse(&self) -> M {
        let d = self.det();
        if float_compare(d, 0.0) {
            panic!("Cant invert non-invertible matrix!!");
        }

        let mut r = M::empty_matrix(self.rows, self.columns);

        for row in 0..self.rows {
            for col in 0..self.columns {
                let cof = self.cofactor(row, col);
                r.set(col, row, cof / d)
            }
        }

        r
    }
}

impl PartialEq for M {
    fn eq(&self, other: &Self) -> bool {
        if !(self.columns == other.columns && self.rows == other.rows) {
            return false;
        }

        let flat_self = self.data.iter().flatten();
        let flat_other = other.data.iter().flatten();

        zip(flat_self, flat_other).all(|(a, b)| float_compare(*a, *b))
    }
}

fn dot_p(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    zip(v1, v2).map(|(x, y)| x * y).sum()
}

impl Mul<M> for M {
    type Output = M;

    fn mul(self, rhs: M) -> Self::Output {
        let rows = self.rows;
        let columns = rhs.columns;

        let mut result = M::empty_matrix(rows, columns);

        for row in 0..rows {
            let row_vector = self.row(row);

            for col in 0..columns {
                let x = dot_p(&row_vector, &rhs.column(col));
                result.set(row, col, x);
            }
        }
        result
    }
}

impl Mul<&M> for &M {
    type Output = M;

    fn mul(self, rhs: &M) -> Self::Output {
        let rows = self.rows;
        let columns = rhs.columns;

        let mut result = M::empty_matrix(rows, columns);

        for row in 0..rows {
            let row_vector = self.row(row);

            for col in 0..columns {
                let x = dot_p(&row_vector, &rhs.column(col));
                result.set(row, col, x);
            }
        }
        result
    }
}

impl Mul<Tuple> for M {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        // convert vector to one column matrix..

        let m_v =
            M::new(vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![rhs.w]])
                .unwrap();

        let r = self * m_v;

        Tuple::new(r.get(0, 0), r.get(1, 0), r.get(2, 0), r.get(3, 0))
    }
}

impl Mul<Tuple> for &M {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        // convert vector to one column matrix..

        let m_v =
            M::new(vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![rhs.w]])
                .unwrap();

        let r = self * &m_v;

        Tuple::new(r.get(0, 0), r.get(1, 0), r.get(2, 0), r.get(3, 0))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::M;
    use crate::vectors::Tuple;

    #[test]
    fn test_matrix_4x4_init() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ])
        .unwrap();
        assert_eq!(m1.rows, 4);
        assert_eq!(m1.columns, 4);
        assert_eq!(m1.get(1, 2), 7.5)
    }

    #[test]
    fn test_matrix_3x3_init() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![5.5, 6.5, 7.5],
            vec![9.0, 10.0, 11.0],
        ])
        .unwrap();
        assert_eq!(m1.rows, 3);
        assert_eq!(m1.columns, 3);
        assert_eq!(m1.get(1, 2), 7.5)
    }

    #[test]
    #[should_panic]
    fn test_illegal_size() {
        M::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![5.5, 6.5, 7.5],
            vec![9.0, 10.0, 11.0, 12.0],
        ])
        .unwrap();
    }

    #[test]
    fn test_matrix_compare() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![5.5, 6.5, 7.5],
            vec![9.0, 10.0, 11.0],
        ])
        .unwrap();

        let m2 = M::new(vec![
            vec![1.0, 2.0, 3.0],
            vec![5.5, 6.5, 7.5],
            vec![9.0, 10.0, 11.0],
        ])
        .unwrap();

        let m3 = M::new(vec![
            vec![2.0, 2.0, 3.0],
            vec![5.5, 6.5, 7.5],
            vec![9.0, 10.0, 11.0],
        ])
        .unwrap();

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ])
        .unwrap();

        let m2 = M::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ])
        .unwrap();

        let m3 = M::new(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ])
        .unwrap();

        assert_eq!(m1 * m2, m3)
    }

    #[test]
    fn test_tuple_multiplication() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();

        let v1 = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let v2 = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(m1 * v1, v2)
    }

    #[test]
    fn test_ident() {
        let m1 = M::new(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();

        assert_eq!(m1, M::ident(4));
    }

    #[test]
    fn test_ident_multiplication() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ])
        .unwrap();

        let m2 = &m1 * &M::ident(4);

        assert_eq!(m2, m1)
    }

    #[test]
    fn test_transpose() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();

        let m1_t = M::new(vec![
            vec![1.0, 2.0, 8.0, 0.0],
            vec![2.0, 4.0, 6.0, 0.0],
            vec![3.0, 4.0, 4.0, 0.0],
            vec![4.0, 2.0, 1.0, 1.0],
        ])
        .unwrap();

        assert_eq!(m1.transpose(), m1_t);
    }

    #[test]
    fn test_small_det() {
        let m1 = M::new(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]).unwrap();

        assert_eq!(m1.det(), 17.0);
    }

    #[test]
    fn test_submatrix() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();

        let subm = M::new(vec![
            vec![4.0, 4.0, 2.0],
            vec![6.0, 4.0, 1.0],
            vec![0.0, 0.0, 1.0],
        ])
        .unwrap();

        assert_eq!(m1.submatrix(0, 0), subm);
    }

    #[test]
    fn test_submatrix_2() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();

        let subm = M::new(vec![
            vec![1.0, 3.0, 4.0],
            vec![8.0, 4.0, 1.0],
            vec![0.0, 0.0, 1.0],
        ])
        .unwrap();
        assert_eq!(m1.submatrix(1, 1), subm);
    }

    #[test]
    fn test_submatrix_3() {
        let m1 = M::new(vec![
            vec![1.0, 3.0, 4.0],
            vec![8.0, 4.0, 1.0],
            vec![0.0, 0.0, 1.0],
        ])
        .unwrap();

        let subm = M::new(vec![vec![1.0, 4.0], vec![0.0, 1.0]]).unwrap();
        assert_eq!(m1.submatrix(1, 1), subm);
    }

    #[test]
    fn test_minor() {
        let m1 = M::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ])
        .unwrap();

        assert_eq!(m1.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let m1 = M::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ])
        .unwrap();

        assert_eq!(m1.minor(0, 0), -12.0);
        assert_eq!(m1.cofactor(0, 0), -12.0);

        // test for sign change
        assert_eq!(m1.minor(1, 0), 25.0);
        assert_eq!(m1.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_det_3x3() {
        let m1 = M::new(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ])
        .unwrap();

        assert_eq!(m1.cofactor(0, 0), 56.0);
        assert_eq!(m1.cofactor(0, 1), 12.0);
        assert_eq!(m1.cofactor(0, 2), -46.0);

        assert_eq!(m1.det(), -196.0);
    }

    #[test]
    fn test_det_4x4() {
        let m1 = M::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ])
        .unwrap();

        assert_eq!(m1.cofactor(0, 0), 690.0);
        assert_eq!(m1.cofactor(0, 1), 447.0);
        assert_eq!(m1.cofactor(0, 2), 210.0);
        assert_eq!(m1.cofactor(0, 3), 51.0);

        assert_eq!(m1.det(), -4071.0);
    }

    #[test]
    fn test_invertible() {
        let m1 = M::new(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ])
        .unwrap();
        assert!(m1.invertible());
        assert_eq!(m1.det(), -2120.0);
    }

    #[test]
    fn test_non_invertible() {
        let m1 = M::new(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ])
        .unwrap();
        assert!(!m1.invertible());
        assert_eq!(m1.det(), 0.0);
    }

    #[test]
    fn test_inverse() {
        let m1 = M::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ])
        .unwrap();

        let m1_inverse = m1.inverse();

        // Then determinant(A) = 532
        assert_eq!(m1.det(), 532.0);
        // And cofactor(A, 2, 3) = -160
        assert_eq!(m1.cofactor(2, 3), -160.0);
        // And B[3,2] = -160/532
        assert_eq!(m1_inverse.get(3, 2), -160.0 / 532.0);
        // And cofactor(A, 3, 2) = 105
        assert_eq!(m1.cofactor(3, 2), 105.0);
        // And B[2,3] = 105/532
        assert_eq!(m1_inverse.get(2, 3), 105.0 / 532.0);

        let m1_inverse_exp = M::new(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ])
        .unwrap();

        assert_eq!(m1_inverse, m1_inverse_exp);
    }

    #[test]
    fn test_inverse_2() {
        let m1 = M::new(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ])
        .unwrap();

        let m1_inverse = m1.inverse();
        let m1_inverse_exp = M::new(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ])
        .unwrap();

        assert_eq!(m1_inverse, m1_inverse_exp);
    }

    #[test]
    fn test_inverse_3() {
        let m1 = M::new(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ])
        .unwrap();

        let m1_inverse = m1.inverse();
        let m1_inverse_exp = M::new(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ])
        .unwrap();

        assert_eq!(m1_inverse, m1_inverse_exp);
    }

    #[test]
    fn test_inverse_mult() {
        let m_a = M::new(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ])
        .unwrap();

        let m_b = M::new(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ])
        .unwrap();
        let c = &m_a * &m_b;
        assert_eq!(m_a, c * m_b.inverse());
    }

    #[test]
    fn test_inverse_itself() {
        let m_a = M::new(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ])
        .unwrap();

        let c = &m_a * &m_a.inverse();
        assert_eq!(c, M::ident(4));
    }
}
