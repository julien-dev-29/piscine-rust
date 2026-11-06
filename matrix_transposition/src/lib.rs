#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let matrix: Matrix = Matrix((m.0.0, m.1.0), (m.0.1, m.1.1));
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix: Matrix = Matrix((1, 3), (4, 5));
        let result: Matrix = transpose(matrix);
        assert_eq!(result, Matrix((1, 4), (3, 5)));
    }
}
