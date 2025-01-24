import numpy as np
from scipy.interpolate import KroghInterpolator

def main():
    # Example data points
    x = [-2, -1, 0, 0.5, 2, 3]
    y = [-5, 1, 1, 0.625, 7, 25]

    interpolator = KroghInterpolator(x, y)

    coefficients = interpolator.c
    print(f"Polynomial coefficients using Krogh: {coefficients}")

def ndd(x, y):
    """
    Computes the coefficients of the Newton's Divided Difference polynomial.
    
    Parameters:
        x (list or numpy array): x-coordinates of data points.
        y (list or numpy array): y-coordinates of data points.
        
    Returns:
        numpy array: Coefficients of the Newton's polynomial.
    """
    n = len(x)
    coefficients = np.array(y, dtype=float)
    
    for i in range(1, n):
        for j in range(n - 1, i - 1, -1):
            coefficients[j] = (coefficients[j] - coefficients[j - 1]) / (x[j] - x[j - i])
    
    return coefficients

def polynomial_degree(coefficients, tolerance=1e-6):
    """
    Determines the degree of the polynomial by identifying the last non-zero coefficient.
    
    Parameters:
        coefficients (numpy array): Coefficients of the polynomial.
        tolerance (float): Threshold below which a coefficient is considered zero.
        
    Returns:
        int: Degree of the polynomial.
    """
    for i in range(len(coefficients) - 1, -1, -1):
        if abs(coefficients[i]) > tolerance:
            return i
    return 0

if __name__ == '__main__':
    main()

