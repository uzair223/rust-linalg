use super::Matrix;

#[allow(dead_code)]
pub trait View {
    fn shape(&self) -> (usize, usize);
    fn matrix(&self) -> &Matrix;

    fn to_matrix_index(&self, view_index: [usize; 2]) -> [usize; 2];

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

    fn index(&self, index: [usize; 2]) -> &f64 {
        &self.matrix()[self.to_matrix_index(index)]
    }
}

pub trait ViewMut: View {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut f64 {
        #[allow(invalid_reference_casting)]
        let matrix: &mut Matrix = unsafe { &mut *(self.matrix() as *const Matrix as *mut Matrix) };
        &mut matrix[self.to_matrix_index(index)]
    }
}

mod slice;
mod transpose;
