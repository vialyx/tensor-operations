/// Example: Basic Matrix Operations
/// Demonstrates fundamental matrix operations with both implementations

use tensor_operations::manual::Matrix;
use tensor_operations::optimized::OptimizedMatrix;

fn repeat(s: &str, n: usize) -> String {
    (0..n).map(|_| s).collect::<String>()
}

fn main() {
    println!("{}", repeat("=", 60));
    println!("Basic Matrix Operations in Rust");
    println!("{}", repeat("=", 60));

    // ===== Manual Implementation =====
    println!("\n1. MANUAL IMPLEMENTATION");
    println!("{}", repeat("-", 60));

    let a_data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let a = Matrix::new(a_data).expect("Failed to create matrix A");
    println!("\nMatrix A ({}×{}):", a.rows(), a.cols());
    println!("{}", a);

    let b_data = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    let b = Matrix::new(b_data).expect("Failed to create matrix B");
    println!("Matrix B ({}×{}):", b.rows(), b.cols());
    println!("{}", b);

    // Matrix multiplication
    match a.mul(&b) {
        Ok(c) => {
            println!("A × B:");
            println!("{}", c);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Transpose
    match a.transpose() {
        Ok(at) => {
            println!("A^T (transpose):");
            println!("{}", at);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Square matrix operations
    let m_data = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
    let m = Matrix::new(m_data).expect("Failed to create matrix M");
    println!("\nMatrix M (square):");
    println!("{}", m);

    // Determinant
    match m.determinant() {
        Ok(det) => println!("Determinant of M: {}", det),
        Err(e) => println!("Error: {}", e),
    }

    // Frobenius norm
    let norm = m.frobenius_norm();
    println!("Frobenius norm of M: {:.4}", norm);

    // Trace
    match m.trace() {
        Ok(trace) => println!("Trace of M: {}", trace),
        Err(e) => println!("Error: {}", e),
    }

    // Inverse
    match m.inverse() {
        Ok(inv) => {
            println!("Inverse of M:");
            println!("{}", inv);

            // Verify: M × M^-1 should be identity
            match m.mul(&inv) {
                Ok(product) => {
                    println!("Verification (M × M^-1 should be identity):");
                    println!("{}", product);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // ===== Optimized Implementation =====
    println!("\n{}", repeat("=", 60));
    println!("2. OPTIMIZED IMPLEMENTATION (nalgebra)");
    println!("{}", repeat("-", 60));

    let opt_a_data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let opt_a = OptimizedMatrix::new(opt_a_data).expect("Failed to create matrix A");
    println!("\nMatrix A ({}×{}):", opt_a.rows(), opt_a.cols());
    println!("{}", opt_a);

    let opt_b_data = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    let opt_b = OptimizedMatrix::new(opt_b_data).expect("Failed to create matrix B");
    println!("Matrix B ({}×{}):", opt_b.rows(), opt_b.cols());
    println!("{}", opt_b);

    // Matrix multiplication
    match opt_a.mul(&opt_b) {
        Ok(c) => {
            println!("A × B:");
            println!("{}", c);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Transpose
    match opt_a.transpose() {
        Ok(at) => {
            println!("A^T (transpose):");
            println!("{}", at);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Square matrix operations
    let opt_m_data = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
    let opt_m = OptimizedMatrix::new(opt_m_data).expect("Failed to create matrix M");
    println!("\nMatrix M (square):");
    println!("{}", opt_m);

    // Determinant
    match opt_m.determinant() {
        Ok(det) => println!("Determinant of M: {}", det),
        Err(e) => println!("Error: {}", e),
    }

    // Frobenius norm
    let norm = opt_m.frobenius_norm();
    println!("Frobenius norm of M: {:.4}", norm);

    // Trace
    match opt_m.trace() {
        Ok(trace) => println!("Trace of M: {}", trace),
        Err(e) => println!("Error: {}", e),
    }

    // Rank
    match opt_m.rank() {
        Ok(r) => println!("Rank of M: {}", r),
        Err(e) => println!("Error: {}", e),
    }

    // Inverse
    match opt_m.inverse() {
        Ok(inv) => {
            println!("Inverse of M:");
            println!("{}", inv);

            // Verify: M × M^-1 should be identity
            match opt_m.mul(&inv) {
                Ok(product) => {
                    println!("Verification (M × M^-1 should be identity):");
                    println!("{}", product);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n{}", repeat("=", 60));
}
