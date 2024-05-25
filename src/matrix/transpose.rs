use core::ops::{Deref, DerefMut, Index, IndexMut};

use super::Matrix;

pub struct MatrixTranposeView<'a> {
    matrix: &'a Matrix,
}

pub struct MutableMatrixTranposeView<'a> {
    matrix: &'a Matrix,
}

impl Matrix {
    pub fn tranpose(&self) -> MatrixTranposeView {
        MatrixTranposeView { matrix: self }
    }

    pub fn tranpose_mut(&mut self) -> MutableMatrixTranposeView {
        MutableMatrixTranposeView { matrix: self }
    }
}

impl MatrixTranposeView<'_> {
    pub fn to_matrix(&self) -> Matrix {
        let mut matrix = Matrix::empty(0., self.shape.1, self.shape.0);
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                matrix[[i, j]] = self.matrix[[j, i]]
            }
        }
        matrix
    }
}

/* immutable view */
impl<'a> Deref for MatrixTranposeView<'a> {
    type Target = Matrix;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl<'a> Index<[usize; 2]> for MatrixTranposeView<'a> {
    type Output = f64;

    fn index(&self, [j, i]: [usize; 2]) -> &Self::Output {
        &self.matrix[[i, j]]
    }
}

/* mutable view */
impl<'a> Index<[usize; 2]> for MutableMatrixTranposeView<'a> {
    type Output = f64;

    fn index(&self, [j, i]: [usize; 2]) -> &Self::Output {
        &self.matrix[[i, j]]
    }
}

impl<'a> Deref for MutableMatrixTranposeView<'a> {
    type Target = Matrix;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl<'a> DerefMut for MutableMatrixTranposeView<'a> {
    #[allow(invalid_reference_casting)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.matrix as *const Matrix as *mut Matrix) }
    }
}

impl<'a> IndexMut<[usize; 2]> for MutableMatrixTranposeView<'a> {
    fn index_mut(&mut self, [j, i]: [usize; 2]) -> &mut Self::Output {
        &mut self.deref_mut()[[i, j]]
    }
}
