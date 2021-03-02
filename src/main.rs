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
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn add(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        let num = (self.num + other.num) % self.prime;
        Ok(FieldElement { prime: self.prime, num })
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = Result<FieldElement>;

    fn sub(self, other: FieldElement) -> Result<FieldElement> {
        if self.prime != other.prime {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        let num = ((self.num - other.num) % self.prime) + self.prime;
        Ok(FieldElement { prime: self.prime, num })
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

fn main() {
    println!("Hello, world!");
}
