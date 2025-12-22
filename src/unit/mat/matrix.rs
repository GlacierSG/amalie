use crate::unit::zz::ZZ;

#[derive(Clone, Debug)]
pub struct Matrix {
    v: Vec<ZZ>,
    pub(super) rows: usize,
    pub(super) cols: usize,
}

impl Matrix {
    pub fn new(values: Vec<ZZ>, rows: usize, cols: usize) -> Matrix {
        Matrix {
            v: values,
            rows,
            cols,
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        let values = vec![ZZ::from(0); rows * cols];
        Matrix {
            v: values,
            rows,
            cols,
        }
    }

    pub fn empty() -> Matrix {
        let values = vec![];
        Matrix {
            v: values,
            rows: 0,
            cols: 0,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn set(&mut self, i: usize, j: usize, val: ZZ) {
        assert!(j < self.cols && i < self.rows);
        self.v[i + j * self.rows] = val;
    }
    pub fn get(&self, i: usize, j: usize) -> &ZZ {
        assert!(j < self.cols && i < self.rows);
        // ith row jth column
        &self.v[i + j * self.rows]
    }
    pub fn swap(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) {
        assert!(j1 < self.cols && i1 < self.rows);
        assert!(j2 < self.cols && i2 < self.rows);
        self.v.swap(i1 + j1 * self.rows, i2 + j2 * self.rows);
    }
}
