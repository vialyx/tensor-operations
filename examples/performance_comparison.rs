/// Example: Performance Comparison
/// Demonstrates timing difference between manual and optimized implementations

use std::time::Instant;
use tensor_operations::manual::Matrix;
use tensor_operations::optimized::OptimizedMatrix;

fn repeat(s: &str, n: usize) -> String {
    (0..n).map(|_| s).collect::<String>()
}

fn create_manual_matrix(size: usize) -> Matrix {
    let mut data = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            row.push((i * size + j) as f64);
        }
        data.push(row);
    }
    Matrix::new(data).unwrap()
}

fn create_optimized_matrix(size: usize) -> OptimizedMatrix {
    let mut data = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            row.push((i * size + j) as f64);
        }
        data.push(row);
    }
    OptimizedMatrix::new(data).unwrap()
}

fn main() {
    println!("{}", repeat("=", 70));
    println!("Performance Comparison: Manual vs Optimized Implementation");
    println!("{}", repeat("=", 70));

    let sizes = vec![10, 20, 50];
    let mut results = Vec::new();

    for size in &sizes {
        println!("\n{} Testing {}×{} matrices {} times", repeat("-", 10), size, size, 10);

        let a_manual = create_manual_matrix(*size);
        let b_manual = create_manual_matrix(*size);

        let a_opt = create_optimized_matrix(*size);
        let b_opt = create_optimized_matrix(*size);

        // Benchmark addition
        let start = Instant::now();
        for _ in 0..10 {
            let _ = a_manual.add(&b_manual);
        }
        let manual_add = start.elapsed();

        let start = Instant::now();
        for _ in 0..10 {
            let _ = a_opt.add(&b_opt);
        }
        let opt_add = start.elapsed();

        let speedup = manual_add.as_secs_f64() / opt_add.as_secs_f64();

        println!(
            "Addition:    Manual: {:>10.6}s | Optimized: {:>10.6}s | Speedup: {:>6.1}x",
            manual_add.as_secs_f64(),
            opt_add.as_secs_f64(),
            speedup
        );

        results.push((
            *size,
            manual_add.as_secs_f64(),
            opt_add.as_secs_f64(),
            speedup,
        ));

        // Benchmark multiplication (only for smaller matrices due to time)
        if *size <= 50 {
            let start = Instant::now();
            for _ in 0..5 {
                let _ = a_manual.mul(&b_manual);
            }
            let manual_mul = start.elapsed();

            let start = Instant::now();
            for _ in 0..5 {
                let _ = a_opt.mul(&b_opt);
            }
            let opt_mul = start.elapsed();

            let speedup_mul = manual_mul.as_secs_f64() / opt_mul.as_secs_f64();

            println!(
                "Multiplication (5x): Manual: {:>10.6}s | Optimized: {:>10.6}s | Speedup: {:>6.1}x",
                manual_mul.as_secs_f64(),
                opt_mul.as_secs_f64(),
                speedup_mul
            );
        }

        // Benchmark transpose
        let start = Instant::now();
        for _ in 0..20 {
            let _ = a_manual.transpose();
        }
        let manual_trans = start.elapsed();

        let start = Instant::now();
        for _ in 0..20 {
            let _ = a_opt.transpose();
        }
        let opt_trans = start.elapsed();

        let speedup_trans = manual_trans.as_secs_f64() / opt_trans.as_secs_f64();

        println!(
            "Transpose (20x):    Manual: {:>10.6}s | Optimized: {:>10.6}s | Speedup: {:>6.1}x",
            manual_trans.as_secs_f64(),
            opt_trans.as_secs_f64(),
            speedup_trans
        );
    }

    println!("\n{}", repeat("=", 70));
    println!("Summary: Addition Performance");
    println!("{}", repeat("-", 70));
    println!("{:<10} {:<15} {:<15} {:<15}", "Size", "Manual (s)", "Optimized (s)", "Speedup");
    println!("{}", repeat("-", 70));

    for (size, manual, opt, speedup) in &results {
        println!("{:<10} {:<15.6} {:<15.6} {:<15.1}x", size, manual, opt, speedup);
    }

    println!("{}", repeat("=", 70));
    println!("\nKey Observations:");
    println!("• Optimized implementation uses nalgebra with BLAS optimization");
    println!("• Performance gap increases with matrix size");
    println!("• Manual implementation: Good for learning, slow for computation");
    println!("• Optimized implementation: Production-ready and efficient");
    println!("{}", repeat("=", 70));
}
