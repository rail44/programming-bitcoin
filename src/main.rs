use anyhow::{Result, anyhow};
use std::ops;

#[derive(Debug, PartialEq)]
struct FieldElement {
    prime: i32,
    num: i32,
}

impl FieldElement {
    fn new(num: i32, prime: i32) -> FieldElement {
        FieldElement { prime, num }
    }

    fn round(&self, num: i32) -> FieldElement {
        let num = (num % self.prime + self.prime) % self.prime;
        FieldElement { prime: self.prime, num }
    }

    fn pow(self, exponent: i32) -> FieldElement {
        let n = self.round(exponent % (self.prime - 1));
        let num = self.num.pow(n.num as u32);
        self.round(num)
    }
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn add(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        Ok(self.round(self.num + other.num))
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn sub(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        Ok(self.round(self.num - other.num))
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn mul(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        Ok(self.round(self.num * other.num))
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
fn test_neg_pow() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(8, 13);
    assert_eq!(FieldElement::pow(a, -3), b);
}

fn main() {
    println!("Hello, world!");
}
