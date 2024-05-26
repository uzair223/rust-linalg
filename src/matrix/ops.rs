use super::Matrix;
use itertools::Itertools;

impl Matrix {
    pub fn get_minor(&self, i: usize, j: usize) -> Matrix {
        let mut data = self.data.clone();
        // deleting row
        data.drain(i * self.shape.1..(i + 1) * self.shape.1);
        // deleting column
        for i in 0..self.shape.0-1 {
            // column index change as we remove previous entries
            data.remove(i * self.shape.1 + j - i);
        }
        Matrix::from(data, (self.shape.0 - 1, self.shape.1 - 1))
    }

    pub fn det(&self) -> f64 {
        _check_square!(&self);
        if self.shape.0 == 2 {
            /*|a,b|
              |c,d| = ad - bc */
            return self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]];
        }
        let mut det = 0.;
        for j in 0..self.shape.1 {
            det += f64::powi(-1., j as i32) * self[[0, j]] * self.get_minor(0, j).det()
        }
        det
    }

}
