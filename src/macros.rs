#[macro_export]
macro_rules! mat {
    ( $( $($val:expr),+ );* $(;)? ) => {
        {
            let mut data = Vec::<f64>::new();
            let mut rows = 0;
            let mut cols = 0;
            $(
                let row_data = vec![$($val),+];
                data.extend(row_data);
                rows += 1;
                let row_len = vec![$($val),+].len();
                if cols == 0 {
                    cols = row_len;
                } else if cols != row_len {
                    panic!("Inconsistent number of elements in the matrix rows");
                }
            )*
            Matrix::from(data, (rows, cols))
        }
    };
}
