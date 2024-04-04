use study002;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, study002::add(2, 2));
}
// cargo test --test integration_test
