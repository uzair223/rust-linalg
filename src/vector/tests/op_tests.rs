use std::f64::consts::PI;

use crate::{vector, vector::Vector};

#[test]
fn dot_product() {
    let a = vector![1., 2., 3.];
    let b = vector![4., 5., 6.];
    assert_eq!(a.dot(b), 32.)
}

#[test]
fn cross_product() {
    let a = vector![1., 2., 3.];
    let b = vector![4., 5., 6.];
    assert_eq!(a.cross(b), vector![-3., 6., -3.])
}

#[test]
fn scalar_op() {
    let a = vector![1., 2., 3.];
    assert_eq!(a + 2., vector![3., 4., 5.])
}

#[test]
fn scalar_op_assign() {
    let mut a = vector![1., 2., 3.];
    a += 2.;
    assert_eq!(a, vector![3., 4., 5.])
}

#[test]
fn vector_op() {
    let a = vector![1., 2., 3., 4.];
    let b = vector![5., 6., 7., 8.];
    assert_eq!(a + b, vector![6., 8., 10., 12.])
}

#[test]
fn vector_op_assign() {
    let mut a = vector![1., 2., 3., 4.];
    a += vector![5., 6., 7., 8.];
    assert_eq!(a, vector![6., 8., 10., 12.])
}

#[test]
fn norm() {
    let a = vector![3., 4.];
    assert_eq!(a.norm(), 5.)
}

#[test]
fn angle() {
    let a = vector![1., 0.];
    let b = vector![1., 1.];
    assert!(a.angle(b) - PI / 4. < 1e-3)
}
