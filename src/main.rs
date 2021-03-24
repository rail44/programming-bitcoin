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
        (p1 + p2).unwrap(),
        c.point(FieldElement::new(170, 223), FieldElement::new(142, 223))
            .unwrap()
    );
}
