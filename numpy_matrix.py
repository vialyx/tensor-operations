"""
NumPy-based Matrix Operations Implementation
Demonstrates matrix operations using NumPy ndarray for comparison
"""

import numpy as np
from typing import Tuple, Union


class NumpyMatrix:
    """
    Matrix class using NumPy for efficient operations.
    Provides the same interface as manual Matrix for easy comparison.
    """
    
    def __init__(self, data: Union[np.ndarray, list]):
        """
        Initialize a matrix from NumPy array or list.
        
        Args:
            data: NumPy array or list of lists
        """
        if isinstance(data, list):
            self.data = np.array(data, dtype=np.float64)
        else:
            self.data = np.asarray(data, dtype=np.float64)
        
        if self.data.ndim != 2:
            raise ValueError("Data must be 2-dimensional")
        
        self.rows, self.cols = self.data.shape
    
    def __repr__(self) -> str:
        """String representation of matrix."""
        return str(self.data)
    
    def __str__(self) -> str:
        """String representation of matrix."""
        return str(self.data)
    
    def __add__(self, other: 'NumpyMatrix') -> 'NumpyMatrix':
        """Element-wise addition."""
        if self.rows != other.rows or self.cols != other.cols:
            raise ValueError("Matrices must have the same dimensions for addition")
        return NumpyMatrix(self.data + other.data)
    
    def __sub__(self, other: 'NumpyMatrix') -> 'NumpyMatrix':
        """Element-wise subtraction."""
        if self.rows != other.rows or self.cols != other.cols:
            raise ValueError("Matrices must have the same dimensions for subtraction")
        return NumpyMatrix(self.data - other.data)
    
    def __mul__(self, scalar: float) -> 'NumpyMatrix':
        """Scalar multiplication."""
        if not isinstance(scalar, (int, float)):
            raise TypeError("Can only multiply by scalar")
        return NumpyMatrix(self.data * scalar)
    
    def __rmul__(self, scalar: float) -> 'NumpyMatrix':
        """Right multiplication (scalar * matrix)."""
        return self.__mul__(scalar)
    
    def __matmul__(self, other: 'NumpyMatrix') -> 'NumpyMatrix':
        """Matrix multiplication (A @ B)."""
        if self.cols != other.rows:
            raise ValueError(
                f"Cannot multiply matrices: "
                f"({self.rows}x{self.cols}) @ ({other.rows}x{other.cols})"
            )
        return NumpyMatrix(self.data @ other.data)
    
    def transpose(self) -> 'NumpyMatrix':
        """Return the transpose of the matrix."""
        return NumpyMatrix(self.data.T)
    
    def determinant(self) -> float:
        """Calculate determinant."""
        if self.rows != self.cols:
            raise ValueError("Determinant only defined for square matrices")
        return float(np.linalg.det(self.data))
    
    def is_square(self) -> bool:
        """Check if matrix is square."""
        return self.rows == self.cols
    
    def is_invertible(self) -> bool:
        """Check if matrix is invertible."""
        if not self.is_square():
            return False
        return abs(self.determinant()) > 1e-10
    
    def inverse(self) -> 'NumpyMatrix':
        """Calculate matrix inverse."""
        if not self.is_invertible():
            raise ValueError("Matrix is not invertible")
        return NumpyMatrix(np.linalg.inv(self.data))
    
    def frobenius_norm(self) -> float:
        """Calculate Frobenius norm."""
        return float(np.linalg.norm(self.data, 'fro'))
    
    def trace(self) -> float:
        """Calculate trace."""
        if not self.is_square():
            raise ValueError("Trace is only defined for square matrices")
        return float(np.trace(self.data))
    
    def eigenvalues(self) -> np.ndarray:
        """Calculate eigenvalues."""
        if not self.is_square():
            raise ValueError("Eigenvalues only defined for square matrices")
        return np.linalg.eigvals(self.data)
    
    def eigen(self) -> Tuple[np.ndarray, 'NumpyMatrix']:
        """
        Calculate eigenvalues and eigenvectors.
        
        Returns:
            Tuple of (eigenvalues, eigenvectors as NumpyMatrix)
        """
        if not self.is_square():
            raise ValueError("Eigendecomposition only defined for square matrices")
        eigenvalues, eigenvectors = np.linalg.eig(self.data)
        return eigenvalues, NumpyMatrix(eigenvectors)
    
    def rank(self) -> int:
        """Calculate matrix rank."""
        return int(np.linalg.matrix_rank(self.data))
    
    def qr_decomposition(self) -> Tuple['NumpyMatrix', 'NumpyMatrix']:
        """
        Calculate QR decomposition.
        
        Returns:
            Tuple of (Q, R) where A = Q @ R
        """
        Q, R = np.linalg.qr(self.data)
        return NumpyMatrix(Q), NumpyMatrix(R)
    
    def svd(self) -> Tuple['NumpyMatrix', np.ndarray, 'NumpyMatrix']:
        """
        Singular Value Decomposition.
        
        Returns:
            Tuple of (U, singular_values, V^T)
        """
        U, s, Vt = np.linalg.svd(self.data, full_matrices=False)
        return NumpyMatrix(U), s, NumpyMatrix(Vt)
    
    def column_space_basis(self) -> 'NumpyMatrix':
        """Get orthonormal basis for column space using QR decomposition."""
        Q, R = self.qr_decomposition()
        return Q
    
    def null_space_basis(self) -> 'NumpyMatrix':
        """Get orthonormal basis for null space."""
        U, s, Vt = self.svd()
        # Null space corresponds to singular vectors with zero singular values
        rank = self.rank()
        null_space = Vt[rank:, :].T
        return NumpyMatrix(null_space) if null_space.size > 0 else None
    
    def solve(self, b: Union['NumpyMatrix', np.ndarray]) -> 'NumpyMatrix':
        """
        Solve the linear system Ax = b.
        
        Args:
            b: Right-hand side vector or matrix
            
        Returns:
            Solution x such that A @ x = b
        """
        if isinstance(b, NumpyMatrix):
            b_data = b.data
        else:
            b_data = np.asarray(b)
        
        return NumpyMatrix(np.linalg.solve(self.data, b_data))
    
    def least_squares(self, b: Union['NumpyMatrix', np.ndarray]) -> Tuple['NumpyMatrix', float]:
        """
        Solve least squares problem: minimize ||Ax - b||_2.
        
        Args:
            b: Right-hand side vector or matrix
            
        Returns:
            Tuple of (solution x, residual norm)
        """
        if isinstance(b, NumpyMatrix):
            b_data = b.data
        else:
            b_data = np.asarray(b)
        
        x, residuals, rank, s = np.linalg.lstsq(self.data, b_data, rcond=None)
        residual_norm = float(np.linalg.norm(residuals)) if residuals.size > 0 else 0.0
        return NumpyMatrix(x.reshape(-1, 1)) if x.ndim == 1 else NumpyMatrix(x), residual_norm
    
    def broadcasting_example(self, vector: np.ndarray) -> 'NumpyMatrix':
        """
        Demonstrate broadcasting: add vector to each row.
        
        Args:
            vector: 1D array to add to each row
            
        Returns:
            Result with vector added to each row
        """
        if len(vector) != self.cols:
            raise ValueError(f"Vector size ({len(vector)}) must match number of columns ({self.cols})")
        return NumpyMatrix(self.data + vector)


def eye(n: int) -> NumpyMatrix:
    """Create identity matrix of size n x n."""
    return NumpyMatrix(np.eye(n))


def zeros(rows: int, cols: int) -> NumpyMatrix:
    """Create zero matrix of size rows x cols."""
    return NumpyMatrix(np.zeros((rows, cols)))


def ones(rows: int, cols: int) -> NumpyMatrix:
    """Create matrix of ones of size rows x cols."""
    return NumpyMatrix(np.ones((rows, cols)))


def random_matrix(rows: int, cols: int, seed: int = None) -> NumpyMatrix:
    """Create random matrix."""
    if seed is not None:
        np.random.seed(seed)
    return NumpyMatrix(np.random.randn(rows, cols))


def hilbert_matrix(n: int) -> NumpyMatrix:
    """Create Hilbert matrix (ill-conditioned for testing)."""
    H = np.zeros((n, n))
    for i in range(n):
        for j in range(n):
            H[i, j] = 1.0 / (i + j + 1)
    return NumpyMatrix(H)
