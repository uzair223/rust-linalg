use crate::{matrix::Matrix, matrix, vector};
use super::{Vector, VectorType};

#[test]
fn to_matrix() {
  let v = vector![0., 1., 2.].to_matrix(VectorType::Column);
  assert_eq!(v, matrix![0.; 1.; 2.])
}

mod op_tests;
