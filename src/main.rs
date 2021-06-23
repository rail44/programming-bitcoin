#![feature(trait_alias)]

mod field_element;
mod helper;
mod point;
mod s256;
mod transaction;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_add_field_element() {
    use field_element::Prime;

    let prime = Prime::new(223);
    let c = prime.curve(0, 7);

    let p1 = c.point(192, 105).unwrap();
    let p2 = c.point(17, 56).unwrap();
    assert_eq!((&p1 + &p2).unwrap(), c.point(170, 142).unwrap());
}

#[test]
fn test_exam_3_4() {
    use field_element::Prime;
    use point::Point;

    let prime = Prime::new(223);
    let c = prime.curve(0, 7);

    // 1
    let p1 = c.point(192, 105).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(r1, c.point(49, 71).unwrap());

    // 2
    let p1 = c.point(143, 98).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(r1, c.point(64, 168).unwrap());

    // 3
    let p1 = c.point(47, 71).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(r1, c.point(36, 111).unwrap());

    // 4
    let p1 = c.point(47, 71).unwrap();

    let mut r1 = (&p1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    assert_eq!(r1, c.point(194, 51).unwrap());

    // 5
    let p1 = c.point(47, 71).unwrap();

    let mut r1 = (&p1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    assert_eq!(r1, c.point(116, 55).unwrap());

    // 6
    let p1 = c.point(47, 71).unwrap();

    let mut r1 = c.inf();
    for _ in 0..21 {
        r1 = (&r1 + &p1).unwrap();
    }
    assert_eq!(&r1.p, &Point::Inf);
}

#[test]
fn test_exam_3_5() {
    use field_element::Prime;
    use point::Point;

    let prime = Prime::new(223);
    let c = prime.curve(0, 7);

    let p1 = c.point(15, 86).unwrap();

    let mut r1 = p1.clone();
    let mut i = 1;
    while r1.p != Point::Inf {
        r1 = (&r1 + &p1).unwrap();
        i += 1;
    }
    assert_eq!(i, 7);
}

#[test]
fn test_mul() {
    use field_element::Prime;
    use num_bigint::BigInt;
    use point::Point;

    let prime = Prime::new(223);
    let c = prime.curve(0, 7);
    let p1 = c.point(47, 71).unwrap();

    let r1 = (BigInt::from(4) * p1).unwrap();
    assert_eq!(r1, c.point(194, 51).unwrap());

    let p1 = c.point(47, 71).unwrap();

    let r1 = (BigInt::from(8) * p1).unwrap();
    assert_eq!(r1, c.point(116, 55).unwrap());

    let p1 = c.point(47, 71).unwrap();

    let r1 = (BigInt::from(21) * p1).unwrap();
    assert_eq!(&r1.p, &Point::Inf);
}

#[test]
fn test_3_9_1() {
    use num_bigint::BigInt;
    use num_traits::Pow;
    let gx = BigInt::parse_bytes(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let gy = BigInt::parse_bytes(
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();
    let p = BigInt::from(2).pow(256_u16) - BigInt::from(2).pow(32_u8) - 977;
    assert_eq!(gy.pow(2_u8) % &p, (gx.pow(3_u8) + 7) % p);
}
