# tensor-operations

### ❤️ Support This Project

If you find this project helpful, please consider supporting it:
- **[Donate via PayPal](https://www.paypal.com/paypalme/vialyx)** - Help keep this project maintained and improved

## Week 13: Matrix and Tensor Operations in Rust

A comprehensive learning project implementing matrix and linear algebra operations in Rust, with two complementary approaches:
- **Manual Implementation**: Pure Rust for understanding algorithms
- **Optimized Implementation**: nalgebra library for production-ready code

Perfect for learning both Rust fundamentals and linear algebra concepts through practical implementation.

### 🎯 Quick Start

**Prerequisites**: Rust 1.56+ installed ([install.rust-lang.org](https://www.rust-lang.org/install.html))

```bash
# Run examples
cargo run --example basic_operations
cargo run --example performance_comparison --release
cargo run --example advanced_operations

# Run tests
cargo test

# Run benchmarks
cargo bench

# View documentation
cargo doc --no-deps --open
```

### 📚 Documentation

1. **[RUST_GUIDE.md](RUST_GUIDE.md)** - Complete learning guide covering:
   - Manual vs optimized implementations
   - Algorithm explanations
   - Rust language fundamentals
   - Performance characteristics
   - Testing and benchmarking

2. **[WEEK13_STUDY_GUIDE.md](WEEK13_STUDY_GUIDE.md)** - Original Python study guide with:
   - Matrix operation fundamentals
   - Linear algebra concepts
   - Performance analysis methodology

### 🗂️ Project Structure

```
├── src/
│   ├── lib.rs              # Library root (module exports)
│   ├── manual.rs           # Pure Rust matrix implementation (~450 lines)
│   └── optimized.rs        # nalgebra-based implementation (~380 lines)
│
├── examples/
│   ├── basic_operations.rs        # Side-by-side comparison demo
│   ├── performance_comparison.rs  # Timing measurements
│   └── advanced_operations.rs     # QR, SVD, eigenvalues, solving
│
├── benches/
│   └── matrix_benchmarks.rs       # Criterion benchmarks
│
├── Cargo.toml              # Project configuration
└── README.md               # This file
```

### 📖 What You'll Learn

**Rust Concepts**:
- Ownership and borrowing
- Error handling with Result types
- Trait implementation (Display, Add, Sub, Mul, Index)
- Module system
- Testing framework
- Benchmarking

**Linear Algebra**:
- Matrix creation and manipulation
- Matrix arithmetic (addition, multiplication)
- Determinant and matrix inverse
- QR decomposition
- Singular Value Decomposition (SVD)
- Eigenvalues and eigenvectors
- Linear system solving

**Performance Engineering**:
- Algorithm complexity analysis
- Benchmarking techniques
- BLAS-optimized operations
- Memory layout impact on performance

### 🚀 Running Examples

#### 1. Basic Operations (Manual vs Optimized)
```bash
cargo run --example basic_operations
```
Shows side-by-side comparison of both implementations performing the same operations.

#### 2. Performance Comparison
```bash
cargo run --example performance_comparison --release
```
Measures execution time and calculates speedup of optimized vs manual implementation.

#### 3. Advanced Operations
```bash
cargo run --example advanced_operations
```
Demonstrates QR decomposition, SVD, eigenvalues, and linear system solving using optimized nalgebra backend.

### 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_matrix_creation

# Run specific module tests
cargo test manual::
cargo test optimized::
```

**Test Coverage**:
- Manual implementation: 17 tests covering all operations
- Optimized implementation: 11 tests
- Total: 28 unit tests

### 📊 Benchmarking

```bash
# Run criterion benchmarks
cargo bench

# Benchmark specific operation
cargo bench benchmark_addition
```

Benchmarks test performance across multiple matrix sizes and generate HTML reports in `target/criterion/`.

### 💡 Key Code Examples

**Creating a Matrix**:
```rust
use tensor_operations::manual::Matrix;

let m = Matrix::new(vec![
    vec![1.0, 2.0],
    vec![3.0, 4.0],
])?;
```

**Matrix Operations**:
```rust
let sum = a.add(&b)?;
let product = a.mul(&b)?;
let det = a.determinant()?;
let inverse = a.inverse()?;
```

**Advanced Operations**:
```rust
use tensor_operations::optimized::OptimizedMatrix;

let (q, r) = m.qr_decomposition()?;
let (u, singular_vals, vt) = m.svd()?;
let evals = m.eigenvalues()?;
let x = a.solve(&b)?;
```

### ⚡ Performance Overview

Typical speedups (optimized vs manual) on modern hardware:

| Operation | Size | Manual | Optimized | Speedup |
|-----------|------|--------|-----------|---------|
| Addition | 10×10 | ~5 µs | ~0.7 µs | **7x** |
| Multiplication | 20×20 | ~16 ms | ~150 µs | **100x** |
| Determinant | 5×5 | ~50 µs | ~5 µs | **10x** |
| Transpose | 50×50 | ~10 µs | ~2 µs | **5x** |

See [RUST_GUIDE.md](RUST_GUIDE.md) for detailed performance analysis.

### 📦 Dependencies

- **nalgebra 0.32**: High-performance linear algebra (BLAS-optimized)
- **ndarray 0.15**: N-dimensional arrays
- **serde 1.0**: Serialization framework
- **criterion 0.5**: Benchmarking (dev)
- **rand 0.8**: Random numbers (dev)

### 📚 Educational Path

1. **Days 1-2**: Learn Rust fundamentals
2. **Days 3-4**: Study manual implementation (`src/manual.rs`)
3. **Days 5-6**: Study optimized implementation (`src/optimized.rs`)
4. **Day 7**: Run benchmarks and analyze results

See [RUST_GUIDE.md](RUST_GUIDE.md) for detailed learning plan.

### 🔗 Further Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [nalgebra Documentation](https://nalgebra.org/)
- [3Blue1Brown - Essence of Linear Algebra](https://www.youtube.com/c/3Blue1Brown)
- [Linear Algebra - MIT OpenCourseWare](https://ocw.mit.edu/courses/18-06-linear-algebra-spring-2010/)

### 📋 File-by-File Guide

| File | Lines | Purpose |
|------|-------|---------|
| `src/manual.rs` | 450+ | Pure Rust implementation, 17 tests |
| `src/optimized.rs` | 380+ | nalgebra wrapper, 11 tests |
| `src/lib.rs` | 44 | Library root with exports |
| `examples/basic_operations.rs` | 180+ | Basic usage demonstration |
| `examples/performance_comparison.rs` | 160+ | Timing measurements |
| `examples/advanced_operations.rs` | 210+ | QR, SVD, eigenvalues |
| `benches/matrix_benchmarks.rs` | 120+ | Criterion benchmarks |

### 🎓 Learning Outcomes

Upon completing this project, you will understand:

✅ How matrix operations work at a fundamental level  
✅ Rust ownership, borrowing, and error handling  
✅ Algorithm complexity and performance optimization  
✅ When to use optimized libraries vs. manual implementation  
✅ How to benchmark and measure performance  
✅ Best practices for numerical computing in Rust  

### 📄 Python Version

This project also includes the original Python implementation for comparison:
- `WEEK13_STUDY_GUIDE.md` - Study materials
- `manual_matrix.py` - Manual implementation
- `numpy_matrix.py` - NumPy implementation
- `Week13_ndarray_tensor_operations.ipynb` - Jupyter notebook

### 📝 License

MIT License
- Speedup increases with matrix size
- BLAS/LAPACK optimizations provide major advantages

### 🚀 Quick Start

```python
# Manual implementation
from manual_matrix import Matrix
A = Matrix([[1, 2], [3, 4]])
B = Matrix([[5, 6], [7, 8]])
C = A @ B  # Matrix multiplication

# NumPy implementation
from numpy_matrix import NumpyMatrix
A = NumpyMatrix([[1, 2], [3, 4]])
B = NumpyMatrix([[5, 6], [7, 8]])
C = A @ B  # Same operation, much faster
```

### 📊 Project Structure

```
tensor-operations/
├── README.md                              # This file
├── WEEK13_STUDY_GUIDE.md                 # Comprehensive study guide
├── manual_matrix.py                      # Manual implementations
├── numpy_matrix.py                       # NumPy implementations
├── Week13_ndarray_tensor_operations.ipynb # Interactive notebook
└── performance_comparison.png            # Benchmark visualization
```

### 💡 Learning Outcomes

By completing this week's materials, you will:
1. ✅ Understand ndarray structure and capabilities
2. ✅ Master tensor operations and broadcasting
3. ✅ Implement matrix operations from scratch
4. ✅ Optimize code using NumPy
5. ✅ Compare performance of different approaches
6. ✅ Know when to use manual vs library implementations

### 📖 Topics Covered

- ndarray creation and initialization
- Tensor operations and linear algebra
- Broadcasting and vectorization
- Matrix operations: addition, subtraction, multiplication, transpose
- Advanced operations: determinant, inverse, eigenvalues, SVD
- Performance benchmarking and analysis
- Memory efficiency considerations
- Best practices for numerical computing