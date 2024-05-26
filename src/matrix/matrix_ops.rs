use super::Matrix;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Matrix {
    pub fn equals(self, other: Self, eps: f64) -> bool {
        let dist = self - other;
        for d in dist.iter() {
            if d.abs() > eps { return false; }
        }
        true
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
    fn ne(&self, other: &Self) -> bool {
        self.data != other.data
    }
}

impl Matrix {
    pub fn dot(self, rhs: &Self) -> Self {
        if self.shape.1 != rhs.shape.0 {
            panic!(
                "shape mismatch: dot product cannot be between {:?} and {:?}",
                self.shape, rhs.shape
            )
        }
        let mut dot = Matrix::empty(0., self.shape.0, rhs.shape.1);
        for i in 0..self.shape.0 {
            for j in 0..rhs.shape.1 {
                for k in 0..self.shape.1 {
                    dot[[i, j]] += self[[i, k]] * rhs[[k, j]];
                }
            }
        }
        dot
    }
}

impl Add for Matrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        _check_same_shape!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a + b).collect(),
            shape: self.shape,
        }
    }
}

impl Sub for Matrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        _check_same_shape!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect(),
            shape: self.shape,
        }
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        _check_same_shape!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect(),
            shape: self.shape,
        }
    }
}

impl Div for Matrix {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        _check_same_shape!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a / b).collect(),
            shape: self.shape,
        }
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        _check_same_shape!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a += b)
            .collect()
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        _check_same_shape!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a -= b)
            .collect()
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        _check_same_shape!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a *= b)
            .collect()
    }
}

impl DivAssign for Matrix {
    fn div_assign(&mut self, rhs: Self) {
        _check_same_shape!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a /= b)
            .collect()
    }
}
