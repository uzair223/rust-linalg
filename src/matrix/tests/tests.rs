use crate::{mat, matrix::Matrix};

#[test]
fn dot_product() {
    let a = mat![
        1., 2.;
        3., 4.;
        5., 6.;
    ];
    let b = mat![
        1., 2.;
        3., 4.;
    ];
    assert_eq!(
        a.dot(&b),
        mat![
             7., 10.;
            15., 22.;
            23., 34.;
        ]
    )
}

#[test]
fn scalar_op() {
    let a = mat![1., 2.; 3., 4.; 5., 6.];
    assert_eq!(a + 2., mat![3., 4.; 5., 6.; 7., 8.])
}

#[test]
fn scalar_op_assign() {
    let mut a = mat![1., 2.; 3., 4.; 5., 6.];
    a += 2.;
    assert_eq!(a, mat![3., 4.; 5., 6.; 7., 8.])
}

#[test]
fn matrix_op() {
    let a = mat![1., 2.; 3., 4.];
    let b = mat![5., 6.; 7., 8.];
    assert_eq!(a + b, mat![6., 8.; 10., 12.])
}

#[test]
fn matrix_op_assign() {
    let mut a = mat![1., 2.; 3., 4.];
    a += mat![5., 6.; 7., 8.];
    assert_eq!(a, mat![6., 8.; 10., 12.])
}

#[test]
fn is_upper_triangular() {
    let a = mat![
        1., 2., 3.;
        0., 4., 5.;
        0., 0., 6.];
    assert!(a.is_upper_triangular())
}

#[test]
fn is_lower_triangular() {
    let a = mat![
        1., 0., 0.;
        2., 4., 0.;
        3., 5., 6.];
    assert!(a.is_lower_triangular())
}

#[test]
fn diag() {
    let a = mat![1., 2., 3.; 4., 5., 6.; 7., 8., 9.];
    assert_eq!(a.diag(), [1., 5., 9.])
}
