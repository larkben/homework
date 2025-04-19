
/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

fn add_values(x: f32, y: f32) -> f32 {
    x + y
}

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let temp: Matrix = Vec::new();

    for columnOne in mat1 {
        for x in columnOne {
            println!("{}", x)
        }
    }

    temp
}