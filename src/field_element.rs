use anyhow::{anyhow, Result};
use num_bigint::BigInt;

use std::convert::{From, Into};
use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldElement {
    prime: BigInt,
    num: BigInt,
}

fn modulo<A, B>(a: A, b: B) -> BigInt
where
    A: Into<BigInt>,
    B: Into<BigInt>,
{
    let a: BigInt = a.into();
    let b: BigInt = b.into();
    (a % b.clone() + b.clone()) % b
}

impl FieldElement {
    pub fn new<N, P>(num: N, prime: P) -> FieldElement
    where
        N: Into<BigInt>,
        P: Into<BigInt>,
    {
        FieldElement {
            num: num.into(),
            prime: prime.into(),
        }
    }

    fn round<N: Into<BigInt>>(&self, num: N) -> FieldElement {
        let num = modulo(num, self.prime.clone());
        FieldElement::new(num, self.prime.clone())
    }

    fn pow<N: Into<BigInt>>(self, exponent: N) -> FieldElement {
        let n = modulo(exponent, self.prime.clone() - 1);
        let num = self
            .num
            .modpow(&BigInt::from(n), &BigInt::from(self.prime.clone()));
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn add(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        Ok(self.round(self.num.clone() + other.num))
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn sub(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot sub two numbers in deffirent Fields"));
        }
        Ok(self.round(self.num.clone() - other.num))
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn mul(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot mul two numbers in deffirent Fields"));
        }
        Ok(self.round(self.num.clone() * other.num))
    }
}

impl ops::Div<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn div(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot div two numbers in deffirent Fields"));
        }
        self.clone() * (other.pow(self.prime - BigInt::from(2)))
    }
}

#[test]
fn test_add() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(6, 13);
    assert_eq!((a + b).unwrap(), c);
}

#[test]
fn test_sub() {
    let a = FieldElement::new(6, 19);
    let b = FieldElement::new(13, 19);
    let c = FieldElement::new(12, 19);
    assert_eq!((a - b).unwrap(), c);
}

#[test]
fn test_mul() {
    let a = FieldElement::new(6, 19);
    let b = FieldElement::new(13, 19);
    let c = FieldElement::new(2, 19);
    assert_eq!((a * b).unwrap(), c);
}

#[test]
fn test_pow() {
    let a = FieldElement::new(9, 19);
    let b = FieldElement::new(7, 19);
    assert_eq!(FieldElement::pow(a, 12), b);

    let a = FieldElement::new(9, 19);
    let b = FieldElement::new(1, 19);
    assert_eq!(FieldElement::pow(a, 18), b);
}

#[test]
fn test_neg_pow() {
    let a = FieldElement::new(7, 13);
    assert_eq!(FieldElement::pow(a.clone(), -3), FieldElement::pow(a, 9));
}

#[test]
fn test_div() {
    let a = FieldElement::new(2, 19);
    let b = FieldElement::new(7, 19);
    let c = FieldElement::new(3, 19);
    assert_eq!((a / b).unwrap(), c);
}
