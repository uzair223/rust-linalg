use std::ops::{Index, IndexMut};

use crate::{matrix::Matrix, vector::VectorType};
use super::{View, ViewMut, ViewToVector};

impl Matrix {
    pub fn row(&self, index: usize) -> MatrixRowColumnView {
        MatrixRowColumnView { matrix: self, index, t: VectorType::Row, size: self.shape.1 }
    }
    pub fn row_mut(&mut self, index: usize) -> MatrixRowColumnViewMut {
        MatrixRowColumnViewMut { matrix: self, index, t: VectorType::Row, size: self.shape.1 }
    }
    pub fn column(&self, index: usize) -> MatrixRowColumnView {
        MatrixRowColumnView { matrix: self, index, t: VectorType::Column, size: self.shape.0 }
    }
    pub fn column_mut(&mut self, index: usize) -> MatrixRowColumnViewMut {
        MatrixRowColumnViewMut { matrix: self, index, t: VectorType::Column, size: self.shape.0 }
    }

}


pub struct MatrixRowColumnView<'a> {
    matrix: &'a Matrix,
    index: usize,
    size: usize,
    t: VectorType
}

pub struct MatrixRowColumnViewMut<'a> {
    matrix: &'a Matrix,
    index: usize,
    size: usize,
    t: VectorType
}

impl View for MatrixRowColumnView<'_> {
    type Index = usize;

    fn shape(&self) -> (usize, usize) {
        match self.t {
            VectorType::Row => (1, self.size),
            VectorType::Column => (self.size, 1),
        }
    }
    
    fn matrix(&self) -> &Matrix {
        self.matrix
    }
    
    fn to_matrix_index(&self, view_index: usize) -> [usize; 2] {
        match self.t {
            VectorType::Row => [self.index, view_index],
            VectorType::Column => [view_index, self.index],
        }
    }
}

impl View for MatrixRowColumnViewMut<'_> {
    type Index = usize;

    fn shape(&self) -> (usize, usize) {
        match self.t {
            VectorType::Row => (1, self.matrix.shape.1),
            VectorType::Column => (self.matrix.shape.0, 1),
        }
    }

    fn matrix(&self) -> &Matrix {
        self.matrix
    }

    fn to_matrix_index(&self, view_index: usize) -> [usize; 2] {
        match self.t {
            VectorType::Row => [self.index, view_index],
            VectorType::Column => [view_index, self.index],
        }
    }
}

impl ViewToVector for MatrixRowColumnView<'_> {
    fn size(&self) -> usize {
        self.size
    }
}

impl ViewToVector for MatrixRowColumnViewMut<'_> {
    fn size(&self) -> usize {
        self.size
    }
}

impl ViewMut for MatrixRowColumnViewMut<'_> {}

impl Index<usize> for MatrixRowColumnView<'_> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        View::index(self, index)
    }
}

impl Index<usize> for MatrixRowColumnViewMut<'_> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        View::index(self, index)
    }
}

impl IndexMut<usize> for MatrixRowColumnViewMut<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        ViewMut::index_mut(self, index)
    }
}
