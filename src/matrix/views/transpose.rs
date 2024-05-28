use core::ops::{Index, IndexMut};

use super::{View, ViewMut, ViewToMatrix};
use crate::matrix::Matrix;

impl Matrix {
    pub fn transpose(&self) -> MatrixTransposeView {
        MatrixTransposeView { matrix: self }
    }

    pub fn transpose_mut(&mut self) -> MatrixTransposeViewMut {
        MatrixTransposeViewMut { matrix: self }
    }
}

pub struct MatrixTransposeView<'a> {
    matrix: &'a Matrix,
}

pub struct MatrixTransposeViewMut<'a> {
    matrix: &'a Matrix,
}

impl View for MatrixTransposeView<'_> {
    type Index = [usize; 2];

    fn matrix(&self) -> &Matrix {
        self.matrix
    }

    fn shape(&self) -> (usize, usize) {
        (self.matrix.shape.1, self.matrix.shape.0)
    }

    fn to_matrix_index(&self, [i, j]: [usize; 2]) -> [usize; 2] {
        [j, i]
    }
}

impl View for MatrixTransposeViewMut<'_> {
    type Index = [usize; 2];

    fn matrix(&self) -> &Matrix {
        self.matrix
    }

    fn shape(&self) -> (usize, usize) {
        (self.matrix.shape.1, self.matrix.shape.0)
    }

    fn to_matrix_index(&self, [i, j]: [usize; 2]) -> [usize; 2] {
        [j, i]
    }
}

impl ViewToMatrix for MatrixTransposeView<'_> {}
impl ViewToMatrix for MatrixTransposeViewMut<'_> {}
impl ViewMut for MatrixTransposeViewMut<'_> {
    fn assign(&mut self, _: &[f64]) -> () {
        todo!()
    }
}

impl Index<[usize; 2]> for MatrixTransposeView<'_> {
    type Output = f64;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        View::index(self, index)
    }
}

impl Index<[usize; 2]> for MatrixTransposeViewMut<'_> {
    type Output = f64;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        View::index(self, index)
    }
}

impl IndexMut<[usize; 2]> for MatrixTransposeViewMut<'_> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        ViewMut::index_mut(self, index)
    }
}

impl MatrixTransposeView<'_> {
    pub fn to_matrix(&self) -> Matrix {
        ViewToMatrix::to_matrix(self)
    }
}

impl MatrixTransposeViewMut<'_> {
    pub fn to_matrix(&self) -> Matrix {
        ViewToMatrix::to_matrix(self)
    }

    pub fn assign(&mut self, new: &[f64]) {
        ViewMut::assign(self, new)
    }
}
