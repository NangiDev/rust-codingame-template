use {{crate_name}}::advanced_math::multiply;
use {{crate_name}}::math::add;

#[test]
fn addition() {
    assert_eq!(5, add(2, 3), "Testing so that addition is working")
}

#[test]
fn multiply() {
    assert_eq!(6, add(2, 3), "Testing so that multiplucation is working")
}
