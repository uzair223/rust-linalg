use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};

use super::{View, ViewMut, ViewToMatrix};
use crate::matrix::Matrix;

pub struct MatrixSliceView<'a> {
    matrix: &'a Matrix,
    offset: (usize, usize),
    shape: (usize, usize),
}

pub struct MatrixSliceViewMut<'a> {
    matrix: &'a Matrix,
    offset: (usize, usize),
    shape: (usize, usize),
}


pub trait Slice {
    fn begin(&self) -> usize;
    fn end(&self, max: usize) -> usize;
    #[inline(always)]
    fn size(&self, max: usize) -> usize {
        self.end(max) - self.begin()
    }
}

impl Slice for usize {
    #[inline(always)]
    fn begin(&self) -> usize {
        *self
    }

    #[inline(always)]
    fn end(&self, _: usize) -> usize {
        self + 1
    }

    #[inline(always)]
    fn size(&self, _: usize) -> usize {
        1
    }
}

impl Slice for Range<usize> {
    #[inline(always)]
    fn begin(&self) -> usize {
        self.start
    }
    
    #[inline(always)]
    fn end(&self, _: usize) -> usize {
        self.end
    }
}

impl Slice for RangeFrom<usize> {
    #[inline(always)]
    fn begin(&self) -> usize {
        self.start
    }

    #[inline(always)]
    fn end(&self, max: usize) -> usize {
        max
    }
}

impl Slice for RangeTo<usize> {
    #[inline(always)]
    fn begin(&self) -> usize {
        0
    }

    #[inline(always)]
    fn end(&self, _: usize) -> usize {
        self.end
    }
}

impl Slice for RangeFull {
    #[inline(always)]
    fn begin(&self) -> usize {
        0
    }

    #[inline(always)]
    fn end(&self, max: usize) -> usize {
        max
    }
}

impl Matrix {
    pub fn slice<R: Slice, C: Slice>(
        &self,
        (r, c): (R, C),
    ) -> MatrixSliceView {
        MatrixSliceView {
            matrix: self,
            offset: (r.begin(), c.begin()),
            shape: (r.size(self.shape.0), c.size(self.shape.1)),
        }
    }

    pub fn slice_mut<R: Slice, C: Slice>(
        &mut self,
        (r, c): (R, C),
    ) -> MatrixSliceViewMut {
        MatrixSliceViewMut {
            matrix: self,
            offset: (r.begin(), c.begin()),
            shape: (r.size(self.shape.0), c.size(self.shape.1)),
        }
    }
}

impl View for MatrixSliceView<'_> {
    type Index = [usize; 2];

    fn matrix(&self) -> &Matrix {
        self.matrix
    }

    fn shape(&self) -> (usize, usize) {
        self.shape
    }

    fn to_matrix_index(&self, [i, j]: [usize; 2]) -> [usize; 2] {
        [i + self.offset.0, j + self.offset.1]
    }
}

impl View for MatrixSliceViewMut<'_> {
    type Index = [usize; 2];

    fn matrix(&self) -> &Matrix {
        self.matrix
    }

    fn shape(&self) -> (usize, usize) {
        self.shape
    }

    fn to_matrix_index(&self, [i, j]: [usize; 2]) -> [usize; 2] {
        [i + self.offset.0, j + self.offset.1]
    }
}

impl ViewToMatrix for MatrixSliceView<'_> {}
impl ViewToMatrix for MatrixSliceViewMut<'_> {}
impl ViewMut for MatrixSliceViewMut<'_> {
    fn assign(&mut self, new: &[f64]) -> () {
        assert_eq!(
            self.shape.0 * self.shape.1,
            new.len(),
            "shape mismatch: {} cannot be reshaped into {}x{}",
            new.len(),
            self.shape.0,
            self.shape.1
        );
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                self[[i, j]] = new[i * self.shape.1 + j];
            }
        }
    }
}

impl Index<[usize; 2]> for MatrixSliceView<'_> {
    type Output = f64;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        View::index(self, index)
    }
}

impl Index<[usize; 2]> for MatrixSliceViewMut<'_> {
    type Output = f64;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        View::index(self, index)
    }
}

impl IndexMut<[usize; 2]> for MatrixSliceViewMut<'_> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        ViewMut::index_mut(self, index)
    }
}

impl MatrixSliceView<'_> {
    pub fn to_matrix(&self) -> Matrix {
        ViewToMatrix::to_matrix(self)
    }
}

impl MatrixSliceViewMut<'_> {
    pub fn to_matrix(&self) -> Matrix {
        ViewToMatrix::to_matrix(self)
    }

    pub fn assign(&mut self, new: &[f64]) {
        ViewMut::assign(self, new)
    }
}
