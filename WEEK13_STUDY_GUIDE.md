# Week 13: ndarray and Tensor Operations

## Study Objectives
- Understand ndarray fundamentals and memory layout
- Master tensor operations and broadcasting
- Compare manual implementations with optimized libraries
- Learn performance considerations

## 1. ndarray Fundamentals

### What is ndarray?
- N-dimensional array object from NumPy
- Homogeneous collection of elements (same dtype)
- Core data structure for numerical computing in Python

### Key Concepts

#### Shape and Dimensions
```python
import numpy as np

# 1D array
a = np.array([1, 2, 3])
print(a.shape)  # (3,)

# 2D array (matrix)
b = np.array([[1, 2, 3], [4, 5, 6]])
print(b.shape)  # (2, 3)

# 3D array (tensor)
c = np.zeros((2, 3, 4))
print(c.shape)  # (2, 3, 4)
```

#### Data Types (dtype)
- int8, int16, int32, int64
- float16, float32, float64
- complex64, complex128
- bool

#### Memory Layout
- **C-contiguous** (row-major): default, rows stored sequentially
- **F-contiguous** (column-major): columns stored sequentially
- Affects performance of operations

### Creation Methods
```python
# From list
a = np.array([1, 2, 3])

# Predefined values
zeros = np.zeros((3, 3))
ones = np.ones((2, 4))
empty = np.empty((3, 3))

# Range and intervals
b = np.arange(0, 10, 2)  # [0, 2, 4, 6, 8]
c = np.linspace(0, 1, 5)  # [0, 0.25, 0.5, 0.75, 1.0]

# Random
d = np.random.rand(3, 3)  # Random values [0, 1)
e = np.random.randn(3, 3) # Standard normal distribution
f = np.random.randint(0, 10, (3, 3))
```

## 2. Tensor Operations

### Basic Operations
All operations are **element-wise** by default:
```python
a = np.array([[1, 2], [3, 4]])
b = np.array([[5, 6], [7, 8]])

add = a + b          # Element-wise addition
sub = a - b          # Element-wise subtraction
mul = a * b          # Element-wise multiplication
div = a / b          # Element-wise division
power = a ** 2       # Element-wise power
```

### Broadcasting
Automatically extends operations to compatible shapes:
```python
a = np.array([[1, 2, 3], [4, 5, 6]])  # (2, 3)
b = np.array([1, 2, 3])                 # (3,)

result = a + b  # b broadcasts to (2, 3)
# [[2, 4, 6],
#  [5, 7, 9]]
```

**Broadcasting Rules:**
1. If arrays have different number of dimensions, pad the smaller with 1s on the left
2. Arrays are compatible if dimensions match or one is 1
3. Dimension with size 1 stretches to match the other

### Matrix Operations
```python
A = np.array([[1, 2], [3, 4]])
B = np.array([[5, 6], [7, 8]])

# Matrix multiplication
dot_product = np.dot(A, B)
# or A @ B

# Transpose
A_T = A.T
# or np.transpose(A)

# Determinant
det = np.linalg.det(A)

# Inverse
A_inv = np.linalg.inv(A)

# Solve linear system Ax = b
b = np.array([1, 2])
x = np.linalg.solve(A, b)

# Eigenvalues and eigenvectors
eigenvalues, eigenvectors = np.linalg.eig(A)

# Matrix rank
rank = np.linalg.matrix_rank(A)
```

### Reduction Operations
```python
a = np.array([[1, 2, 3], [4, 5, 6]])

# Sum
sum_all = np.sum(a)           # 21
sum_rows = np.sum(a, axis=0)  # [5, 7, 9]
sum_cols = np.sum(a, axis=1)  # [6, 15]

# Mean, median, std
mean_val = np.mean(a)
median_val = np.median(a)
std_val = np.std(a)

# Min, max
min_val = np.min(a)
max_val = np.max(a)

# Argmin, argmax
idx_min = np.argmin(a)
idx_max = np.argmax(a)
```

### Reshaping and Indexing
```python
a = np.arange(12)

# Reshape
reshaped = a.reshape(3, 4)

# Flatten
flat = reshaped.flatten()

# Indexing
element = a[0]           # First element
row = a[0:2]             # Rows 0-1
subset = a[1:5:2]        # Elements 1, 3 (step 2)

# 2D indexing
matrix = np.arange(12).reshape(3, 4)
element = matrix[0, 1]   # Row 0, column 1
row = matrix[0, :]       # First row
col = matrix[:, 1]       # Second column

# Boolean indexing
mask = a > 5
filtered = a[mask]
```

## 3. Performance Considerations

### Why Use ndarray?
1. **Speed**: Operations implemented in C
2. **Memory Efficiency**: Contiguous memory layout
3. **Convenience**: Vectorized operations eliminate loops
4. **Functionality**: Extensive linear algebra library

### Performance Tips
1. **Vectorize operations** - avoid Python loops
2. **Use appropriate dtypes** - float32 when precision allows
3. **Reuse arrays** - avoid unnecessary copying
4. **Consider memory layout** - operations on rows faster than columns
5. **Use in-place operations** - `a += b` instead of `a = a + b`

### Typical Performance Improvements
- Manual loops: baseline (slow)
- Pure NumPy: 10-100x faster
- Optimized BLAS (linear algebra): 100-1000x faster

## 4. Mini Project Structure

### Part 1: Manual Implementation
- Basic matrix class
- Operations: add, subtract, multiply, transpose
- Determinant calculation
- Matrix inversion

### Part 2: NumPy Implementation
- Same operations using ndarray
- Performance comparison
- Benchmarking

### Part 3: Analysis
- Speed comparison charts
- Memory usage analysis
- When to use each approach

## Further Resources
- [NumPy Documentation](https://numpy.org/doc/)
- [NumPy Broadcasting Guide](https://numpy.org/doc/stable/user/basics.broadcasting.html)
- [Linear Algebra (numpy.linalg)](https://numpy.org/doc/stable/reference/routines.linalg.html)
