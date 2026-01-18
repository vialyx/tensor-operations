# Week 13: ndarray and Tensor Operations - Complete Project Index

## 📋 Project Structure

```
tensor-operations/
├── README.md                              ← Start here! Project overview
├── QUICK_REFERENCE.md                    ← Quick reference guide
├── COMPLETION_SUMMARY.md                 ← Detailed project summary
├── WEEK13_STUDY_GUIDE.md                 ← Theory and concepts
│
├── manual_matrix.py                      ← Manual implementations (260 lines)
├── numpy_matrix.py                       ← NumPy implementations (200 lines)
│
├── Week13_ndarray_tensor_operations.ipynb ← Interactive notebook (20 cells)
└── performance_comparison.png            ← Benchmark visualization
```

## 🎯 Learning Path

### Stage 1: Understand the Concepts (30 min)
1. Start with **README.md** - Get the overview
2. Read **QUICK_REFERENCE.md** - Quick key concepts
3. Read **WEEK13_STUDY_GUIDE.md** (Sections 1-3) - Foundational knowledge

### Stage 2: See It in Action (45 min)
4. Review **manual_matrix.py** - Understand the algorithms
5. Review **numpy_matrix.py** - See the optimized version
6. Compare the two implementations side-by-side

### Stage 3: Interactive Learning (60 min)
7. Run the Jupyter notebook step-by-step
8. Execute cells 1-6 to see examples
9. Execute cells 7-8 to see benchmarks
10. Review visualization and insights

### Stage 4: Deepen Understanding (45 min)
11. Read **WEEK13_STUDY_GUIDE.md** (Sections 4+) - Advanced topics
12. Read **COMPLETION_SUMMARY.md** - Detailed analysis
13. Experiment with your own examples

**Total Time**: ~3 hours for comprehensive understanding

## 📖 Each File Explained

### README.md
- Project description and goals
- Feature overview
- Quick start examples
- Learning outcomes

### QUICK_REFERENCE.md
- One-page cheat sheet
- File purposes
- Code examples
- Troubleshooting tips

### WEEK13_STUDY_GUIDE.md
**Sections:**
1. Study Objectives
2. ndarray Fundamentals
   - What is ndarray
   - Key concepts (shape, dtype, memory layout)
   - Creation methods
3. Tensor Operations
   - Basic operations
   - Broadcasting rules
   - Matrix operations
   - Reduction operations
   - Reshaping and indexing
4. Performance Considerations
5. Mini Project Structure

### manual_matrix.py
**Classes:**
- `Matrix` - Manual matrix implementation

**Methods:**
- `__add__`, `__sub__`, `__mul__` - Basic operations
- `__matmul__` - Matrix multiplication
- `transpose()` - Transpose
- `determinant()` - Determinant (cofactor expansion)
- `inverse()` - Matrix inverse
- `frobenius_norm()` - Norm calculation
- `trace()` - Trace calculation

**Helper Functions:**
- `eye()`, `zeros()`, `ones()` - Matrix creation

### numpy_matrix.py
**Classes:**
- `NumpyMatrix` - NumPy-based implementation

**Methods:**
- Same basic operations as manual_matrix
- **Additional advanced methods:**
  - `eigenvalues()`, `eigen()` - Eigendecomposition
  - `rank()` - Matrix rank
  - `qr_decomposition()` - QR decomposition
  - `svd()` - Singular Value Decomposition
  - `column_space_basis()` - Column space
  - `null_space_basis()` - Null space
  - `solve()` - Linear system solving
  - `least_squares()` - Least squares solution
  - `broadcasting_example()` - Broadcasting demo

**Helper Functions:**
- `eye()`, `zeros()`, `ones()`
- `random_matrix()` - Random matrix generation
- `hilbert_matrix()` - Test matrix (ill-conditioned)

### Week13_ndarray_tensor_operations.ipynb
**Cell Breakdown:**
1. **Import Required Libraries** - Setup
2. **Understanding ndarrays and Tensors** - Theory
3. **Basic ndarray Operations** - Examples
4. **Tensor Operations** - Linear algebra examples
5. **Manual Matrix Operations** - Manual class usage
6. **NumPy Matrix Operations** - NumPy class usage
7-8. **Performance Comparison** - Benchmarks and visualization
9. **Advanced Tensor Operations** - Broadcasting, ufuncs
10. **Summary and Conclusions** - Key takeaways

### COMPLETION_SUMMARY.md
- What's included
- Benchmark results table
- Key findings
- Learning outcomes
- Further learning recommendations

## 🔑 Key Concepts Quick Lookup

### ndarray Properties
```python
array.shape      # Dimensions (e.g., (3, 4) for 3x4 matrix)
array.dtype      # Data type (float64, int32, etc.)
array.ndim       # Number of dimensions
array.size       # Total number of elements
array.itemsize   # Bytes per element
array.nbytes     # Total bytes used
```

### Broadcasting
```python
# Matrix + Vector broadcasting
matrix.shape = (3, 4)
vector.shape = (4,)
result = matrix + vector  # vector broadcasts to (3, 4)
```

### Key Operations
```python
# Creation
np.zeros((3, 4))
np.ones((2, 3))
np.eye(3)
np.random.rand(3, 4)

# Operations
A @ B              # Matrix multiplication
A + B              # Element-wise addition
A.T                # Transpose
np.linalg.det(A)   # Determinant
np.linalg.inv(A)   # Inverse
np.linalg.eig(A)   # Eigendecomposition
np.linalg.svd(A)   # SVD
```

## 📊 Benchmark Results at a Glance

| Operation | Size | Manual | NumPy | Speedup |
|-----------|------|--------|-------|---------|
| Addition | 100×100 | 0.0281s | 0.0003s | **82x** |
| Multiplication | 40×40 | 0.1179s | 0.0001s | **960x** |

## 🎓 Learning Outcomes

After completing this project, you can:

✅ Understand how NumPy ndarrays work internally  
✅ Implement matrix algorithms from scratch  
✅ Use NumPy efficiently for numerical computing  
✅ Understand broadcasting and vectorization  
✅ Make informed decisions about tool selection  
✅ Benchmark and optimize code  
✅ Solve linear systems and perform decompositions  
✅ Write production-ready numerical code  

## 💻 Hands-On Exercises

**Beginner:**
1. Create various ndarrays and explore their properties
2. Perform element-wise operations on different shaped arrays
3. Use broadcasting with a matrix and vector
4. Calculate determinant of a 3×3 matrix manually

**Intermediate:**
1. Implement matrix addition from scratch
2. Compare timing between manual and NumPy addition
3. Use QR decomposition to solve a linear system
4. Calculate eigenvalues of a matrix

**Advanced:**
1. Implement matrix inversion algorithm
2. Benchmark matrix multiplication at multiple sizes
3. Optimize a numerical computation using NumPy
4. Solve a least squares problem

## 🔗 External Resources

| Resource | URL | Topic |
|----------|-----|-------|
| NumPy Docs | numpy.org/doc | Official documentation |
| Broadcasting Guide | numpy.org/doc/.../broadcasting.html | Broadcasting rules |
| Linear Algebra | numpy.org/doc/.../linalg.html | Matrix operations |
| SciPy | scipy.org | Advanced scientific computing |

## ✨ Tips for Success

1. **Run the code** - Don't just read it, execute it
2. **Experiment** - Modify examples and see what happens
3. **Time it** - Actually measure the performance differences
4. **Visualize** - Plot the benchmark results
5. **Understand** - Know why NumPy is faster
6. **Practice** - Write your own implementations

---

## 🚀 Next Steps After Week 13

Once you've mastered this material:

1. **Explore SciPy** - More advanced numerical methods
2. **Learn Pandas** - Data manipulation with DataFrames
3. **Study TensorFlow/PyTorch** - Deep learning frameworks
4. **Optimize further** - Numba, Cython, GPU acceleration
5. **Build projects** - Apply to real-world problems

---

**Happy Learning!** 🎉

Last Updated: January 18, 2026
