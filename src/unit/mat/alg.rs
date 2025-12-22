use super::Matrix;
use crate::unit::zz::ZZ;
use std::cmp::min;

fn sqrt_floor(value: u128) -> u128 {
    if value < 2 { return value }

    let (mut lo, mut hi) = (0, min(value, 18446744073709551615)); // sqrt(u128::max)
    let mut out = 0;
    while lo <= hi {
        let m = (hi + lo) / 2;
        let v = m.pow(2);
        if v <= value {
            out = m;
            lo = m + 1;
        }
        else {
            hi = m - 1;
        }
    }
    out
}

fn determinant(mat: &Matrix, index: Vec<usize>) -> ZZ {
    let get = |i: usize| mat.get(i / mat.cols, i % mat.cols);
    match index.len() {
        0 => return ZZ::zero(),
        1 => return get(index[0]).clone(),
        4 => return get(index[0]) * get(index[3]) - get(index[1]) * get(index[2]),
        _ => {}
    }

    let s = sqrt_floor(index.len() as u128) as usize;
    let mut out = ZZ::zero();
    for j in 0..s {
        let mut sub = Vec::with_capacity((s - 1) * (s - 1));
        for i1 in 1..s {
            for j1 in 0..s {
                if j1 != j { 
                    sub.push(index[i1*s + j1]);
                }
            }
        }
        out += if j % 2 == 0 { 1 } else { -1 } * get(index[j]) * determinant(mat, sub)
    }
    out
}


impl Matrix {
    pub fn transpose(&self) -> Matrix {
        if self.rows == 1 && self.cols == 1 || self.cols == 0 { return self.clone(); }

        let mut mat = Matrix::zeros(self.cols, self.rows);
        for t in 0..self.rows*self.cols {
            let (i, j) = (t%self.cols, t/self.cols);
            mat.set(i, j, self.get(j, i).clone());
        }
        mat
    }

    pub fn det(&self) -> ZZ {
        assert!(self.rows == self.cols && self.rows != 0);
        return determinant(self, (0..(self.cols*self.rows)).collect());
    }

    pub fn swap_rows(&mut self, i: usize, j: usize) {
        assert!(i < self.rows && j < self.rows);
        for x in 0..self.cols {
            self.swap(i, x, j, x);
        }
    }
    pub fn swap_cols(&mut self, i: usize, j: usize) {
        assert!(i < self.cols && j < self.cols);
        for x in 0..self.rows {
            self.swap(x, i, x, j);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{zz, matrix, ZZ};
    use super::Matrix;

    #[test]
    fn transpose() {
        let mat: Matrix = matrix![[1]];
        let mat_t = mat.transpose();
        assert_eq!(mat, mat_t);

        let mat = matrix![[1, 2],[3, 4]];
        let mat_t = mat.transpose();
        let mat_a = matrix![[1, 3],[2, 4]];
        assert_eq!(mat_t, mat_a);
        assert_eq!(mat_t.transpose(), mat);

        let mat = matrix![[1, 3, 5],[2, 4, 6]];
        let mat_t = mat.transpose();
        assert_eq!(mat_t, matrix![[1, 2],[3, 4], [5,6]]);
    }

    #[test]
    fn det() {
        let mat = matrix![[1]]; 
        assert_eq!(mat.det(), 1);

        let mat = matrix![[123213, 58890912], [92349089, 90980932549089909890832908]];
        assert_eq!(mat.det(), zz!(11210033642171009628857121514236));
        
        let mat = matrix![[845987438574387, 89798798798437, 98758932847983274], [31321321312, 21, 938479827433213], [1,2932847982374982379847329847983274,0]];
        assert_eq!(mat.det(), zz!(-2319439557016705698734121838804798042159530631553151761401634055));

        let mat = matrix![[5,10,100,100],[5,6,7,8],[123,512,142,123],[132135,14,123,125]];
        assert_eq!(mat.det(), zz!(-7907536850));

        let mat = matrix![[1237,79,78,987,-69742698],[987,987,-98798,7969,6921],[69698,69,8698,7,876],[487,-96,7987,9798,7987],[98,7987,987,987,-987]];
        assert_eq!(mat.det(), zz!(-40139353700095230482784330));
    }
}
