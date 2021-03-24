use anyhow::{anyhow, Result};
use num::traits::{Pow, Zero};
use num_bigint::BigInt;

use std::convert::{From, Into};
use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldElement {
    prime: BigInt,
    num: BigInt,
}

fn modulo(a: &'_ BigInt, b: &'_ BigInt) -> BigInt {
    (a % b + b) % b
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

    fn round(&self, num: &'_ BigInt) -> FieldElement {
        let num = modulo(&num, &self.prime);
        FieldElement::new(num, self.prime.clone())
    }

    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }
}

impl<'a, 'b> ops::Add<&'b FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement>;

    fn add(self, other: &'b FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        Ok(self.round(&(&self.num + &other.num)))
    }
}

impl<'a, 'b> ops::Sub<&'b FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement>;

    fn sub(self, other: &'b FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot sub two numbers in deffirent Fields"));
        }
        Ok(self.round(&(&self.num - &other.num)))
    }
}

impl<'a, 'b> ops::Mul<&'b FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement>;

    fn mul(self, other: &'b FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot mul two numbers in deffirent Fields"));
        }
        Ok(self.round(&(&self.num * &other.num)))
    }
}

impl<'a, 'b> ops::Mul<&'b BigInt> for &'a FieldElement {
    type Output = Result<FieldElement>;

    fn mul(self, other: &'b BigInt) -> Result<FieldElement> {
        Ok(self.round(&(&self.num * other)))
    }
}

impl<'a, 'b> ops::Div<&'b FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement>;

    fn div(self, other: &'b FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot div two numbers in deffirent Fields"));
        }
        self * &other.pow(&(&self.prime - BigInt::from(2)))
    }
}

impl<'a, 'b> Pow<&'b BigInt> for &'a FieldElement {
    type Output = FieldElement;

    fn pow(self, exponent: &'b BigInt) -> FieldElement {
        let n = modulo(exponent, &(&self.prime - BigInt::from(1)));
        let num = self.num.modpow(&n, &self.prime);
        FieldElement {
            num,
            prime: self.prime.clone(),
        }
    }
}

#[test]
fn test_add() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(6, 13);
    assert_eq!((&a + &b).unwrap(), c);
}

#[test]
fn test_sub() {
    let a = FieldElement::new(6, 19);
    let b = FieldElement::new(13, 19);
    let c = FieldElement::new(12, 19);
    assert_eq!((&a - &b).unwrap(), c);
}

#[test]
fn test_mul() {
    let a = FieldElement::new(6, 19);
    let b = FieldElement::new(13, 19);
    let c = FieldElement::new(2, 19);
    assert_eq!((&a * &b).unwrap(), c);
}

#[test]
fn test_pow() {
    let a = FieldElement::new(9, 19);
    let b = FieldElement::new(7, 19);
    assert_eq!(a.pow(&BigInt::from(12)), b);

    let a = FieldElement::new(9, 19);
    let b = FieldElement::new(1, 19);
    assert_eq!(a.pow(&BigInt::from(18)), b);
}

#[test]
fn test_neg_pow() {
    let a = FieldElement::new(7, 13);
    assert_eq!(a.pow(&BigInt::from(-3)), a.pow(&BigInt::from(9)));
}

#[test]
fn test_div() {
    let a = FieldElement::new(2, 19);
    let b = FieldElement::new(7, 19);
    let c = FieldElement::new(3, 19);
    assert_eq!((&a / &b).unwrap(), c);
}
