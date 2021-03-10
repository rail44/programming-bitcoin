use anyhow::{Result, anyhow};
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

use std::ops;

#[derive(Debug, PartialEq, Clone)]
struct FieldElement {
    prime: i64,
    num: BigInt,
}

fn modulo(a: i64, b: i64) -> i64 {
    (a % b + b) % b
}

impl FieldElement {
    fn new(num: i64, prime: i64) -> FieldElement {
        FieldElement { num: BigInt::from(num), prime }
    }

    fn round(&self, num: BigInt) -> FieldElement {
        let num = modulo(num.to_i64().unwrap(), self.prime);
        FieldElement::new(num, self.prime)
    }

    fn pow(self, exponent: i64) -> FieldElement {
        let n = modulo(exponent, self.prime - 1);
        let num = self.num.modpow(&BigInt::from(n), &BigInt::from(self.prime));
        FieldElement { num, prime: self.prime }
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
        let prime = self.prime as i64;
        self * (other.pow(prime - 2))
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

fn main() {
    println!("Hello, world!");
}
