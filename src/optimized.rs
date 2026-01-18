/// Optimized matrix operations using nalgebra library
/// nalgebra provides high-performance linear algebra operations

use nalgebra::DMatrix;
use rand::random;
use std::fmt;

/// A wrapper around nalgebra's DMatrix for consistency with our API
pub struct OptimizedMatrix {
    data: DMatrix<f64>,
}

impl OptimizedMatrix {
    /// Create a new optimized matrix from a nested vector
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

        // Convert to nalgebra matrix
        let mut flat_data = Vec::new();
        for row in data {
            flat_data.extend(row);
        }

        let matrix = DMatrix::from_row_slice(rows, cols, &flat_data);
        Ok(OptimizedMatrix { data: matrix })
    }

    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.data.nrows()
    }

    /// Get the number of columns
    pub fn cols(&self) -> usize {
        self.data.ncols()
    }

    /// Get the shape as (rows, cols)
    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    /// Get a reference to the underlying nalgebra matrix
    pub fn data(&self) -> &DMatrix<f64> {
        &self.data
    }

    /// Get the element at position (i, j)
    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        self.data.get((i, j)).copied()
    }

    /// Convert to nested vector format
    pub fn to_vec(&self) -> Vec<Vec<f64>> {
        let mut result = Vec::new();
        for i in 0..self.rows() {
            let mut row = Vec::new();
            for j in 0..self.cols() {
                row.push(self.data[(i, j)]);
            }
            result.push(row);
        }
        result
    }

    /// Matrix addition
    pub fn add(&self, other: &OptimizedMatrix) -> Result<OptimizedMatrix, String> {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return Err(format!(
                "Cannot add matrices with different dimensions: ({},{}) + ({},{})",
                self.rows(),
                self.cols(),
                other.rows(),
                other.cols()
            ));
        }

        let result = &self.data + &other.data;
        Ok(OptimizedMatrix { data: result })
    }

    /// Matrix subtraction
    pub fn sub(&self, other: &OptimizedMatrix) -> Result<OptimizedMatrix, String> {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return Err(format!(
                "Cannot subtract matrices with different dimensions: ({},{}) - ({},{})",
                self.rows(),
                self.cols(),
                other.rows(),
                other.cols()
            ));
        }

        let result = &self.data - &other.data;
        Ok(OptimizedMatrix { data: result })
    }

    /// Scalar multiplication
    pub fn scalar_mul(&self, scalar: f64) -> Result<OptimizedMatrix, String> {
        let result = &self.data * scalar;
        Ok(OptimizedMatrix { data: result })
    }

    /// Matrix multiplication
    pub fn mul(&self, other: &OptimizedMatrix) -> Result<OptimizedMatrix, String> {
        if self.cols() != other.rows() {
            return Err(format!(
                "Cannot multiply matrices: ({},{}) × ({},{})",
                self.rows(),
                self.cols(),
                other.rows(),
                other.cols()
            ));
        }

        let result = &self.data * &other.data;
        Ok(OptimizedMatrix { data: result })
    }

    /// Calculate determinant
    pub fn determinant(&self) -> Result<f64, String> {
        if !self.is_square() {
            return Err("Determinant only defined for square matrices".to_string());
        }

        Ok(self.data.determinant())
    }

    /// Check if matrix is square
    pub fn is_square(&self) -> bool {
        self.rows() == self.cols()
    }

    /// Check if matrix is invertible
    pub fn is_invertible(&self) -> Result<bool, String> {
        if !self.is_square() {
            return Ok(false);
        }

        let det = self.determinant()?;
        Ok(det.abs() > 1e-10)
    }

    /// Calculate matrix inverse
    pub fn inverse(&self) -> Result<OptimizedMatrix, String> {
        if !self.is_invertible()? {
            return Err("Matrix is not invertible (determinant is zero)".to_string());
        }

        match self.data.clone().try_inverse() {
            Some(inv) => Ok(OptimizedMatrix { data: inv }),
            None => Err("Failed to compute matrix inverse".to_string()),
        }
    }

    /// Transpose the matrix
    pub fn transpose(&self) -> Result<OptimizedMatrix, String> {
        let result = self.data.transpose();
        Ok(OptimizedMatrix { data: result })
    }

    /// Calculate the Frobenius norm
    pub fn frobenius_norm(&self) -> f64 {
        self.data.norm()
    }

    /// Calculate the trace (sum of diagonal elements)
    pub fn trace(&self) -> Result<f64, String> {
        if !self.is_square() {
            return Err("Trace only defined for square matrices".to_string());
        }

        Ok(self.data.trace())
    }

    /// Calculate eigenvalues (for symmetric matrices)
    pub fn eigenvalues(&self) -> Result<Vec<f64>, String> {
        if !self.is_square() {
            return Err("Eigenvalues only defined for square matrices".to_string());
        }

        // For symmetric matrices
        let eigen = self.data.clone().symmetric_eigen();
        Ok(eigen.eigenvalues.data.as_vec().to_vec())
    }

    /// Get matrix rank
    pub fn rank(&self) -> Result<usize, String> {
        let eps = 1e-10;
        Ok(self.data.rank(eps))
    }

    /// QR Decomposition
    pub fn qr_decomposition(
        &self,
    ) -> Result<(OptimizedMatrix, OptimizedMatrix), String> {
        let qr = self.data.clone().qr();
        let q = qr.q();
        let r = qr.r();

        Ok((
            OptimizedMatrix { data: q },
            OptimizedMatrix { data: r },
        ))
    }

    /// Singular Value Decomposition
    pub fn svd(&self) -> Result<(OptimizedMatrix, Vec<f64>, OptimizedMatrix), String> {
        let svd_result = self.data.clone().svd(true, true);
        let u = svd_result.u.expect("U matrix should exist");
        let vt = svd_result.v_t.expect("V^T matrix should exist");
        let singular_values = svd_result.singular_values.data.as_vec().to_vec();

        Ok((
            OptimizedMatrix { data: u },
            singular_values,
            OptimizedMatrix { data: vt },
        ))
    }

    /// Solve linear system Ax = b
    pub fn solve(&self, b: &OptimizedMatrix) -> Result<OptimizedMatrix, String> {
        if !self.is_square() {
            return Err("Linear system solver requires square matrix".to_string());
        }

        match self.data.clone().lu().solve(&b.data) {
            Some(result) => Ok(OptimizedMatrix { data: result }),
            None => Err("Failed to solve linear system".to_string()),
        }
    }

    /// Create an identity matrix of size n×n
    pub fn identity(n: usize) -> Result<OptimizedMatrix, String> {
        if n == 0 {
            return Err("Matrix dimensions must be positive".to_string());
        }
        let data = DMatrix::identity(n, n);
        Ok(OptimizedMatrix { data })
    }

    /// Create a zero matrix of size rows×cols
    pub fn zeros(rows: usize, cols: usize) -> Result<OptimizedMatrix, String> {
        if rows == 0 || cols == 0 {
            return Err("Matrix dimensions must be positive".to_string());
        }
        let data = DMatrix::zeros(rows, cols);
        Ok(OptimizedMatrix { data })
    }

    /// Create a matrix of ones
    pub fn ones(rows: usize, cols: usize) -> Result<OptimizedMatrix, String> {
        if rows == 0 || cols == 0 {
            return Err("Matrix dimensions must be positive".to_string());
        }
        let data = DMatrix::from_element(rows, cols, 1.0);
        Ok(OptimizedMatrix { data })
    }

    /// Create a random matrix
    pub fn random(rows: usize, cols: usize) -> Result<OptimizedMatrix, String> {
        if rows == 0 || cols == 0 {
            return Err("Matrix dimensions must be positive".to_string());
        }
        // Create matrix with random values using from_fn
        let data = DMatrix::from_fn(rows, cols, |_, _| {
            (random::<f64>() - 0.5) * 2.0
        });
        Ok(OptimizedMatrix { data })
    }
}

impl fmt::Display for OptimizedMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "OptimizedMatrix ({} × {})", self.rows(), self.cols())?;
        for i in 0..self.rows() {
            write!(f, "[")?;
            for j in 0..self.cols() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:8.2}", self.data[(i, j)])?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl fmt::Debug for OptimizedMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m = OptimizedMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_matrix_addition() {
        let a = OptimizedMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let b = OptimizedMatrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();
        let c = a.add(&b).unwrap();
        assert_eq!(c.get(0, 0).unwrap(), 6.0);
        assert_eq!(c.get(1, 1).unwrap(), 12.0);
    }

    #[test]
    fn test_determinant() {
        let m = OptimizedMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let det = m.determinant().unwrap();
        assert!((det - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_transpose() {
        let m = OptimizedMatrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        let t = m.transpose().unwrap();
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
    }

    #[test]
    fn test_trace() {
        let m = OptimizedMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
        let trace = m.trace().unwrap();
        assert_eq!(trace, 5.0);
    }
}
