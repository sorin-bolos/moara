#[cfg(test)]

#[macro_use]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::gates;


#[test]
fn swap_gives_correct_opertator() {
  let swap_4 = gates::swap(4);
  let expected_data = vec![
    vec![ C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1)]];

    assert_eq!(&expected_data, swap_4.data());
}