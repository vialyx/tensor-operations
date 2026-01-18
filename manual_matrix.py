"""
Manual Matrix Operations Implementation
Demonstrates matrix operations implemented from scratch without NumPy
"""

from typing import List, Tuple
import math


class Matrix:
    """A simple matrix class implementing basic linear algebra operations."""
    
    def __init__(self, data: List[List[float]]):
        """
        Initialize a matrix from a list of lists.
        
        Args:
            data: List of lists where each inner list is a row
            
        Raises:
            ValueError: If rows have different lengths
        """
        if not data or not data[0]:
            raise ValueError("Matrix cannot be empty")
        
        # Validate all rows have same length
        row_length = len(data[0])
        for row in data:
            if len(row) != row_length:
                raise ValueError("All rows must have the same length")
        
        self.data = data
        self.rows = len(data)
        self.cols = len(data[0])
    
    def __repr__(self) -> str:
        """String representation of matrix."""
        lines = []
        for row in self.data:
            formatted_row = " ".join(f"{val:8.2f}" for val in row)
            lines.append(f"[{formatted_row}]")
        return "\n".join(lines)
    
    def __str__(self) -> str:
        """String representation of matrix."""
        return self.__repr__()
    
    def __add__(self, other: 'Matrix') -> 'Matrix':
        """Element-wise addition."""
        if self.rows != other.rows or self.cols != other.cols:
            raise ValueError("Matrices must have the same dimensions for addition")
        
        result = []
        for i in range(self.rows):
            row = []
            for j in range(self.cols):
                row.append(self.data[i][j] + other.data[i][j])
            result.append(row)
        return Matrix(result)
    
    def __sub__(self, other: 'Matrix') -> 'Matrix':
        """Element-wise subtraction."""
        if self.rows != other.rows or self.cols != other.cols:
            raise ValueError("Matrices must have the same dimensions for subtraction")
        
        result = []
        for i in range(self.rows):
            row = []
            for j in range(self.cols):
                row.append(self.data[i][j] - other.data[i][j])
            result.append(row)
        return Matrix(result)
    
    def __mul__(self, scalar: float) -> 'Matrix':
        """Scalar multiplication."""
        if not isinstance(scalar, (int, float)):
            raise TypeError("Can only multiply by scalar")
        
        result = []
        for i in range(self.rows):
            row = []
            for j in range(self.cols):
                row.append(self.data[i][j] * scalar)
            result.append(row)
        return Matrix(result)
    
    def __rmul__(self, scalar: float) -> 'Matrix':
        """Right multiplication (scalar * matrix)."""
        return self.__mul__(scalar)
    
    def __matmul__(self, other: 'Matrix') -> 'Matrix':
        """Matrix multiplication (A @ B)."""
        if self.cols != other.rows:
            raise ValueError(
                f"Cannot multiply matrices: "
                f"({self.rows}x{self.cols}) @ ({other.rows}x{other.cols})"
            )
        
        result = []
        for i in range(self.rows):
            row = []
            for j in range(other.cols):
                # Calculate dot product of row i and column j
                value = sum(self.data[i][k] * other.data[k][j] 
                           for k in range(self.cols))
                row.append(value)
            result.append(row)
        return Matrix(result)
    
    def transpose(self) -> 'Matrix':
        """Return the transpose of the matrix."""
        result = []
        for j in range(self.cols):
            row = []
            for i in range(self.rows):
                row.append(self.data[i][j])
            result.append(row)
        return Matrix(result)
    
    def get_minor(self, row: int, col: int) -> 'Matrix':
        """Get matrix with specified row and column removed."""
        result = []
        for i in range(self.rows):
            if i == row:
                continue
            new_row = []
            for j in range(self.cols):
                if j == col:
                    continue
                new_row.append(self.data[i][j])
            result.append(new_row)
        return Matrix(result)
    
    def determinant(self) -> float:
        """Calculate determinant (recursive implementation for small matrices)."""
        if self.rows != self.cols:
            raise ValueError("Determinant only defined for square matrices")
        
        n = self.rows
        
        # Base case: 1x1 matrix
        if n == 1:
            return self.data[0][0]
        
        # Base case: 2x2 matrix
        if n == 2:
            return (self.data[0][0] * self.data[1][1] - 
                   self.data[0][1] * self.data[1][0])
        
        # Recursive case: use cofactor expansion along first row
        det = 0
        for j in range(n):
            minor = self.get_minor(0, j)
            cofactor = ((-1) ** j) * self.data[0][j] * minor.determinant()
            det += cofactor
        
        return det
    
    def is_square(self) -> bool:
        """Check if matrix is square."""
        return self.rows == self.cols
    
    def is_invertible(self) -> bool:
        """Check if matrix is invertible (determinant != 0)."""
        if not self.is_square():
            return False
        return abs(self.determinant()) > 1e-10
    
    def inverse(self) -> 'Matrix':
        """Calculate matrix inverse using adjugate matrix method."""
        if not self.is_invertible():
            raise ValueError("Matrix is not invertible (determinant is zero or near-zero)")
        
        n = self.rows
        det = self.determinant()
        
        # Special case: 2x2 matrix
        if n == 2:
            result = [[self.data[1][1] / det, -self.data[0][1] / det],
                     [-self.data[1][0] / det, self.data[0][0] / det]]
            return Matrix(result)
        
        # General case: use adjugate matrix method
        # Step 1: Calculate matrix of minors
        minors = []
        for i in range(n):
            row = []
            for j in range(n):
                minor = self.get_minor(i, j)
                row.append(minor.determinant())
            minors.append(row)
        minors_matrix = Matrix(minors)
        
        # Step 2: Apply checkerboard of signs to get cofactor matrix
        cofactors = []
        for i in range(n):
            row = []
            for j in range(n):
                sign = (-1) ** (i + j)
                row.append(sign * minors[i][j])
            cofactors.append(row)
        cofactor_matrix = Matrix(cofactors)
        
        # Step 3: Transpose to get adjugate matrix
        adjugate = cofactor_matrix.transpose()
        
        # Step 4: Divide by determinant
        result = adjugate * (1 / det)
        
        return result
    
    def frobenius_norm(self) -> float:
        """Calculate Frobenius norm (sqrt of sum of squared elements)."""
        sum_squares = sum(self.data[i][j] ** 2 
                         for i in range(self.rows) 
                         for j in range(self.cols))
        return math.sqrt(sum_squares)
    
    def trace(self) -> float:
        """Calculate trace (sum of diagonal elements)."""
        if not self.is_square():
            raise ValueError("Trace is only defined for square matrices")
        return sum(self.data[i][i] for i in range(self.rows))


def eye(n: int) -> Matrix:
    """Create identity matrix of size n x n."""
    data = [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
    return Matrix(data)


def zeros(rows: int, cols: int) -> Matrix:
    """Create zero matrix of size rows x cols."""
    data = [[0.0 for _ in range(cols)] for _ in range(rows)]
    return Matrix(data)


def ones(rows: int, cols: int) -> Matrix:
    """Create matrix of ones of size rows x cols."""
    data = [[1.0 for _ in range(cols)] for _ in range(rows)]
    return Matrix(data)
