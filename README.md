# rust-linalg

My introduction to Rust. A basic linear algebra library implementing some matrix/vector operations.

## Vector
- dereference to expose underlying `Vec<f64>`
- index a vector like `my_vector[n]`
- macro to create vectors quickly `vector![1., 2., 3.]`
- convert to `Matrix`; `a.to_matrix(VectorType::Column)`
### Operations
- element-wise arithmetic operations with assignment
- vector dot product `u.dot(&v)`
- vector cross product `u.cross(&v)`
- vector norm `u.norm()`

## Matrix
- implementation uses an underlying `Vec<f64>`
- index a matrix like `my_matrix[[i,j]]` which results in the underlying data being indexed `data[i*columns+j]`
- dereference to expose underlying `Vec<f64>`
- create an identity matrix of `n` size with `Matrix::eye(n)`
- create an empty `m`x`n` matrix filled with value `x` with `Matrix::empty(x, m, n)`.
- macro to create matrices `matrix![0., 1., 2.; 3., 4., 5;]`
### Views
- `a.transpose()` returns a transposed view of the original matrix.
- `a.row(i)`, `a.column(j)` returns a view of the row/column and can be converted to a vector or matrix.
- `slice` returns a sliced view of the original matrix using ranges, i.e., `matrix.slice((2..4, 0))`
-  mutable views with the `_mut` which allows modification to the underlying matrix.
-  `view.assign(data)` on a mutable view modifies the underlying matrix corresponding to the view:
```rust
let mut a = matrix![
  0., 1., 2.;
  3., 4., 5.;
];
let mut view = a.column_mut(0); // column view [0., 3.];
view[1] = 0.;
/* 
[[      0., 1., 2.]
 [3. => 0., 4., 5.]]
*/
view.assign([10., 20.]);
/*
[[0. => 10., 1., 2.]
 [0. => 20., 4., 5.]]
*/
```
### Operations
- element-wise arithmetic operations with assignment
- matrix multiplication `a.dot(&b)`
- matrix-vector product `a.dot_vec(&v)`
- recursive determinant algorithm `a.det()` 
- gaussian elimination `a.gauss()`
- matrix inverse using augmented matrix `a.inverse()`
- qr decomposition using modified gram-schmidt algorithm `a.qr()`
- qr algorithm to find eigenvalues/vectors `a.eigen()`
- `is_upper_triangular(eps)`, `is_lower_triangular(eps)`, `is_triangular(eps)`, `is_square()` boolean flags 