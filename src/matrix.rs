use crate::utils::float_compare;
use std::iter::zip;


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

    pub fn get(&self, i: usize, j: usize) -> Result<f32, String> {
        if self.columns - 1 < j || self.rows - 1 < i {
            return Err("Illegal access".into());
        }
        Ok(self.data[i][j])
    }
}


impl PartialEq for M {
    fn eq(&self, other: &Self) -> bool {
        if !(self.columns == other.columns && self.rows == other.rows) {
            return false;
        }

        let flat_self = self.data.iter().flatten();
        let flat_other = other.data.iter().flatten();

        for (a, b) in zip(flat_self, flat_other) {
            if !float_compare(*a, *b) {
                return false;
            }
        }
        true
    }
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
}