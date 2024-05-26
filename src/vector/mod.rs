use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
    pub size: usize,
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
