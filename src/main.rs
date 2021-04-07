#![feature(trait_alias)]

mod field_element;
mod point;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_add_field_element() {
    use field_element::Prime;

    let prime = Prime::new(223);
    let c = prime.curve(0, 7);

    let x1 = prime.field_element(192);
    let y1 = prime.field_element(105);

    let x2 = prime.field_element(17);
    let y2 = prime.field_element(56);

    let p1 = c.point(x1, y1).unwrap();
    let p2 = c.point(x2, y2).unwrap();
    assert_eq!(
        (&p1 + &p2).unwrap(),
        c.point(prime.field_element(170), prime.field_element(142))
            .unwrap()
    );
}

#[test]
fn test_exam_3_4() {
    use field_element::Prime;
    use point::Point;

    let prime = Prime::new(223);
    let c = prime.curve(0, 7);

    // 1
    let x1 = prime.field_element(192);
    let y1 = prime.field_element(105);

    let p1 = c.point(x1, y1).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&prime.field_element(49), &prime.field_element(71))
    );

    // 2
    let x1 = prime.field_element(143);
    let y1 = prime.field_element(98);

    let p1 = c.point(x1, y1).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&prime.field_element(64), &prime.field_element(168))
    );

    // 3
    let x1 = prime.field_element(47);
    let y1 = prime.field_element(71);

    let p1 = c.point(x1, y1).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&prime.field_element(36), &prime.field_element(111))
    );

    // 4
    let x1 = prime.field_element(47);
    let y1 = prime.field_element(71);

    let p1 = c.point(x1, y1).unwrap();

    let mut r1 = (&p1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&prime.field_element(194), &prime.field_element(51))
    );

    // 5
    let x1 = prime.field_element(47);
    let y1 = prime.field_element(71);

    let p1 = c.point(x1, y1).unwrap();

    let mut r1 = (&p1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&prime.field_element(116), &prime.field_element(55))
    );

    // 6
    let x1 = prime.field_element(47);
    let y1 = prime.field_element(71);

    let p1 = c.point(x1, y1).unwrap();

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

    let x1 = prime.field_element(15);
    let y1 = prime.field_element(86);

    let p1 = c.point(x1, y1).unwrap();

    let mut r1 = p1.clone();
    let mut i = 1;
    while r1.p != Point::Inf {
        r1 = (&r1 + &p1).unwrap();
        i += 1;
    }
    assert_eq!(i, 7);
}
