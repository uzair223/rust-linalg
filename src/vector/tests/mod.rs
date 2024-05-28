use super::{Vector, VectorType};
use crate::{matrix, matrix::Matrix, vector};

#[test]
fn to_matrix() {
    let v = vector![0., 1., 2.].to_matrix(VectorType::Column);
    assert_eq!(v, matrix![0.; 1.; 2.])
}

mod op_tests;
