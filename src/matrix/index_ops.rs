use super::Matrix;
use core::ops::{Deref, DerefMut, Index, IndexMut};

impl Deref for Matrix {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        if i >= self.shape.0 {
            panic!("row index {} out of bounds", i)
        }
        if j >= self.shape.1 {
            panic!("col index {} out of bounds", j)
        }
        &self.data[i * self.shape.1 + j]
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, [i, j]: [usize; 2]) -> &mut Self::Output {
        &mut self.data[i * self.shape.1 + j]
    }
}
