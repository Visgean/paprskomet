


pub struct M {
    data: Vec<Vec<f32>>,
    rows: usize,
    columns: usize
}

impl M {
    pub fn new(data: Vec<Vec<f32>>) -> Result<M, String> {
        // verify that every row has the same number of elements
        let rows = data.len();

        if rows < 1 {
            return Err("Cant have empty matrix!".into())
        }

        let columns = data[0].len();
        if columns < 1 {
            return Err("Cant have empty matrix!".into())
        }

        Ok(
            M {
                data,
                columns,
                rows,
        })
    }
}