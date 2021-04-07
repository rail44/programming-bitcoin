#![feature(trait_alias)]

mod field_element;
mod point;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_add_field_element() {
    use field_element::FieldElement;
    use point::Curve;

    let a = FieldElement::new(0, 223);
    let b = FieldElement::new(7, 223);
    let c = Curve::new(a, b);

    let x1 = FieldElement::new(192, 223);
    let y1 = FieldElement::new(105, 223);

    let x2 = FieldElement::new(17, 223);
    let y2 = FieldElement::new(56, 223);

    let p1 = c.point(x1, y1).unwrap();
    let p2 = c.point(x2, y2).unwrap();
    assert_eq!(
        (&p1 + &p2).unwrap(),
        c.point(FieldElement::new(170, 223), FieldElement::new(142, 223))
            .unwrap()
    );
}

#[test]
fn test_exam_3_4() {
    use field_element::FieldElement;
    use point::{Point, Curve};

    let a = FieldElement::new(0, 223);
    let b = FieldElement::new(7, 223);
    let c = Curve::new(a, b);

    // 1
    let x1 = FieldElement::new(192, 223);
    let y1 = FieldElement::new(105, 223);

    let p1 = c.point(x1, y1).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&FieldElement::new(49, 223), &FieldElement::new(71, 223))
    );

    // 2
    let x1 = FieldElement::new(143, 223);
    let y1 = FieldElement::new(98, 223);

    let p1 = c.point(x1, y1).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&FieldElement::new(64, 223), &FieldElement::new(168, 223))
    );

    // 3
    let x1 = FieldElement::new(47, 223);
    let y1 = FieldElement::new(71, 223);

    let p1 = c.point(x1, y1).unwrap();

    let r1 = (&p1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&FieldElement::new(36, 223), &FieldElement::new(111, 223))
    );

    // 4
    let x1 = FieldElement::new(47, 223);
    let y1 = FieldElement::new(71, 223);

    let p1 = c.point(x1, y1).unwrap();

    let mut r1 = (&p1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    r1 = (&r1 + &p1).unwrap();
    assert_eq!(
        (&r1.p.as_actual().x, &r1.p.as_actual().y),
        (&FieldElement::new(194, 223), &FieldElement::new(51, 223))
    );

    // 5
    let x1 = FieldElement::new(47, 223);
    let y1 = FieldElement::new(71, 223);

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
        (&FieldElement::new(116, 223), &FieldElement::new(55, 223))
    );

    // 5
    let x1 = FieldElement::new(47, 223);
    let y1 = FieldElement::new(71, 223);

    let p1 = c.point(x1, y1).unwrap();

    let mut r1 = p1.clone();
    for _ in 0..20 {
        r1 = (&r1 + &p1).unwrap();
    }
    assert_eq!(
        &r1.p,
        &Point::Inf
    );
}

#[test]
fn test_exam_3_5() {
    use field_element::FieldElement;
    use point::{Point, Curve};

    let a = FieldElement::new(0, 223);
    let b = FieldElement::new(7, 223);
    let c = Curve::new(a, b);


    let x1 = FieldElement::new(15, 223);
    let y1 = FieldElement::new(86, 223);

    let p1 = c.point(x1, y1).unwrap();

    let mut r1 = p1.clone();
    let mut i = 1;
    while r1.p != Point::Inf {
        r1 = (&r1 + &p1).unwrap();
        i += 1;
    }
    assert_eq!(i, 7);
}
