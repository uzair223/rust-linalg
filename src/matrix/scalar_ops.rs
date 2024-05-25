use super::Matrix;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/* scalar operations */
impl Add<f64> for Matrix {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            data: self.iter().map(|x| x + rhs).collect(),
            shape: self.shape,
        }
    }
}

impl Sub<f64> for Matrix {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            data: self.iter().map(|x| x - rhs).collect(),
            shape: self.shape,
        }
    }
}

impl Mul<f64> for Matrix {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            data: self.iter().map(|x| x * rhs).collect(),
            shape: self.shape,
        }
    }
}

impl Div<f64> for Matrix {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            data: self.iter().map(|x| x / rhs).collect(),
            shape: self.shape,
        }
    }
}

impl AddAssign<f64> for Matrix {
    fn add_assign(&mut self, rhs: f64) {
        self.iter_mut().for_each(|x| *x += rhs)
    }
}

impl SubAssign<f64> for Matrix {
    fn sub_assign(&mut self, rhs: f64) {
        self.iter_mut().for_each(|x| *x -= rhs)
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        self.iter_mut().for_each(|x| *x *= rhs)
    }
}

impl DivAssign<f64> for Matrix {
    fn div_assign(&mut self, rhs: f64) {
        self.iter_mut().for_each(|x| *x /= rhs)
    }
}
