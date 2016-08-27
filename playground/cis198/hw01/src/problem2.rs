/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // TODO
    let mat1_cols = mat1[0].len();
    let mat2_cols = mat2[0].len();
    let mat1_rows = mat1.len();
    let mat2_rows = mat2.len();
    assert!(mat2_rows == mat1_cols);
    let mut result:Matrix = vec![vec![0.;mat2_cols]; mat1_rows];
    for row in 0..mat1_rows {
        for col in 0..mat2_cols {
            for runner in 0..mat1_cols {
                result[row][col] += mat1[row][runner] * mat2[runner][col];
            }
        }
    }
    result
}
