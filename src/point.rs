use anyhow::{anyhow, Result};
use num::traits::{Pow, Zero};
use num_bigint::{BigInt, BigUint};
use std::fmt;
use std::ops;

pub trait Ops<'a> = Sized
    + ops::Add<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + ops::Div<Self, Output = Self>
    + ops::Mul<Self, Output = Self>
    + ops::Add<&'a Self, Output = Self>
    + ops::Sub<&'a Self, Output = Self>
    + ops::Div<&'a Self, Output = Self>
    + ops::Mul<&'a Self, Output = Self>
    + ops::Mul<BigInt, Output = Self>
    + ops::Mul<&'a BigInt, Output = Self>
where
    Self: 'a,
    for<'b> &'b Self: ops::Add<Self, Output = Self>
        + ops::Sub<Self, Output = Self>
        + ops::Div<Self, Output = Self>
        + ops::Mul<Self, Output = Self>
        + ops::Add<&'b Self, Output = Self>
        + ops::Sub<&'b Self, Output = Self>
        + ops::Div<&'b Self, Output = Self>
        + ops::Mul<&'b Self, Output = Self>
        + ops::Mul<BigInt, Output = Self>
        + ops::Mul<&'b BigInt, Output = Self>
        + Pow<BigUint, Output = Self>;

pub trait Number<'a> = fmt::Debug + PartialEq + Ops<'a> + Zero + Clone;

#[derive(Debug, PartialEq, Clone)]
struct ActualPoint<N> {
    x: N,
    y: N,
}

#[derive(Debug, PartialEq, Clone)]
enum Point<N> {
    Inf,
    Actual(ActualPoint<N>),
}

impl<N> Point<N> {
    fn new(x: N, y: N) -> Point<N> {
        Point::Actual(ActualPoint { x, y })
    }

    fn into_actual(self) -> ActualPoint<N> {
        if let Point::Actual(p) = self {
            return p;
        }
        panic!("not normal point");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Curve<N> {
    a: N,
    b: N,
}

impl<'a, N> Curve<N>
where
    N: Number<'a>,
{
    pub fn new(a: N, b: N) -> Curve<N> {
        Curve { a, b }
    }

    pub fn point(&'a self, x: N, y: N) -> Result<CurvePoint<'a, N>> {
        if y.pow(2_u8.into()) != x.pow(3_u8.into()) + &self.a * &x + &self.b {
            return Err(anyhow!("({:?}, {:?}) is not on the curve", x, y));
        }
        Ok(CurvePoint {
            c: self,
            p: Point::new(x, y),
        })
    }

    pub fn inf<'b>(&'b self) -> CurvePoint<'b, N> {
        CurvePoint {
            c: self,
            p: Point::Inf,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CurvePoint<'a, N> {
    c: &'a Curve<N>,
    p: Point<N>,
}

impl<'a, N> ops::Add<CurvePoint<'a, N>> for CurvePoint<'a, N>
where
    N: Number<'a>,
{
    type Output = Result<CurvePoint<'a, N>>;

    fn add(self, other: CurvePoint<'a, N>) -> Result<CurvePoint<'a, N>> {
        if self.c != other.c {
            return Err(anyhow!(
                "Points {:?}, {:?} are not on the same curve",
                self,
                other
            ));
        }

        if self.p == Point::Inf {
            return Ok(other);
        }

        if other.p == Point::Inf {
            return Ok(self);
        }

        let self_p = self.p.into_actual();
        let other_p = other.p.into_actual();

        if self_p.x != other_p.x {
            let s = (&other_p.y - &self_p.y) / (&other_p.x - &self_p.x);
            let x = s.pow(2_u8.into()) - self_p.x.clone() - other_p.x;
            let y = s * (&self_p.x - &x) - self_p.y.clone();
            return self.c.point(x, y);
        }

        if self_p.y != other_p.y {
            return Ok(self.c.inf());
        }

        if self_p.y.is_zero() {
            return Ok(self.c.inf());
        }

        let s = (self_p.x.pow(2_u8.into()) * BigInt::from(3) + &self.c.a) / (&self_p.y * BigInt::from(2));
        let x = s.pow(2_u8.into()) - &self_p.x * BigInt::from(2);
        let y = s * (self_p.x - x.clone()) - self_p.y;
        self.c.point(x, y)
    }
}

#[test]
fn test_new() {
    let c = Curve::<BigInt>::new(5.into(), 7.into());
    let p1 = c.point((-1).into(), (-1).into());
    assert!(p1.is_ok());

    let p2 = c.point((-1).into(), (-2).into());
    assert!(p2.is_err());
}

#[test]
fn test_add_1() {
    let c = Curve::<BigInt>::new(5.into(), 7.into());
    let p1 = c.point(2.into(), 5.into()).unwrap();
    let p2 = c.point((-1).into(), (-1).into()).unwrap();
    assert_eq!((p1 + p2).unwrap(), c.point(3.into(), (-7).into()).unwrap());
}

#[test]
fn test_add_2() {
    let c = Curve::<BigInt>::new(5.into(), 7.into());
    let p1 = c.point((-1).into(), (-1).into()).unwrap();
    assert_eq!((p1.clone() + p1).unwrap(), c.point(18.into(), 77.into()).unwrap());
}
