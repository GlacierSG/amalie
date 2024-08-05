use crate::unit::zz::ZZ;

trait Dot {
    fn dot(&self, other: &Vec<ZZ>) -> Result<ZZ, ()>;
}

impl Dot for Vec<ZZ> {
    fn dot(&self, other: &Vec<ZZ>) -> Result<ZZ, ()> {
        if self.len() != other.len() {
            return Err(());
        }

        Ok(self.iter().zip(other.iter()).map(|(x, y)| x * y).sum())
    }
}

