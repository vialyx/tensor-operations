/// Manual matrix implementation from scratch in Rust
/// Educational implementation to understand linear algebra algorithms

use std::fmt;
use std::ops::{Add, Sub, Mul, Index};

/// A simple matrix struct storing data in row-major format
#[derive(Clone, Debug)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Create a new matrix from a vector of vectors
    pub fn new(data: Vec<Vec<f64>>) -> Result<Self, String> {
        if data.is_empty() || data[0].is_empty() {
            return Err("Matrix cannot be empty".to_string());
        }

        let rows = data.len();
        let cols = data[0].len();

        // Validate all rows have the same length
        for row in &data {
            if row.len() != cols {
                return Err("All rows must have the same length".to_string());
            }
        }

        Ok(Matrix { data, rows, cols })
    }

    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get the shape as (rows, cols)
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Get a reference to the underlying data
    pub fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    /// Get the element at position (i, j)
    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        self.data.get(i).and_then(|row| row.get(j).copied())
    }

    /// Matrix addition
    pub fn add(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(format!(
                "Cannot add matrices with different dimensions: ({},{}) + ({},{})",
                self.rows, self.cols, other.rows, other.cols
            ));
        }

        let mut result = Vec::new();
        for i in 0..self.rows {
            let mut row = Vec::new();
            for j in 0..self.cols {
                row.push(self.data[i][j] + other.data[i][j]);
            }
            result.push(row);
        }

        Matrix::new(result)
    }

    /// Matrix subtraction
    pub fn sub(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(format!(
                "Cannot subtract matrices with different dimensions: ({},{}) - ({},{})",
                self.rows, self.cols, other.rows, other.cols
            ));
        }

        let mut result = Vec::new();
        for i in 0..self.rows {
            let mut row = Vec::new();
            for j in 0..self.cols {
                row.push(self.data[i][j] - other.data[i][j]);
            }
            result.push(row);
        }

        Matrix::new(result)
    }

    /// Scalar multiplication
    pub fn scalar_mul(&self, scalar: f64) -> Result<Matrix, String> {
        let mut result = Vec::new();
        for i in 0..self.rows {
            let mut row = Vec::new();
            for j in 0..self.cols {
                row.push(self.data[i][j] * scalar);
            }
            result.push(row);
        }

        Matrix::new(result)
    }

    /// Matrix multiplication
    pub fn mul(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.cols != other.rows {
            return Err(format!(
                "Cannot multiply matrices: ({},{}) × ({},{})",
                self.rows, self.cols, other.rows, other.cols
            ));
        }

        let mut result = Vec::new();
        for i in 0..self.rows {
            let mut row = Vec::new();
            for j in 0..other.cols {
                let mut value = 0.0;
                for k in 0..self.cols {
                    value += self.data[i][k] * other.data[k][j];
                }
                row.push(value);
            }
            result.push(row);
        }

        Matrix::new(result)
    }

    /// Get the minor (submatrix with row i and column j removed)
    fn get_minor(&self, row: usize, col: usize) -> Result<Matrix, String> {
        let mut result = Vec::new();
        for i in 0..self.rows {
            if i == row {
                continue;
            }
            let mut new_row = Vec::new();
            for j in 0..self.cols {
                if j == col {
                    continue;
                }
                new_row.push(self.data[i][j]);
            }
            result.push(new_row);
        }

        if result.is_empty() {
            Err("Cannot compute minor of 1x1 matrix".to_string())
        } else {
            Matrix::new(result)
        }
    }

    /// Calculate determinant using cofactor expansion
    pub fn determinant(&self) -> Result<f64, String> {
        if self.rows != self.cols {
            return Err("Determinant only defined for square matrices".to_string());
        }

        let n = self.rows;

        // Base case: 1x1 matrix
        if n == 1 {
            return Ok(self.data[0][0]);
        }

        // Base case: 2x2 matrix
        if n == 2 {
            return Ok(self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]);
        }

        // Recursive case: cofactor expansion along first row
        let mut det = 0.0;
        for j in 0..n {
            let minor = self.get_minor(0, j)?;
            let minor_det = minor.determinant()?;
            let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
            det += sign * self.data[0][j] * minor_det;
        }

        Ok(det)
    }

    /// Check if matrix is square
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// Check if matrix is invertible
    pub fn is_invertible(&self) -> Result<bool, String> {
        if !self.is_square() {
            return Ok(false);
        }
        let det = self.determinant()?;
        Ok(det.abs() > 1e-10)
    }

    /// Calculate matrix inverse using adjugate method
    pub fn inverse(&self) -> Result<Matrix, String> {
        if !self.is_invertible()? {
            return Err("Matrix is not invertible (determinant is zero)".to_string());
        }

        let n = self.rows;
        let det = self.determinant()?;

        // Special case: 2x2 matrix
        if n == 2 {
            let data = vec![
                vec![self.data[1][1] / det, -self.data[0][1] / det],
                vec![-self.data[1][0] / det, self.data[0][0] / det],
            ];
            return Matrix::new(data);
        }

        // General case: compute cofactor matrix then transpose
        let mut cofactors = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                let minor = self.get_minor(i, j)?;
                let minor_det = minor.determinant()?;
                let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                row.push(sign * minor_det / det);
            }
            cofactors.push(row);
        }

        let cofactor_matrix = Matrix::new(cofactors)?;
        cofactor_matrix.transpose()
    }

    /// Transpose the matrix
    pub fn transpose(&self) -> Result<Matrix, String> {
        let mut result = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[j][i] = self.data[i][j];
            }
        }
        Matrix::new(result)
    }

    /// Calculate the Frobenius norm
    pub fn frobenius_norm(&self) -> f64 {
        let mut sum = 0.0;
        for row in &self.data {
            for &val in row {
                sum += val * val;
            }
        }
        sum.sqrt()
    }

    /// Calculate the trace (sum of diagonal elements)
    pub fn trace(&self) -> Result<f64, String> {
        if !self.is_square() {
            return Err("Trace only defined for square matrices".to_string());
        }

        let mut sum = 0.0;
        for i in 0..self.rows {
            sum += self.data[i][i];
        }

        Ok(sum)
    }

    /// Create an identity matrix of size n×n
    pub fn identity(n: usize) -> Result<Matrix, String> {
        let mut data = vec![vec![0.0; n]; n];
        for i in 0..n {
            data[i][i] = 1.0;
        }
        Matrix::new(data)
    }

    /// Create a zero matrix of size rows×cols
    pub fn zeros(rows: usize, cols: usize) -> Result<Matrix, String> {
        if rows == 0 || cols == 0 {
            return Err("Matrix dimensions must be positive".to_string());
        }
        let data = vec![vec![0.0; cols]; rows];
        Matrix::new(data)
    }

    /// Create a matrix of ones
    pub fn ones(rows: usize, cols: usize) -> Result<Matrix, String> {
        if rows == 0 || cols == 0 {
            return Err("Matrix dimensions must be positive".to_string());
        }
        let data = vec![vec![1.0; cols]; rows];
        Matrix::new(data)
    }
}

// Implement Display trait for pretty printing
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Matrix ({} × {})", self.rows, self.cols)?;
        for row in &self.data {
            write!(f, "[")?;
            for (i, &val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:8.2}", val)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

// Implement operator overloading
impl Add for &Matrix {
    type Output = Result<Matrix, String>;

    fn add(self, other: &Matrix) -> Result<Matrix, String> {
        self.add(other)
    }
}

impl Sub for &Matrix {
    type Output = Result<Matrix, String>;

    fn sub(self, other: &Matrix) -> Result<Matrix, String> {
        self.sub(other)
    }
}

impl Mul for &Matrix {
    type Output = Result<Matrix, String>;

    fn mul(self, other: &Matrix) -> Result<Matrix, String> {
        self.mul(other)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &f64 {
        &self.data[i][j]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_matrix_addition() {
        let a = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let b = Matrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        let c = a.add(&b).unwrap();
        assert_eq!(c[(0, 0)], 6.0);
        assert_eq!(c[(1, 1)], 12.0);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let b = Matrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        let c = a.mul(&b).unwrap();
        assert_eq!(c[(0, 0)], 19.0); // 1*5 + 2*7
        assert_eq!(c[(0, 1)], 22.0); // 1*6 + 2*8
    }

    #[test]
    fn test_determinant_2x2() {
        let m = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let det = m.determinant().unwrap();
        assert!((det - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_2x2() {
        let m = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let inv = m.inverse().unwrap();
        let product = (&m * &inv).unwrap();
        let identity = Matrix::identity(2).unwrap();
        
        for i in 0..2 {
            for j in 0..2 {
                assert!((product[(i, j)] - identity[(i, j)]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_transpose() {
        let m = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let t = m.transpose().unwrap();
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
        assert_eq!(t[(0, 0)], 1.0);
        assert_eq!(t[(0, 1)], 4.0);
    }

    #[test]
    fn test_frobenius_norm() {
        let m = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let norm = m.frobenius_norm();
        let expected = (1.0 + 4.0 + 9.0 + 16.0_f64).sqrt();
        assert!((norm - expected).abs() < 1e-10);
    }

    #[test]
    fn test_trace() {
        let m = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let trace = m.trace().unwrap();
        assert_eq!(trace, 5.0);
    }
}
