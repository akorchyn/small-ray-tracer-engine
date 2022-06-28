use std::ops::{Index, IndexMut, Mul};

use super::normal::Normal;
use super::point::Point;
use super::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Matrix<const ROW: usize, const COLUMN: usize> {
    data: [[f64; COLUMN]; ROW],
}

impl<const ROW: usize, const COLUMN: usize> Matrix<ROW, COLUMN> {
    pub(crate) fn new() -> Self {
        Matrix {
            data: [[0.0; COLUMN]; ROW],
        }
    }

    pub(crate) fn with_data(data: [[f64; COLUMN]; ROW]) -> Self {
        Matrix { data }
    }
}

impl Matrix<4, 4> {
    pub(crate) fn rotation_x(radians: f64) -> Matrix<4, 4> {
        let c = radians.cos();
        let s = radians.sin();
        Self::with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, -s, 0.0],
            [0.0, s, c, 0.0],
            [0.0, 0.0, 0.0, 0.1],
        ])
    }

    pub(crate) fn rotation_y(radians: f64) -> Matrix<4, 4> {
        let c = radians.cos();
        let s = radians.sin();
        Self::with_data([
            [c, 0.0, s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [0.0, 0.0, 0.0, 0.1],
        ])
    }

    pub(crate) fn rotation_z(radians: f64) -> Matrix<4, 4> {
        let c = radians.cos();
        let s = radians.sin();
        Self::with_data([
            [c, -s, 0.0, 0.0],
            [s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub(crate) fn scale(vector: Vector) -> Matrix<4, 4> {
        Self::with_data([
            [vector.x, 0.0, 0.0, 0.0],
            [0.0, vector.y, 0.0, 0.0],
            [0.0, 0.0, vector.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub(crate) fn translation(vector: Vector) -> Matrix<4, 4> {
        Self::with_data([
            [1.0, 0.0, 0.0, vector.x],
            [0.0, 1.0, 0.0, vector.y],
            [0.0, 0.0, 1.0, vector.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl<const ROW: usize, const COLUMN: usize> Index<usize> for Matrix<ROW, COLUMN> {
    type Output = [f64];

    fn index(&self, row: usize) -> &[f64] {
        self.data[row].as_slice()
    }
}

impl<const ROW: usize, const COLUMN: usize> IndexMut<usize> for Matrix<ROW, COLUMN> {
    fn index_mut(&mut self, row: usize) -> &mut [f64] {
        self.data[row].as_mut_slice()
    }
}

impl<const ROW1: usize, const COLUMN1: usize, const ROW2: usize, const COLUMN2: usize>
    Mul<Matrix<ROW2, COLUMN2>> for Matrix<ROW1, COLUMN1>
{
    type Output = Matrix<ROW1, COLUMN2>;
    fn mul(self, rhs: Matrix<ROW2, COLUMN2>) -> Self::Output {
        let mut data = [[0.0; COLUMN2]; ROW1];
        for i in 0..ROW1 {
            for j in 0..COLUMN2 {
                let mut sum = 0.0;
                for k in 0..ROW2 {
                    sum += self[i][k] * rhs[k][j];
                }
                data[i][j] = sum;
            }
        }
        Matrix { data }
    }
}

impl<const COLUMN: usize> Mul<Vector> for Matrix<4, COLUMN> {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        let result = self.mul(from_vector(rhs));
        Vector::new(result[0][0], result[1][0], result[2][0])
    }
}

impl Mul<Point> for Matrix<4, 4> {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        let result = self.mul(from_point(rhs));
        Point::new(result[0][0], result[1][0], result[2][0])
    }
}

impl<const COLUMN: usize> Mul<Normal> for Matrix<4, COLUMN> {
    type Output = Normal;
    fn mul(self, rhs: Normal) -> Self::Output {
        self.mul(Vector::from(rhs)).normalize()
    }
}

pub(crate) fn from_point(point: Point) -> Matrix<4, 1> {
    Matrix {
        data: [[point.x], [point.y], [point.z], [1.0]],
    }
}

pub(crate) fn from_vector(vector: Vector) -> Matrix<4, 1> {
    Matrix {
        data: [[vector.x], [vector.y], [vector.z], [1.0]],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_mul() {
        let m1 =
            Matrix::<3, 4>::with_data([[1.0, 2., 3., 4.], [5., 6., 7., 8.], [9., 10., 11., 12.]]);
        let m2 = Matrix::<4, 3>::with_data([
            [13.0, 14., 15.],
            [16., 17., 18.],
            [19., 20., 21.],
            [22., 23., 24.],
        ]);
        let m3 = m1 * m2;
        assert_eq!(
            m3,
            Matrix::<3, 3>::with_data([[190., 200., 210.], [470., 496., 522.], [750., 792., 834.]])
        );
    }

    #[test]
    fn test_translation() {
        let t = Matrix::translation(Vector::new(1.0, 2.0, 3.0));
        let matrix = Matrix::<4, 4>::with_data([
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 2.0],
            [0.0, 0.0, 1.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(t, matrix);
        let p = Point::new(4.0, 5.0, 6.0);
        let expected = Point::new(5.0, 7.0, 9.0);
        assert_eq!(t * p, expected);
    }
}
