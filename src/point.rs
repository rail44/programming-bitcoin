use anyhow::{anyhow, Result};
use num::traits::Pow;
use num_bigint::BigInt;
use std::fmt::Debug;
use std::ops;

use crate::field_element::FieldElement;

#[derive(Debug, PartialEq, Clone)]
pub struct ActualPoint<F> {
    pub x: F,
    pub y: F,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Point<F> {
    Inf,
    Actual(ActualPoint<F>),
}

impl<'a, F> Point<F> {
    fn new(x: F, y: F) -> Point<F> {
        Point::Actual(ActualPoint { x, y })
    }

    pub fn as_actual(&self) -> &ActualPoint<F> {
        if let Point::Actual(p) = self {
            return p;
        }
        panic!("not normal point");
    }

    pub fn into_actual(self) -> ActualPoint<F> {
        if let Point::Actual(p) = self {
            return p;
        }
        panic!("not normal point");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Curve<F> {
    a: F,
    b: F,
}

impl<'a, F> Curve<F>
where
    F: AsRef<FieldElement<'a>> + From<FieldElement<'a>> + Debug,
{
    pub fn new(a: F, b: F) -> Curve<F> {
        Curve { a, b }
    }

    pub fn point_from_field_element(&self, x: F, y: F) -> Result<CurvePoint<F>> {
        if y.as_ref().pow(&2.into())
            != (&(&x.as_ref().pow(&3.into()) + &(self.a.as_ref() * x.as_ref())?)?
                + self.b.as_ref())?
        {
            return Err(anyhow!("({:?}, {:?}) is not on the curve", x, y));
        }
        Ok(CurvePoint {
            c: self,
            p: Point::new(x, y),
        })
    }

    pub fn point<A, B>(&self, x: A, y: B) -> Result<CurvePoint<F>>
    where
        A: Into<BigInt>,
        B: Into<BigInt>,
    {
        self.point_from_field_element(
            self.a.as_ref().prime.field_element(x).into(),
            self.a.as_ref().prime.field_element(y).into(),
        )
    }

    pub fn inf(&self) -> CurvePoint<F> {
        CurvePoint {
            c: self,
            p: Point::Inf,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CurvePoint<'a, F> {
    pub c: &'a Curve<F>,
    pub p: Point<F>,
}

impl<'a, 'b, 'c, F> ops::Add<&'c CurvePoint<'a, F>> for &'b CurvePoint<'a, F>
where
    F: AsRef<FieldElement<'a>> + From<FieldElement<'a>> + Debug + PartialEq + Clone,
{
    type Output = Result<CurvePoint<'a, F>>;

    fn add(self, other: &'c CurvePoint<'a, F>) -> Result<CurvePoint<'a, F>> {
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
            let s = (&(other_p.y.as_ref() - self_p.y.as_ref())?
                / &(other_p.x.as_ref() - self_p.x.as_ref())?)?;
            let x = (&(&s.pow(&2.into()) - self_p.x.as_ref())? - other_p.x.as_ref())?;
            let y = (&(&s * &(self_p.x.as_ref() - &x)?)? - self_p.y.as_ref())?;
            return self.c.point_from_field_element(x.into(), y.into());
        }

        if self_p.y != other_p.y {
            return Ok(self.c.inf());
        }

        if self_p.y.as_ref().is_zero() {
            return Ok(self.c.inf());
        }

        let s = (&(&(&BigInt::from(3) * &self_p.x.as_ref().pow(&2.into()))? + self.c.a.as_ref())?
            / &(&BigInt::from(2) * self_p.y.as_ref())?)?;
        let x = (&s.pow(&2.into()) - &(&BigInt::from(2) * self_p.x.as_ref())?)?;
        let y = (&(&s * &(self_p.x.as_ref() - &x)?)? - self_p.y.as_ref())?;
        self.c.point_from_field_element(x.into(), y.into())
    }
}

impl<'a, F> ops::Mul<CurvePoint<'a, F>> for BigInt
where
    F: AsRef<FieldElement<'a>> + From<FieldElement<'a>> + Debug + PartialEq + Clone,
{
    type Output = Result<CurvePoint<'a, F>>;

    fn mul(self, other: CurvePoint<'a, F>) -> Result<CurvePoint<'a, F>> {
        let mut coef = self;
        let mut current = other.clone();
        let mut result = other.c.inf();
        let zero: BigInt = 0.into();
        while coef != zero {
            if &coef & &1.into() != zero {
                result = (&result + &current)?;
            }
            current = (&current + &current)?;
            coef >>= 1;
        }
        Ok(result)
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
