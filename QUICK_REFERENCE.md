# Week 13: Quick Reference Guide

## 🎯 Project Overview
Study ndarray and tensor operations with a mini project comparing manual vs. NumPy implementations.

## 📂 Files at a Glance

| File | Purpose | Lines |
|------|---------|-------|
| `WEEK13_STUDY_GUIDE.md` | Complete theory and concepts | ~300 |
| `manual_matrix.py` | From-scratch implementation | ~260 |
| `numpy_matrix.py` | NumPy-based implementation | ~200 |
| `Week13_ndarray_tensor_operations.ipynb` | Interactive notebook | 20 cells |
| `COMPLETION_SUMMARY.md` | Project summary and results | ~300 |

## 🚀 Quick Start

### Run the Notebook
```bash
jupyter notebook Week13_ndarray_tensor_operations.ipynb
```

### Use the Matrix Classes
```python
# Manual implementation
from manual_matrix import Matrix
A = Matrix([[1, 2], [3, 4]])
B = Matrix([[5, 6], [7, 8]])
C = A @ B

# NumPy implementation
from numpy_matrix import NumpyMatrix
A_np = NumpyMatrix([[1, 2], [3, 4]])
B_np = NumpyMatrix([[5, 6], [7, 8]])
C_np = A_np @ B_np
```

## 📊 Benchmark Highlights

- **Matrix Addition**: 7x - 82x speedup
- **Matrix Multiplication**: 7x - 960x speedup
- **Larger matrices show exponential speedup advantage**

## 🧠 Core Concepts

### ndarray
- Homogeneous N-dimensional arrays
- Memory-efficient storage
- Support for vectorized operations
- Broadcasting for shape-compatible operations

### Broadcasting
```python
matrix = np.array([[1, 2, 3], [4, 5, 6]])  # shape (2, 3)
vector = np.array([1, 10, 100])             # shape (3,)
result = matrix + vector                    # broadcasts to (2, 3)
```

### Operations Covered
- ✅ Addition/Subtraction
- ✅ Scalar multiplication
- ✅ Matrix multiplication
- ✅ Transpose
- ✅ Determinant
- ✅ Inverse
- ✅ Eigenvalues/Eigenvectors
- ✅ SVD, QR decomposition
- ✅ Matrix rank and trace

## 💡 Key Takeaways

1. **NumPy is ~100x faster** for typical operations
2. **Manual implementations** useful for learning algorithms
3. **Broadcasting** eliminates loops and simplifies code
4. **Memory efficiency** is 3-5x better with NumPy
5. **Choose the right tool** based on use case

## 🔍 When to Use What

| Scenario | Tool | Reason |
|----------|------|--------|
| Learning algorithms | Manual | Understand implementation details |
| Small experiments | Manual | Acceptable speed, educational |
| Production code | NumPy | 100x+ faster, optimized |
| Large datasets | NumPy | Essential for performance |
| Scientific computing | NumPy/SciPy | Specialized algorithms |
| Deep learning | PyTorch/TensorFlow | GPU support, automatic differentiation |

## 📚 Study Order

1. Read `WEEK13_STUDY_GUIDE.md` - Build foundational knowledge
2. Review `manual_matrix.py` - Understand algorithm implementation
3. Review `numpy_matrix.py` - See optimized version
4. Run the Jupyter notebook - Interactive learning with execution
5. Experiment and modify - Try your own examples

## 🎓 Learning Objectives Checklist

- [ ] Understand ndarray structure and properties
- [ ] Master broadcasting and vectorization
- [ ] Implement basic matrix operations manually
- [ ] Use NumPy effectively
- [ ] Understand performance differences
- [ ] Make informed tool selection decisions

## 🔗 External Resources

- [NumPy Documentation](https://numpy.org/doc/)
- [NumPy Broadcasting](https://numpy.org/doc/stable/user/basics.broadcasting.html)
- [Linear Algebra (numpy.linalg)](https://numpy.org/doc/stable/reference/routines.linalg.html)
- [Python Performance Tips](https://wiki.python.org/moin/PythonSpeed)

## ❓ Troubleshooting

### Notebook won't run?
```bash
pip install numpy matplotlib seaborn
```

### Import errors?
```bash
python -c "import numpy; print(numpy.__version__)"
```

### Performance seems wrong?
- Make sure you're using the correct implementation
- Check that loops are properly contained in the manual version
- Verify NumPy is using system BLAS (should be automatic)

---

**Happy learning!** 🎉
