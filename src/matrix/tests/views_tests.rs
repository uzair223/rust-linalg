use crate::{matrix, matrix::Matrix, vector, vector::Vector};

#[test]
fn transpose() {
    let mat = matrix![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let transpose = mat.transpose();
    assert_eq!(
        transpose.to_matrix(),
        matrix![
          1.,4.,7.;
          2.,5.,8.;
          3.,6.,9.;
        ]
    )
}

#[test]
fn slice() {
    let mat = matrix![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let slice = mat.slice((1..3, ..2));
    assert_eq!(slice.to_matrix(), matrix![4.,5.; 7.,8.])
}

#[test]
fn slice_index() {
    let mat = matrix![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let slice = mat.slice((1..3, 2));
    assert_eq!(slice.to_matrix(), matrix![6.; 9.])
}

#[test]
fn row_column() {
    let mat = matrix![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    assert_eq!(mat.column(0).to_vector(), vector![1., 4., 7.])
}

#[test]
fn index_view() {
    let mat = matrix![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let transpose = mat.transpose();
    assert_eq!(transpose[[0, 1]], 4.)
}

#[test]
fn index_view_mut() {
    let mut mat = matrix![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let mut transpose = mat.transpose_mut();
    transpose[[0, 1]] += 1.;

    assert_eq!(transpose[[0, 1]], 5.);
    assert_eq!(mat[[1, 0]], 5.)
}

#[test]
fn view_assign() {
    let mut mat = matrix![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let mut column = mat.column_mut(0);
    column.assign(&[0., 0., 0.]);
    assert_eq!(column.to_vector(), vector![0., 0., 0.]);
    assert_eq!(mat, matrix![0., 2., 3.; 0., 5., 6.; 0., 8., 9.]);
}
