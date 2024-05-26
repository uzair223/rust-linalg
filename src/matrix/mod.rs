use core::fmt;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    pub shape: (usize, usize),
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.data
                .chunks(self.shape.1)
                .map(|r| format!("{:?}", r))
                .join("\n ")
        )
    }
}

macro_rules! _check_same_shape {
    ($a:expr, $b:expr) => {
        if ($a.shape != $b.shape) {
            panic!("shape mismatch: {:?} != {:?}", $a.shape, $b.shape)
        }
    };
}

macro_rules! _check_square {
    ($a:expr) => {
        if ($a.shape.0 != $a.shape.1) {
            panic!("matrix must be square: {:?}", $a.shape)
        }
    };
}

mod constructors;
mod index_ops;
mod matrix_ops;
mod ops;
mod scalar_ops;
mod views;

#[cfg(test)]
mod tests;
