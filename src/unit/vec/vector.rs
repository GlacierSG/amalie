use crate::unit::zz::ZZ;

struct Vector {
    v: Vec<ZZ>
}


impl Vector {
    pub fn new() -> Vector {
        Vector { v: vec![] }
    }

    pub fn dot(&self, rhs: Vector) -> ZZ {
        self.v.iter().zip(rhs.v.iter()).map(|(x,y)| x*y).sum()
    }
}
