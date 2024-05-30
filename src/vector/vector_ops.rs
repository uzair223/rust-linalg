use crate::{matrix::Matrix, vector};

use super::Vector;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Vector {
    pub fn equals(self, other: Self, eps: f64) -> bool {
        let dist = self - other;
        for d in dist.iter() {
            if d.abs() > eps {
                return false;
            }
        }
        true
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
    fn ne(&self, other: &Self) -> bool {
        self.data != other.data
    }
}

impl Vector {
    pub fn dot(&self, rhs: &Self) -> f64 {
        _check_same_size!(self, rhs);
        self.iter()
            .zip(rhs.iter())
            .fold(0., |dot, (a, b)| dot + a * b)
    }

    pub fn cross(self, rhs: Self) -> Vector {
        _check_same_size!(self, rhs);
        let mat = Matrix::new(vec![vec![0.; self.size], self.data, rhs.data]);
        let mut cross = vector![0.; self.size];
        for j in 0..self.size {
            cross[j] = f64::powi(-1., j as i32) * mat.get_minor(0, j).det()
        }
        cross
    }

    pub fn angle(self, rhs: Self) -> f64 {
        _check_same_size!(self, rhs);
        let a_norm = self.norm();
        let b_norm = rhs.norm();
        let dot = self.dot(&rhs);
        f64::acos(dot / (a_norm * b_norm))
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        _check_same_size!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a + b).collect(),
            size: self.size,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        _check_same_size!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect(),
            size: self.size,
        }
    }
}

impl Mul for Vector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        _check_same_size!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect(),
            size: self.size,
        }
    }
}

impl Div for Vector {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        _check_same_size!(self, rhs);
        Self {
            data: self.iter().zip(rhs.iter()).map(|(a, b)| a / b).collect(),
            size: self.size,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        _check_same_size!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a += b)
            .collect()
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        _check_same_size!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a -= b)
            .collect()
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        _check_same_size!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a *= b)
            .collect()
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, rhs: Self) {
        _check_same_size!(self, rhs);
        self.iter_mut()
            .zip(rhs.iter())
            .map(|(a, b)| *a /= b)
            .collect()
    }
}
