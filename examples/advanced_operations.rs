/// Example: Advanced Operations
/// Demonstrates QR decomposition, SVD, eigenvalues, and other advanced operations

use tensor_operations::optimized::OptimizedMatrix;

fn repeat(s: &str, n: usize) -> String {
    (0..n).map(|_| s).collect::<String>()
}

fn main() {
    println!("{}", repeat("=", 70));
    println!("Advanced Matrix Operations");
    println!("{}", repeat("=", 70));

    // Example 1: QR Decomposition
    println!("\n1. QR DECOMPOSITION");
    println!("{}", repeat("-", 70));

    let a_data = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let a = OptimizedMatrix::new(a_data).expect("Failed to create matrix A");
    println!("Original matrix A:");
    println!("{}", a);

    match a.qr_decomposition() {
        Ok((q, r)) => {
            println!("\nQ (orthogonal matrix):");
            println!("{}", q);
            println!("\nR (upper triangular matrix):");
            println!("{}", r);

            // Verify: Q × R should equal A
            match q.mul(&r) {
                Ok(product) => {
                    println!("\nVerification (Q × R = A):");
                    println!("{}", product);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Singular Value Decomposition
    println!("\n{}", repeat("=", 70));
    println!("2. SINGULAR VALUE DECOMPOSITION (SVD)");
    println!("{}", repeat("-", 70));

    let b_data = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0],
        vec![0.0, 0.0, 3.0],
    ];
    let b = OptimizedMatrix::new(b_data).expect("Failed to create matrix B");
    println!("Original matrix B:");
    println!("{}", b);

    match b.svd() {
        Ok((u, singular_values, vt)) => {
            println!("\nU matrix:");
            println!("{}", u);
            println!("\nSingular values: {:?}", singular_values);
            println!("\nV^T matrix:");
            println!("{}", vt);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 3: Eigenvalues and rank
    println!("\n{}", repeat("=", 70));
    println!("3. EIGENVALUES AND MATRIX RANK");
    println!("{}", repeat("-", 70));

    let c_data = vec![
        vec![4.0, 1.0],
        vec![1.0, 3.0],
    ];
    let c = OptimizedMatrix::new(c_data).expect("Failed to create matrix C");
    println!("Symmetric matrix C:");
    println!("{}", c);

    match c.eigenvalues() {
        Ok(evals) => {
            println!("\nEigenvalues: {:?}", evals);
        }
        Err(e) => println!("Error computing eigenvalues: {}", e),
    }

    match c.rank() {
        Ok(rank) => {
            println!("Rank of C: {}", rank);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 4: Linear System Solving
    println!("\n{}", repeat("=", 70));
    println!("4. LINEAR SYSTEM SOLVING");
    println!("{}", repeat("-", 70));

    let coeff_data = vec![
        vec![2.0, 1.0],
        vec![1.0, 3.0],
    ];
    let coeff = OptimizedMatrix::new(coeff_data).expect("Failed to create coefficient matrix");
    println!("Coefficient matrix A:");
    println!("{}", coeff);

    let rhs_data = vec![
        vec![5.0],
        vec![6.0],
    ];
    let rhs = OptimizedMatrix::new(rhs_data).expect("Failed to create RHS vector");
    println!("\nRight-hand side b:");
    println!("{}", rhs);

    match coeff.solve(&rhs) {
        Ok(solution) => {
            println!("\nSolution x (A × x = b):");
            println!("{}", solution);

            // Verify: A × x should equal b
            match coeff.mul(&solution) {
                Ok(verification) => {
                    println!("\nVerification (A × x = b):");
                    println!("{}", verification);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 5: Matrix Statistics
    println!("\n{}", repeat("=", 70));
    println!("5. MATRIX STATISTICS");
    println!("{}", repeat("-", 70));

    let d_data = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let d = OptimizedMatrix::new(d_data).expect("Failed to create matrix D");
    println!("Matrix D:");
    println!("{}", d);

    println!("\nMatrix Statistics:");
    println!("• Shape: {:?}", d.shape());
    println!("• Frobenius norm: {:.4}", d.frobenius_norm());
    
    match d.trace() {
        Ok(tr) => println!("• Trace: {}", tr),
        Err(e) => println!("• Trace: Error - {}", e),
    }

    match d.determinant() {
        Ok(det) => println!("• Determinant: {}", det),
        Err(e) => println!("• Determinant: Error - {}", e),
    }

    match d.rank() {
        Ok(rank) => println!("• Rank: {}", rank),
        Err(e) => println!("• Rank: Error - {}", e),
    }

    println!("\n{}", repeat("=", 70));
}
