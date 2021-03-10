use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Clone)]
struct Point {
    a: i64,
    b: i64,
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64, a: i64, b: i64) -> Result<Point> {
        if y.pow(2) != x.pow(3) + a * x + b {
            return Err(anyhow!("Cannot add two numbers in deffirent Fields"));
        }
        Ok(Point { a, b, x, y })
    }
}


#[test]
fn test_new() {
    let p1 = Point::new(-1, -1, 5, 7);
    assert!(p1.is_ok());

    let p2 = Point::new(-1, -2, 5, 7);
    assert!(p2.is_err());
}
