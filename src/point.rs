use anyhow::{anyhow, Result};
use std::ops;

#[derive(Debug, PartialEq, Clone)]
struct ActualPoint {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Clone)]
enum Point {
    Inf,
    Actual(ActualPoint),
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point::Actual(ActualPoint { x, y })
    }

    fn into_normal_point(self) -> ActualPoint {
        if let Point::Actual(p) = self {
            return p;
        }
        panic!("not normal point");
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
        Ok(CurvePoint { a, b, p: Point::new(x, y) })
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

        let self_p = self.p.into_normal_point();
        let other_p = other.p.into_normal_point();

        if self_p.x != other_p.x {
            let s = (other_p.y - self_p.y) / (other_p.x - self_p.x);
            let x = s.pow(2) - self_p.x - other_p.x;
            let y = s * (self_p.x - x) - self_p.y;
            return CurvePoint::new(x, y, self.a, self.b);
        }

        if self_p.x == other_p.x && self_p.y != other_p.y {
            return Ok(CurvePoint::inf(self.a, self.b));
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

#[test]
fn test_add_1() {
    let p1 = CurvePoint::new(2, 5, 5, 7).unwrap();
    let p2 = CurvePoint::new(-1, -1, 5, 7).unwrap();
    assert_eq!((p1 + p2).unwrap(), CurvePoint::new(3, -7, 5, 7).unwrap());
}
