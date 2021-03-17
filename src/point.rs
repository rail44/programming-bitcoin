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

    fn into_actual(self) -> ActualPoint {
        if let Point::Actual(p) = self {
            return p;
        }
        panic!("not normal point");
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Curve {
    a: i64,
    b: i64,
}

impl Curve {
    fn new(a: i64, b: i64) -> Curve {
        Curve {
            a,
            b,
        }
    }

    fn point<'a>(&'a self, x: i64, y: i64) -> Result<CurvePoint<'a>> {
        if y.pow(2) != x.pow(3) + self.a * x + self.b {
            return Err(anyhow!("({}, {}) is not on the curve", x, y));
        }
        Ok(CurvePoint {
            c: self,
            p: Point::new(x, y),
        })
    }

    fn inf<'a>(&'a self) -> CurvePoint<'a> {
        CurvePoint {
            c: self,
            p: Point::Inf,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct CurvePoint<'a> {
    c: &'a Curve,
    p: Point,
}

impl<'a> ops::Add<CurvePoint<'a>> for CurvePoint<'a> {
    type Output = Result<CurvePoint<'a>>;

    fn add(self, other: CurvePoint<'a>) -> Result<CurvePoint<'a>> {
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
            let s = (other_p.y - self_p.y) / (other_p.x - self_p.x);
            let x = s.pow(2) - self_p.x - other_p.x;
            let y = s * (self_p.x - x) - self_p.y;
            return self.c.point(x, y);
        }

        if self_p.y != other_p.y {
            return Ok(self.c.inf());
        }

        if self_p.y == 0 {
            return Ok(self.c.inf());
        }

        let s = (3 * self_p.x.pow(2) + self.c.a) / (2 * self_p.y);
        let x = s.pow(2) - 2 * self_p.x;
        let y = s * (self_p.x - x) - self_p.y;
        self.c.point(x, y)
    }
}

#[test]
fn test_new() {
    let c = Curve::new(5, 7);
    let p1 = c.point(-1, -1);
    assert!(p1.is_ok());

    let p2 = c.point(-1, -2);
    assert!(p2.is_err());
}

#[test]
fn test_add_1() {
    let c = Curve::new(5, 7);
    let p1 = c.point(2, 5).unwrap();
    let p2 = c.point(-1, -1).unwrap();
    assert_eq!((p1 + p2).unwrap(), c.point(3, -7).unwrap());
}

#[test]
fn test_add_2() {
    let c = Curve::new(5, 7);
    let p1 = c.point(-1, -1).unwrap();
    assert_eq!(
        (p1.clone() + p1).unwrap(),
        c.point(18, 77).unwrap()
    );
}
