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

  let swap_2 = gates::swap(2);
  let expected_data_2 = vec![
    vec![ C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1)]];
    assert_eq!(&expected_data_2, swap_2.data());
  
}


#[test]
fn cx_gives_correct_opertator() {
 
  let cx_3 = gates::cx(3,true);
  let expected_data_3 = vec![
    vec![ C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0)]];
  assert_eq!(&expected_data_3, cx_3.data());

  let cx_2 = gates::cx(2,true);
  let expected_data_2 = vec![
    vec![ C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1)],
    vec![ C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0)]];
    assert_eq!(&expected_data_2, cx_2.data());
}

#[test]
fn u3_gives_correct_opertator() {
  let u3_1= gates::u3_gate(0.0,0.0,0.0);
  println!("u3:{:?}",u3_1.data());
  let expected_data_1= vec![
    vec![ C!(1), C!(0)],
    vec![ C!(0), C!(1)]];
  assert_eq!(&expected_data_1, u3_1.data());

  let u3_2= gates::u3_gate(0.0,3.16,0.0);
  println!("u3_2:{:?}",u3_2.data());
  let expected_data_2= vec![
    vec![ C!(1), C!(0)],
    vec![ C!(0), C!(-0.00920354+0.99995765*i)]];
  

}
/*
#[test]
fn cu3_gives_correct_opertator() {
  let cx = gates::cx(3,true);
  let expected_data = vec![
    vec![ C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0)]];

    assert_eq!(&expected_data, cx.data());
}
*/