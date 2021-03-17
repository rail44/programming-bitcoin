use anyhow::{anyhow, Result};
use std::ops;

#[derive(Debug, PartialEq, Clone)]
enum Point {
    Inf,
    Normal {
        x: i64,
        y: i64,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct CurvePoint {
    a: i64,
    b: i64,
    p: Point,
}

impl CurvePoint {
    fn new(x: i64, y: i64, a: i64, b: i64) -> Result<CurvePoint> {
        if y.pow(2) != x.pow(3) + a * x + b {
            return Err(anyhow!("({}, {}) is not on the curve", x, y));
        }
        Ok(CurvePoint { a, b, p: Point::Normal { x, y } })
    }

    fn inf(a: i64, b: i64) -> CurvePoint {
        CurvePoint { a, b, p: Point::Inf }
    }
}

impl ops::Add<CurvePoint> for CurvePoint {
    type Output = Result<CurvePoint>;

    fn add(self, other: CurvePoint) -> Result<CurvePoint> {
        if self.a != other.a || self.b != other.b {
            return Err(anyhow!("Points {:?}, {:?} are not on the same curve", self, other));
        }

        if self.p == Point::Inf {
            return Ok(other);
        }

        if other.p == Point::Inf {
            return Ok(self);
        }

        panic!();
    }
}


#[test]
fn test_new() {
    let p1 = CurvePoint::new(-1, -1, 5, 7);
    assert!(p1.is_ok());

    let p2 = CurvePoint::new(-1, -2, 5, 7);
    assert!(p2.is_err());
}
