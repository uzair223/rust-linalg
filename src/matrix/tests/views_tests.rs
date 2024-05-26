use crate::matrix::views::View;
use crate::{mat, matrix::Matrix};

#[test]
fn transpose() {
    let mat = mat![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let transpose = mat.transpose();
    assert_eq!(
        transpose.to_matrix(),
        mat![
          1.,4.,7.;
          2.,5.,8.;
          3.,6.,9.;
        ]
    )
}

#[test]
fn slice() {
    let mat = mat![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let slice = mat.slice((1..3, ..2));
    assert_eq!(slice.to_matrix(), mat![4.,5.; 7.,8.])
}

#[test]
fn index_view() {
    let mat = mat![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let transpose = mat.transpose();
    assert_eq!(transpose[[0, 1]], 4.)
}

#[test]
fn index_view_mut() {
    let mut mat = mat![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let mut transpose = mat.transpose_mut();
    transpose[[0, 1]] += 1.;

    assert_eq!(transpose[[0, 1]], 5.);
    assert_eq!(mat[[1, 0]], 5.)
}
