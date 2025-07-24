use crate::{alg::gcd, alg::is_prime, zz, Error, Result, ZZ};

fn g(mut x: ZZ, n: &ZZ) -> ZZ {
    x <<= 1;
    x += 1;
    x % n
}

fn pollard_rho(n: impl AsRef<ZZ>) -> Result<ZZ> {
    let n = n.as_ref();
    if n < 2 {
        return Err(Error::InvalidInput("n < 2".to_string()));
    }

    let mut seed = zz!(2);
    loop {
        let mut x = seed.clone();
        let mut y = x.clone();
        let mut d = zz!(1);
        while &d == 1 {
            x = g(x, n);
            y = g(g(y, n), n);
            d = gcd((&x - &y).abs(), n)
        }
        if &d != n {
            return Ok(d);
        }
        seed += 1;
    }
}

pub fn factor(n: ZZ) -> Result<Vec<ZZ>> {
    let mut factors = Vec::new();
    let mut composite = n.clone();

    if composite < 0 {
        composite *= zz!(-1);
        factors.push(zz!(-1));
    }

    if composite < zz!(2) {
        return Err(Error::InvalidInput("n < 2".to_string()));
    }

    // Pollard rho may fail to factor even numbers
    while composite.clone() & 1 == 0 {
        factors.push(zz!(2));
        composite >>= 1;
    }

    if composite == zz!(1) {
        return Ok(factors);
    }

    if is_prime(composite.clone()) {
        factors.push(composite);
        return Ok(factors);
    }

    println!("factoring {:?}", composite.clone());

    let mut divisor_queue = vec![composite];
    while let Some(mut val) = divisor_queue.pop() {
        // TODO: Pollard Rho can fail when factoring perfect powers.
        // Since factoring perfect powers only takes O((lg^3 n) lg lg lg n) iirc, while Pollard Rho
        // takes O(n^(1/4)), make sure to factor perfect powers first.
        let d = pollard_rho(&val)?;
        val /= d.clone();
        if is_prime(val.clone()) {
            factors.push(val);
        } else {
            divisor_queue.push(val);
        }
        if is_prime(d.clone()) {
            factors.push(d);
        } else {
            divisor_queue.push(d);
        }
    }

    factors.sort_unstable();
    Ok(factors)
}

#[cfg(test)]
mod test {
    use super::pollard_rho;
    use crate::{alg::factor, zz, ZZ};

    #[test]
    fn test_pollard_rho() {
        assert_eq!(pollard_rho(zz!(123)).unwrap(), 3);

        assert!(pollard_rho(zz!(1)).is_err());

        assert_eq!(pollard_rho(zz!(6131066257801)).unwrap(), 19);

        assert_eq!(pollard_rho(zz!(131101) * zz!(2097169)).unwrap(), 131101);
    }

    #[test]
    fn test_factor() {
        assert_eq!(factor(zz!(2)).unwrap(), vec![2]);
        assert_eq!(factor(zz!(3)).unwrap(), vec![3]);
        assert_eq!(factor(zz!(2 * 3)).unwrap(), vec![2, 3]);
        assert_eq!(factor(zz!(3 * 3 * 5)).unwrap(), vec![3, 3, 5]);
        assert_eq!(factor(zz!(-1 * 3 * 3 * 5)).unwrap(), vec![-1, 3, 3, 5]);
        assert_eq!(factor(zz!(123)).unwrap(), vec![3, 41]);
        assert_eq!(factor(zz!(6131066257801)).unwrap(), vec![19; 10]);
        assert_eq!(
            factor(zz!(11 * 13 * 17 * 19)).unwrap(),
            vec![11, 13, 17, 19]
        );
        assert_eq!(
            factor(zz!(131101) * zz!(2097169)).unwrap(),
            vec![131101, 2097169]
        );
    }
}
