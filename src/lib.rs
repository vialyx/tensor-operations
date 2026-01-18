//! # Tensor Operations: Matrix and Linear Algebra in Rust
//!
//! This library provides two implementations of matrix operations:
//!
//! 1. **Manual Implementation** (`manual` module): Pure Rust implementation from scratch
//!    - Educational purpose to understand algorithms
//!    - No external dependencies for core operations
//!    - Good for learning linear algebra concepts
//!
//! 2. **Optimized Implementation** (`optimized` module): High-performance using nalgebra
//!    - Production-ready code
//!    - Uses BLAS-optimized linear algebra
//!    - 10-1000x faster than manual implementation
//!
//! ## Quick Start
//!
//! ```no_run
//! use tensor_operations::manual::Matrix;
//!
//! // Create a matrix
//! let a = Matrix::new(vec![
//!     vec![1.0, 2.0],
//!     vec![3.0, 4.0],
//! ]).unwrap();
//!
//! // Matrix operations
//! let b = Matrix::new(vec![
//!     vec![5.0, 6.0],
//!     vec![7.0, 8.0],
//! ]).unwrap();
//!
//! let sum = a.add(&b).unwrap();
//! let product = a.mul(&b).unwrap();
//! let det = a.determinant().unwrap();
//! let inv = a.inverse().unwrap();
//! ```

pub mod manual;
pub mod optimized;

// Re-export commonly used types
pub use manual::Matrix;
pub use optimized::OptimizedMatrix;
