use crate::{matrix, matrix::Matrix};

#[test]
fn qr() {
    let mat = matrix![
      12., -51.,   4.;
       6., 167., -68.;
      -4.,  24., -41.;
    ];
    let (q, r) = mat.qr();
    assert!(mat.equals(q.dot(&r), 1e-10));
}

#[test]
fn eigenvalues() {
    let mat = matrix![
      -4., 1., -2., 2.;
      1., 2., 0., 1.;
      -2., 0., 3., -2.;
      2., 1., -2., -1.
    ];

    let eigenvalues = mat.eigen(0., 1000).0;
    let mut v = eigenvalues.iter().map(|x| x.round() as i32).collect::<Vec<i32>>();
    v.sort();

    assert_eq!(v, [-5, -2, 2, 5]);
}

#[test]
fn eigenvectors() {
  let mat = matrix![
    -4., 1., -2.,  2.;
     1., 2.,  0.,  1.;
    -2., 0.,  3., -2.;
     2., 1., -2., -1.;
  ];

  let (eigenvalues, eigenvectors) = mat.eigen(0., 1000);

  for i in 0..4 {
    let l = eigenvalues[i];
    let v = eigenvectors.column(i).to_vector();
    assert!(mat.dot_vec(&v).equals(v.clone()*l, 1e-10))
  }
}