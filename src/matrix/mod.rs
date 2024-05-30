use core::fmt;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    pub shape: (usize, usize),
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rows, cols) = self.shape;
        write!(f, "[")?;
        for i in 0..rows {
            if i == 0 {
                write!(f, "[")?;
            } else {
                write!(f, " [")?;
            }
            for j in 0..cols {
                if j > 0 {
                    write!(f, " ")?; // add a space between columns
                }
                // Apply the formatter's precision settings to each element
                if let Some(precision) = f.precision() {
                    write!(f, "{:.1$}", self.data[i * cols + j], precision)?;
                } else {
                    write!(f, "{}", self.data[i * cols + j])?;
                }
            }
            write!(f, "]")?;
            if i < rows - 1 {
                writeln!(f)?; // add a newline between rows
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

macro_rules! _check_same_shape {
    ($a:expr, $b:expr) => {
        if ($a.shape != $b.shape) {
            panic!("shape mismatch: {:?} != {:?}", $a.shape, $b.shape)
        }
    };
}

macro_rules! _check_square {
    ($a:expr) => {
        if ($a.shape.0 != $a.shape.1) {
            panic!("matrix must be square: {:?}", $a.shape)
        }
    };
}

mod constructors;
mod index_ops;
mod matrix_ops;
mod ops;
mod scalar_ops;
mod views;

#[cfg(test)]
mod tests;
