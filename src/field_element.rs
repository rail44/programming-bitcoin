use anyhow::{anyhow, Result};
use num::traits::{Pow, Zero};
use num_bigint::BigInt;

use std::convert::{From, Into};
use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct Prime(BigInt);

impl Prime {
    pub fn new<N>(num: N) -> Self
    where
        N: Into<BigInt>,
    {
        Prime(num.into())
    }

    pub fn field_element<N>(&self, num: N) -> FieldElement<'_>
    where
        N: Into<BigInt>,
    {
        FieldElement {
            num: num.into(),
            prime: self,
        }
    }
}

impl AsRef<BigInt> for Prime {
    fn as_ref(&self) -> &BigInt {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldElement<'a> {
    prime: &'a Prime,
    num: BigInt,
}

fn modulo(a: &'_ BigInt, b: &'_ BigInt) -> BigInt {
    (a % b + b) % b
}

impl<'a> FieldElement<'a> {
    fn round(&self, num: &'_ BigInt) -> FieldElement<'a> {
        let num = modulo(&num, self.prime.as_ref());
        self.prime.field_element(num)
    }

    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }
}

impl<'a, 'b, 'c> ops::Add<&'b FieldElement<'c>> for &'a FieldElement<'c> {
    type Output = Result<FieldElement<'c>>;

    fn add(self, other: &'b FieldElement) -> Result<FieldElement<'c>> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        Ok(self.round(&(&self.num + &other.num)))
    }
}

impl<'a, 'b, 'c> ops::Sub<&'b FieldElement<'c>> for &'a FieldElement<'c> {
    type Output = Result<FieldElement<'c>>;

    fn sub(self, other: &'b FieldElement) -> Result<FieldElement<'c>> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot sub two numbers in deffirent Fields"));
        }
        Ok(self.round(&(&self.num - &other.num)))
    }
}

impl<'a, 'b, 'c> ops::Mul<&'b FieldElement<'c>> for &'a FieldElement<'c> {
    type Output = Result<FieldElement<'c>>;

    fn mul(self, other: &'b FieldElement) -> Result<FieldElement<'c>> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot mul two numbers in deffirent Fields"));
        }
        Ok(self.round(&(&self.num * &other.num)))
    }
}

impl<'a, 'b, 'c> ops::Mul<&'b BigInt> for &'a FieldElement<'c> {
    type Output = Result<FieldElement<'c>>;

    fn mul(self, other: &'b BigInt) -> Result<FieldElement<'c>> {
        Ok(self.round(&(&self.num * other)))
    }
}

impl<'a, 'b, 'c> ops::Div<&'b FieldElement<'c>> for &'a FieldElement<'c> {
    type Output = Result<FieldElement<'c>>;

    fn div(self, other: &'b FieldElement<'c>) -> Result<FieldElement<'c>> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot div two numbers in deffirent Fields"));
        }
        self * &other.pow(&(self.prime.as_ref() - BigInt::from(2)))
    }
}

impl<'a, 'b, 'c> Pow<&'b BigInt> for &'a FieldElement<'c> {
    type Output = FieldElement<'c>;

    fn pow(self, exponent: &'b BigInt) -> FieldElement<'c> {
        let n = modulo(exponent, &(self.prime.as_ref() - BigInt::from(1)));
        let num = self.num.modpow(&n, self.prime.as_ref());
        self.prime.field_element(num)
    }
}

#[test]
fn test_add() {
    let prime = Prime::new(13);
    let a = prime.field_element(7);
    let b = prime.field_element(12);
    let c = prime.field_element(6);
    assert_eq!((&a + &b).unwrap(), c);
}

#[test]
fn test_sub() {
    let prime = Prime::new(19);
    let a = prime.field_element(6);
    let b = prime.field_element(13);
    let c = prime.field_element(12);
    assert_eq!((&a - &b).unwrap(), c);
}

#[test]
fn test_mul() {
    let prime = Prime::new(19);
    let a = prime.field_element(6);
    let b = prime.field_element(13);
    let c = prime.field_element(2);
    assert_eq!((&a * &b).unwrap(), c);
}

#[test]
fn test_pow() {
    let prime = Prime::new(19);
    let a = prime.field_element(9);
    let b = prime.field_element(7);
    assert_eq!(a.pow(&BigInt::from(12)), b);

    let a = prime.field_element(9);
    let b = prime.field_element(1);
    assert_eq!(a.pow(&BigInt::from(18)), b);
}

#[test]
fn test_neg_pow() {
    let prime = Prime::new(13);
    let a = prime.field_element(7);
    assert_eq!(a.pow(&BigInt::from(-3)), a.pow(&BigInt::from(9)));
}

#[test]
fn test_div() {
    let prime = Prime::new(19);
    let a = prime.field_element(2);
    let b = prime.field_element(7);
    let c = prime.field_element(3);
    assert_eq!((&a / &b).unwrap(), c);
}
