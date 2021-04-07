use anyhow::{anyhow, Result};
use num::traits::Pow;
use num_bigint::BigInt;
use std::ops;

use crate::field_element::FieldElement;

#[derive(Debug, PartialEq, Clone)]
pub struct ActualPoint<'a> {
    pub x: FieldElement<'a>,
    pub y: FieldElement<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Point<'a> {
    Inf,
    Actual(ActualPoint<'a>),
}

impl<'a> Point<'a> {
    fn new(x: FieldElement<'a>, y: FieldElement<'a>) -> Point<'a> {
        Point::Actual(ActualPoint { x, y })
    }

    pub fn as_actual(&self) -> &ActualPoint<'a> {
        if let Point::Actual(p) = self {
            return p;
        }
        panic!("not normal point");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Curve<'a> {
    a: FieldElement<'a>,
    b: FieldElement<'a>,
}

impl<'a> Curve<'a> {
    pub fn new(a: FieldElement<'a>, b: FieldElement<'a>) -> Curve<'a> {
        Curve { a, b }
    }

    pub fn point(&'a self, x: FieldElement<'a>, y: FieldElement<'a>) -> Result<CurvePoint<'a>> {
        if y.pow(&2.into()) != (&(&x.pow(&3.into()) + &(&self.a * &x)?)? + &self.b)? {
            return Err(anyhow!("({:?}, {:?}) is not on the curve", x, y));
        }
        Ok(CurvePoint {
            c: self,
            p: Point::new(x, y),
        })
    }

    pub fn inf(&'_ self) -> CurvePoint<'_> {
        CurvePoint {
            c: self,
            p: Point::Inf,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CurvePoint<'a> {
    c: &'a Curve<'a>,
    pub p: Point<'a>,
}

impl<'a, 'b, 'c> ops::Add<&'c CurvePoint<'a>> for &'b CurvePoint<'a> {
    type Output = Result<CurvePoint<'a>>;

    fn add(self, other: &'c CurvePoint<'a>) -> Result<CurvePoint<'a>> {
        if self.c != other.c {
            return Err(anyhow!(
                "Points {:?}, {:?} are not on the same curve",
                self,
                other
            ));
        }

        if self.p == Point::Inf {
            return Ok(other.clone());
        }

        if other.p == Point::Inf {
            return Ok(self.clone());
        }

        let self_p = self.p.as_actual();
        let other_p = other.p.as_actual();

        if self_p.x != other_p.x {
            let s = (&(&other_p.y - &self_p.y)? / &(&other_p.x - &self_p.x)?)?;
            let x = (&(&s.pow(&2.into()) - &self_p.x)? - &other_p.x)?;
            let y = (&(&s * &(&self_p.x - &x)?)? - &self_p.y)?;
            return self.c.point(x, y);
        }

        if self_p.y != other_p.y {
            return Ok(self.c.inf());
        }

        if self_p.y.is_zero() {
            return Ok(self.c.inf());
        }

        let s = (&(&(&self_p.x.pow(&2.into()) * &BigInt::from(3))? + &self.c.a)?
            / &(&self_p.y * &BigInt::from(2))?)?;
        let x = (&s.pow(&2.into()) - &(&self_p.x * &BigInt::from(2))?)?;
        let y = (&(&s * &(&self_p.x - &x)?)? - &self_p.y)?;
        self.c.point(x, y)
    }
}

// #[test]
// fn test_new() {
//     let c = Curve::<BigInt>::new(5.into(), 7.into());
//     let p1 = c.point((-1).into(), (-1).into());
//     assert!(p1.is_ok());
//
//     let p2 = c.point((-1).into(), (-2).into());
//     assert!(p2.is_err());
// }
//
// #[test]
// fn test_add_1() {
//     let c = Curve::<BigInt>::new(5.into(), 7.into());
//     let p1 = c.point(2.into(), 5.into()).unwrap();
//     let p2 = c.point((-1).into(), (-1).into()).unwrap();
//     assert_eq!((p1 + p2).unwrap(), c.point(3.into(), (-7).into()).unwrap());
// }
//
// #[test]
// fn test_add_2() {
//     let c = Curve::<BigInt>::new(5.into(), 7.into());
//     let p1 = c.point((-1).into(), (-1).into()).unwrap();
//     assert_eq!(
//         (p1.clone() + p1).unwrap(),
//         c.point(18.into(), 77.into()).unwrap()
//     );
// }
