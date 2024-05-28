use core::ops::{Index, IndexMut, RangeBounds};

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

fn range<T: RangeBounds<usize>>(min: usize, max: usize, range: T) -> (usize, usize) {
    let start = match range.start_bound() {
        std::ops::Bound::Included(&start) => start,
        std::ops::Bound::Excluded(&start) => start,
        std::ops::Bound::Unbounded => min,
    };

    let end = match range.end_bound() {
        std::ops::Bound::Included(&end) => end,
        std::ops::Bound::Excluded(&end) => end,
        std::ops::Bound::Unbounded => max,
    };

    return (start, end);
}

impl Matrix {
    pub fn slice<R: RangeBounds<usize>, C: RangeBounds<usize>>(
        &self,
        (row_range, col_range): (R, C),
    ) -> MatrixSliceView {
        let (row_start, row_end) = range(0, self.shape.0, row_range);
        let (col_start, col_end) = range(0, self.shape.1, col_range);

        MatrixSliceView {
            matrix: self,
            offset: (row_start, col_start),
            shape: (row_end - row_start, col_end - col_start),
        }
    }

    pub fn slice_mut<R: RangeBounds<usize>, C: RangeBounds<usize>>(
        &mut self,
        (row_range, col_range): (R, C),
    ) -> MatrixSliceViewMut {
        let (row_start, row_end) = range(0, self.shape.0, row_range);
        let (col_start, col_end) = range(0, self.shape.1, col_range);

        MatrixSliceViewMut {
            matrix: self,
            offset: (row_start, col_start),
            shape: (row_end - row_start, col_end - col_start),
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
