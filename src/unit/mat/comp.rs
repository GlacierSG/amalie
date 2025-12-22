use super::Matrix;

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
         if self.rows == rhs.rows && self.cols == rhs.cols {
            let mut same = true;
            for i in 0..self.rows {
                for j in 0..self.cols {
                    if self.get(i, j) != rhs.get(i, j) {
                        same = false;
                    }
                }
            }
            return same;
        }
        false
    }
}
impl Eq for Matrix {}
