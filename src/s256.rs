use crate::field_element::Prime;
use crate::point::{Curve, CurvePoint};
use anyhow::Result;
use num_bigint::BigInt;
use num_traits::Pow;
use once_cell::sync::Lazy;
use std::ops;

static P: Lazy<Prime> =
    Lazy::new(|| Prime::new(BigInt::from(2).pow(256_u16) - BigInt::from(2).pow(32_u8) - 977));
static C: Lazy<Curve> = Lazy::new(|| P.curve(0, 7));
static N: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
});
static G: Lazy<S256Point> = Lazy::new(|| {
    S256Point::new(
        BigInt::parse_bytes(
            b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap(),
        BigInt::parse_bytes(
            b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        )
        .unwrap(),
    )
    .unwrap()
});

#[derive(Debug, PartialEq, Clone)]
pub struct S256Point<'a> {
    pub cp: CurvePoint<'a>,
}

impl<'a> S256Point<'a> {
    pub fn new<A, B>(x: A, y: B) -> Result<S256Point<'a>>
    where
        A: Into<BigInt>,
        B: Into<BigInt>,
    {
        C.point(x, y).map(|cp| cp.into())
    }

    pub fn inf() -> S256Point<'a> {
        C.inf().into()
    }
}

impl<'a> ops::Deref for S256Point<'a> {
    type Target = CurvePoint<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cp
    }
}

impl<'a> From<CurvePoint<'a>> for S256Point<'a> {
    fn from(cp: CurvePoint<'a>) -> S256Point<'a> {
        Self { cp }
    }
}

impl<'a> ops::Mul<S256Point<'a>> for BigInt {
    type Output = Result<S256Point<'a>>;

    fn mul(self, other: S256Point<'a>) -> Result<S256Point<'a>> {
        let coef = self % N.clone();
        coef.mul(other.cp).map(|cp| cp.into())
    }
}

#[test]
fn test_3_9() {
    assert_eq!((N.clone() * G.clone()).unwrap(), C.inf().into());
}
