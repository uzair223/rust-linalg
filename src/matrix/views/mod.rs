use crate::vector::Vector;

use super::Matrix;

#[allow(dead_code)]
pub trait View {
    type Index;

    fn shape(&self) -> (usize, usize);

    fn matrix(&self) -> &Matrix;

    fn to_matrix_index(&self, view_index: Self::Index) -> [usize; 2];
    
    fn index(&self, index: Self::Index) -> &f64 {
        &self.matrix()[self.to_matrix_index(index)]
    }
}

pub trait ViewMut: View {
    fn index_mut(&mut self, index: Self::Index) -> &mut f64 {
        #[allow(invalid_reference_casting)]
        let matrix: &mut Matrix = unsafe { &mut *(self.matrix() as *const Matrix as *mut Matrix) };
        &mut matrix[self.to_matrix_index(index)]
    }
}

#[allow(dead_code)]
pub trait ViewToMatrix: View<Index=[usize; 2]> {
    fn to_matrix(&self) -> Matrix {
        let (rows, cols) = self.shape();
        let mut matrix = Matrix::empty(0., rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                matrix[[i, j]] = *self.index([i, j])
            }
        }
        matrix
    }
}

#[allow(dead_code)]
pub trait ViewToVector: View<Index=usize> {
    fn size(&self) -> usize;
    
    fn to_vector(&self) -> Vector {
        Vector::new((0..self.size()).map(|i| *self.index(i)).collect())
    }
}

mod slice;
mod transpose;
mod row_column;