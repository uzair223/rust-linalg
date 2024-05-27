use core::ops::{Deref, DerefMut};

use crate::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
    pub size: usize,
}

pub enum VectorType {
    Column = 0,
    Row = 1,
}

impl Vector {
    pub fn new(data: Vec<f64>) -> Self {
        let size = data.len();
        Vector { data, size }
    }

    pub fn empty(x: f64, size: usize) -> Self {
        Vector {
            data: vec![x; size],
            size,
        }
    }

    pub fn to_matrix(&self, t: VectorType) -> Matrix {
        match t {
            VectorType::Column => Matrix::from(self.data.clone(), (self.size, 1)),
            VectorType::Row => Matrix::from(self.data.clone(), (1, self.size))
        }
    }
}

impl Deref for Vector {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

macro_rules! _check_same_size {
    ($a:expr, $b:expr) => {
        if ($a.size != $b.size) {
            panic!("size mismatch: {:?} != {:?}", $a.size, $b.size)
        }
    };
}

mod ops;
mod scalar_ops;
mod vector_ops;

#[cfg(test)]
mod tests;
