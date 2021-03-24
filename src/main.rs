#![feature(trait_alias)]

mod field_element;
mod point;

fn main() {
    println!("Hello, world!");
}

// #[test]
// fn test_add_field_element() {
//     use field_element::FieldElement;
//     use point::Curve;
//
//     let a = FieldElement::new(0, 223);
//     let b = FieldElement::new(7, 223);
//     let x = FieldElement::new(192, 223);
//     let y = FieldElement::new(105, 223);
//     let c = Curve::new(a, b);
//     let p1 = c.point(x, y);
//     assert!(p1.is_ok());
// }
