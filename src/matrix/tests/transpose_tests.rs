use crate::{mat, matrix::Matrix};

#[test]
fn transpose() {
    let mat = mat![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let transpose = mat.tranpose();
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
fn index_transpose() {
    let mat = mat![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let transpose = mat.tranpose();
    assert_eq!(transpose[[0, 1]], 4.)
}

#[test]
fn index_transpose_mut() {
    let mut mat = mat![
      1.,2.,3.;
      4.,5.,6.;
      7.,8.,9.
    ];
    let mut transpose = mat.tranpose_mut();
    transpose[[0, 1]] += 1.;

    assert_eq!(transpose[[0, 1]], 5.);
    assert_eq!(mat[[1, 0]], 5.)
}
