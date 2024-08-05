use super::zz;
use num::bigint::BigInt;
use pyo3::prelude::*;

pub fn py_zz(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ZZ>()?;
    Ok(())
}

#[pyclass]
pub struct ZZ {
    v: zz::ZZ,
}

#[pymethods]
impl ZZ {
    #[new]
    fn new(num: BigInt) -> Self {
        ZZ { v: num.into() }
    }

    fn __add__(&self, rhs: &ZZ) -> Self {
        ZZ {
            v: &self.v + &rhs.v,
        }
    }
    fn __iadd__(&mut self, rhs: &ZZ) -> () {
        self.v += &rhs.v;
    }
    fn __sub__(&self, rhs: &ZZ) -> Self {
        ZZ {
            v: &self.v - &rhs.v,
        }
    }
    fn __isub__(&mut self, rhs: &ZZ) -> () {
        self.v -= &rhs.v;
    }
    fn __neg__(&self) -> Self {
        ZZ { v: -&self.v }
    }
    fn __abs__(&self) -> Self {
        ZZ {
            v: self.v.clone().abs(),
        }
    }
    fn __mul__(&self, rhs: &ZZ) -> Self {
        ZZ {
            v: &self.v * &rhs.v,
        }
    }
    fn __imul__(&mut self, rhs: &ZZ) -> () {
        self.v *= &rhs.v;
    }
    fn __truediv__(&self, rhs: &ZZ) -> Self {
        ZZ {
            v: &self.v / &rhs.v,
        }
    }
    fn __itruediv_(&mut self, rhs: &ZZ) -> () {
        self.v /= &rhs.v;
    }
    fn __mod__(&self, rhs: &ZZ) -> Self {
        ZZ {
            v: &self.v % &rhs.v,
        }
    }
    fn __imod__(&mut self, rhs: &ZZ) -> () {
        self.v %= &rhs.v;
    }

    fn __pow__(&self, exp: u32, modulo: Option<BigInt>) -> Self {
        ZZ { v: self.v.pow(exp) }
    }
    fn __ipow__(&mut self, exp: u32, modulo: Option<BigInt>) -> () {
        self.v = self.v.pow(exp);
    }

    fn __lt__(&self, rhs: &ZZ) -> bool {
        self.v < rhs.v
    }
    fn __le__(&self, rhs: &ZZ) -> bool {
        self.v <= rhs.v
    }
    fn __eq__(&self, rhs: &ZZ) -> bool {
        self.v == rhs.v
    }
    fn __ne__(&self, rhs: &ZZ) -> bool {
        self.v != rhs.v
    }
    fn __gt__(&self, rhs: &ZZ) -> bool {
        self.v > rhs.v
    }
    fn __ge__(&self, rhs: &ZZ) -> bool {
        self.v >= rhs.v
    }

    fn __and__(&self, rhs: &ZZ) -> ZZ {
        ZZ {
            v: &self.v & &rhs.v,
        }
    }
    fn __or__(&self, rhs: &ZZ) -> ZZ {
        ZZ {
            v: &self.v | &rhs.v,
        }
    }
    fn __xor__(&self, rhs: &ZZ) -> ZZ {
        ZZ {
            v: &self.v ^ &rhs.v,
        }
    }

    fn __lshift__(&self, rhs: u32) -> ZZ {
        ZZ { v: &self.v << rhs }
    }
    fn __rshift__(&self, rhs: u32) -> ZZ {
        ZZ { v: &self.v >> rhs }
    }

    fn __str__(&self) -> String {
        self.v.to_string()
    }
    fn __int__(&self) -> BigInt {
        self.v.clone().into()
    }
}
