mod common;

#[test]
fn it_adds_two() {
    common::set_up();
    assert_eq!(4, rust_learning::add_two(2));
}
