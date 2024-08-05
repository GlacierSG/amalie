use super::ZZ;
use num::bigint::RandBigInt;
use num::Integer;
use crate::mod_inv;
use crate::{Result, Error};
use rand::thread_rng;

impl ZZ {
    /// lower bound (a) is inclusive
    /// upper bound (b) is exclusive
    pub fn rand_range(a: impl AsRef<ZZ>, b: impl AsRef<ZZ>) -> ZZ {
        let a = a.as_ref();
        let b = b.as_ref();

        let mut rng = thread_rng();
        ZZ {
            v: rng.gen_bigint_range(&a.v, &b.v),
        }
    }

    pub fn sqrt(&self) -> ZZ {
        ZZ { v: self.v.sqrt() }
    }
    pub fn gcd(&self, other: &Self) -> ZZ {
        ZZ {
            v: self.v.gcd(&other.v),
        }
    }

    pub fn egcd(&self, other: &Self) -> (ZZ, ZZ, ZZ) {
        let t = self.v.extended_gcd(&other.v);
        (ZZ { v: t.gcd }, ZZ { v: t.x }, ZZ { v: t.y })
    }

    pub fn lcm(&self, other: &Self) -> ZZ {
        ZZ {
            v: self.v.lcm(&other.v),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        ZZ { v: self.v.pow(exp) }
    }

    pub fn mod_pow(&self, exp: impl AsRef<ZZ>, m: impl AsRef<ZZ>) -> Result<Self> {
        let exp = exp.as_ref();
        let m = m.as_ref();

        if m == 0 {
            return Err(Error::InvalidInput("modulus is 0".to_string()));
        }
        if exp < 0 {
            let v = mod_inv(&self, m)?;
            Ok(ZZ {
                v: (v.mod_pow(-exp, &m)?).v,
            })
        }
        else {
            Ok(ZZ {
                v: self.v.modpow(&exp.v, &m.v),
            })
        }
    }

    pub fn nth_root(&self, n: impl AsRef<ZZ>) -> Result<ZZ> {
        let n = n.as_ref();
        match (&n.v).try_into() {
            Ok(nth) => {
                let r = self.v.nth_root(nth);
                if r.pow(nth) == n.v {
                    return Ok(ZZ{ v: r })
                }
                else {
                    return Err(Error::NoResult);
                }
            }
            _ => {
                todo!("the root was too high");
            }
        }
    }


    pub fn root_floor(&self, n: impl AsRef<ZZ>) -> Result<ZZ> {
        let n= n.as_ref();
        match (&n.v).try_into() {
            Ok(nth) => {
                Ok(ZZ{v: self.v.nth_root(nth)})
            }
            _ => {
                todo!("the root was too high");
            }
        }
    }
    pub fn root_ceil(&self, nth: impl AsRef<ZZ>) -> Result<ZZ> {
        let nth = nth.as_ref();
        match (&nth.v).try_into() {
            Ok(nth) => {
                let r = self.v.nth_root(nth);
                if r.pow(nth) < self.v {
                    return Ok(ZZ{v: r+1});
                }
                else {
                    return Ok(ZZ{v: r});
                }
            }
            _ => {
                todo!("the root was too high");
            }
        }
    }

    pub fn is_even(&self) -> bool {
        self.v.is_even()
    }
    pub fn is_prime(&self) -> bool {
        crate::alg::is_prime(&self)
    }
    pub fn is_square(&self) -> bool {
        let v = self.v.nth_root(2);
        v.pow(2) == self.v
    }
}
