use std::io::empty;
use crate::utils::float_compare;
use std::iter::zip;

use std::ops::{Mul};

#[derive(Debug)]
struct M {
    data: Vec<Vec<f32>>,
    pub rows: usize,
    pub columns: usize,
}


impl M {
    pub fn new(data: Vec<Vec<f32>>) -> Result<M, String> {
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
            return Err("Right now only 4x4 matrices are supported, sorry!".into());
        }


        for row in &data {
            if row.len() != columns {
                return Err("Illegal matrix size".into());
            }
        }


        Ok(
            M {
                data,
                columns,
                rows,
            })
    }

    pub fn empty_matrix(m: usize, n: usize) -> M {
        M {
            data: vec![vec![0.0; n]; m],
            columns: n,
            rows: n,
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

    pub fn column(&self, n: usize) -> Vec<f32> {
        let mut result = vec![0.0; self.rows];

        for i in 0..self.rows {
            result[i] = self.data[i][n];
        }

        result
    }

    pub fn row(&self, m: usize) -> Vec<f32> {
        self.data[m].clone()
    }


    pub fn get(&self, i: usize, j: usize) -> Result<f32, String> {
        if self.columns - 1 < j || self.rows - 1 < i {
            return Err("Illegal access".into());
        }
        Ok(self.data[i][j])
    }

    pub fn set(&mut self, i: usize, j: usize, x: f32){
        self.data[i][j] = x;
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


impl Mul<M> for M {
    type Output = M;

    fn mul(self, rhs: M) -> Self::Output {
        let rows = self.rows;
        let columns = rhs.columns;

        let mut result = M::empty_matrix(rows, columns);

        for row in 0..rows {
            let row_vector = self.row(row);
            
            for col in 0..columns {
                let x = dot_p(&row_vector, rhs.column(col));
                result.set(row, col, x);
            }
        }


        result

    }
}

fn dot_p(v1: &Vec<f32>, v2: Vec<f32>) -> f32 {
    zip(v1, v2).map(|(x, y) | x * y).sum()
}


#[cfg(test)]
mod tests {
    use crate::matrix::M;

    #[test]
    fn test_matrix_4x4_init() {
        let m1 = M::new(
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.5, 6.5, 7.5, 8.5],
                vec![9.0, 10.0, 11.0, 12.0],
                vec![13.5, 14.5, 15.5, 16.5],
            ]
        ).unwrap();
        assert_eq!(m1.rows, 4);
        assert_eq!(m1.columns, 4);
        assert_eq!(m1.get(1, 2).unwrap(), 7.5)
    }

    #[test]
    fn test_matrix_3x3_init() {
        let m1 = M::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![5.5, 6.5, 7.5],
                vec![9.0, 10.0, 11.0],
            ]
        ).unwrap();
        assert_eq!(m1.rows, 3);
        assert_eq!(m1.columns, 3);
        assert_eq!(m1.get(1, 2).unwrap(), 7.5)
    }


    #[test]
    #[should_panic]
    fn test_illegal_size() {
        M::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![5.5, 6.5, 7.5],
                vec![9.0, 10.0, 11.0, 12.0],
            ]
        ).unwrap();
    }

    #[test]
    fn test_matrix_compare() {
        let m1 = M::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![5.5, 6.5, 7.5],
                vec![9.0, 10.0, 11.0],
            ]
        ).unwrap();

        let m2 = M::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![5.5, 6.5, 7.5],
                vec![9.0, 10.0, 11.0],
            ]
        ).unwrap();

        let m3 = M::new(
            vec![
                vec![2.0, 2.0, 3.0],
                vec![5.5, 6.5, 7.5],
                vec![9.0, 10.0, 11.0],
            ]
        ).unwrap();


        assert!(m1 == m2);
        assert!(m1 != m3);
    }


    #[test]
    fn test_matrix_multiplication() {
        let m1 = M::new(
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ]
        ).unwrap();

        let m2 = M::new(
            vec![
                vec![-2.0, 1.0, 2.0, 3.0],
                vec![3.0, 2.0, 1.0, -1.0],
                vec![4.0, 3.0, 6.0, 5.0],
                vec![1.0, 2.0, 7.0, 8.0],
            ]
        ).unwrap();

        let m3 = M::new(
            vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ]
        ).unwrap();

        assert_eq!(m1 * m2, m3)
    }


    #[test]
    fn test_ident() {
        let m1 = M::new(
            vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ]
        ).unwrap();


        assert_eq!(m1, M::ident(4));
    }
}