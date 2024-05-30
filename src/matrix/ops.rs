use super::Matrix;

impl Matrix {
    pub fn get_minor(&self, i: usize, j: usize) -> Matrix {
        let mut data = self.data.clone();
        // deleting row
        data.drain(i * self.shape.1..(i + 1) * self.shape.1);
        // deleting column
        for i in 0..self.shape.0 - 1 {
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

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        let row_a = a * self.shape.1..(a + 1) * self.shape.1;
        let row_b = b * self.shape.1..(b + 1) * self.shape.1;
        for (i, j) in itertools::zip_eq(row_a, row_b) {
            self.data.swap(i, j)
        }
    }

    pub fn gauss(&self, singular_error: bool) -> Matrix {
        let (rows, cols) = self.shape;
        let mut mat = self.clone();
        let mut lead = 0;
        for r in 0..rows {
            if lead >= cols {
                break;
            }
            let mut i = r;
            while i < rows && mat[[i, lead]] == 0. {
                i += 1;
            }
            if i == rows {
                if singular_error {
                    panic!("matrix is singular")
                }
                break;
            }
            mat.swap_rows(i, r);
            let divisor = mat[[r, lead]];
            if divisor == 0. {
                if singular_error {
                    panic!("matrix is singular")
                }
                break;
            }
            for j in 0..cols {
                mat[[r, j]] /= divisor;
            }
            for i in 0..rows {
                if i != r {
                    let factor = mat[[i, lead]];
                    for j in 0..cols {
                        mat[[i, j]] -= factor * mat[[r, j]];
                    }
                }
            }
            lead += 1;
        }
        mat
    }

    pub fn inverse(&self) -> Matrix {
        _check_square!(&self);
        let n = self.shape.0;
        let eye = Self::eye(n);

        let mut vec: Vec<f64> = vec![];
        for i in 0..n {
            vec.extend(self.data[i * n..(i + 1) * n].iter());
            vec.extend(eye.data[i * n..(i + 1) * n].iter())
        }
        let aug = Matrix::from(vec, (n, n * 2)).gauss(true);

        let mut data: Vec<f64> = vec![];
        for i in 0..n {
            data.extend(aug.data[i * n * 2..(i + 1) * n * 2][n..].iter());
        }
        Self {
            data,
            shape: self.shape,
        }
    }

    pub fn is_square(&self) -> bool {
        self.shape.0 == self.shape.1
    }

    pub fn is_upper_triangular(&self, eps: f64) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 1..self.shape.0 {
            for j in 0..i {
                if self[[i, j]] > eps {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_lower_triangular(&self, eps: f64) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 0..self.shape.0 - 1 {
            for j in i + 1..self.shape.0 {
                if self[[i, j]] > eps {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_triangular(&self, eps: f64) -> bool {
        self.is_upper_triangular(eps) | self.is_lower_triangular(eps)
    }

    pub fn diag(&self) -> Vec<f64> {
        (0..usize::min(self.shape.0, self.shape.1))
            .map(|i| self[[i, i]])
            .collect()
    }
}
