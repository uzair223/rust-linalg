use crate::{matrix, matrix::Matrix};

#[test]
fn get_minor() {
    let mat = matrix![
      1., 2., 3.;
      4., 5., 6.;
      7., 8., 9.;
    ];
    assert_eq!(mat.get_minor(0, 1), matrix![ 4., 6.; 7., 9.; ])
}

#[test]
fn det() {
    let mat = matrix![
      1., 2., 3.;
      4., 5., 6.;
      7., 8., 9.;
    ];
    assert_eq!(mat.det(), 0.)
}
