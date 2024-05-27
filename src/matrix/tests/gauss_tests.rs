use crate::{matrix, matrix::Matrix};

#[test]
fn eye() {
    let mat = Matrix::eye(3);
    assert_eq!(mat, matrix![1.,0.,0.; 0.,1.,0.; 0.,0.,1.])
}

#[test]
fn gauss() {
    let mat = matrix![
        4., -3., 1., -8.;
        -2., 1., -3., -4.;
        1., -1., 2., 3.;
    ];
    assert_eq!(
        mat.gauss(false),
        matrix![
            1., 0., 0., -2.;
            0., 1., 0., 1.;
            0., 0., 1., 3.;

        ]
    )
}

#[test]
fn inverse() {
    let mat = matrix![
        3., 0., 2.;
        2., 0., -2.;
        0., 1., 1.;
    ];

    assert!(mat.inverse().equals(
        matrix![
            0.2, 0.2, 0.;
            -0.2, 0.3, 1.;
            0.2, -0.3, 0.;
        ],
        1e-10,
    ))
}
