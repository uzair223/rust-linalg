use super::Matrix;

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();

        if (rows | cols) == 0 {
            panic!("matrix must not be empty")
        }
        for i in 1..rows {
            if data[i].len() != cols {
                panic!(
                    "all rows must be of equal length: {} != {}",
                    data[i].len(),
                    cols
                )
            }
        }

        Matrix {
            data: data.concat(),
            shape: (rows, cols),
        }
    }

    pub fn from(vec: Vec<f64>, (rows, cols): (usize, usize)) -> Matrix {
        let size = vec.len();
        if rows * cols != size {
            panic!("vector {} cannot be reshaped into {:?}", size, (rows, cols))
        }

        Matrix {
            data: vec,
            shape: (rows, cols),
        }
    }

    pub fn empty(x: f64, rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: vec![x; rows * cols],
            shape: (rows, cols),
        }
    }

    pub fn eye(n: usize) -> Matrix {
        let mut data = vec![0.; n * n];
        for i in 0..n {
            data[i * n + i] = 1.
        }
        Matrix {
            data,
            shape: (n, n),
        }
    }
}
