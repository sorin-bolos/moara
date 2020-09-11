#[cfg(test)]

extern crate moara;
extern crate num_complex;

use core::f32::consts::PI;
use num_complex::Complex32;
use moara::operator::Operator;
use moara::gates;
use moara::C;
use moara::tools;



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
    let expected_data_1= vec![
      vec![ C!(1), C!(0)],
      vec![ C!(0), C!(1)]];
    assert_eq!(&expected_data_1, u3_1.data());

    let u3_2= gates::u3_gate(PI/2.0,-PI/2.0,PI/2.0);
    let expected = Operator::new(vec![
      vec![ C!((1.0/2_f32.sqrt())), C!((-1.0/2_f32.sqrt())*i)],
      vec![ C!((-1.0/2_f32.sqrt())*i), C!((1.0/2_f32.sqrt()))]]);

    assert!(operators_are_equal(expected, u3_2));
}


// #[test]
// fn cu3_gives_correct_opertator() {
// let cu3_i = gates::cu3_gate(0,2,0.0,0.0,0.0);
// let expected_data_i = vec![
//   vec![ C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
//    vec![ C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
//    vec![ C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0)],
//    vec![ C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0)],
//   vec![ C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0)],
//   vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0)],
//   vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0)],
//   vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1)]];
//   assert_eq!(&expected_data_i, cu3_i.data());
// }

fn operators_are_equal(a:Operator, b:Operator) -> bool
{
    if a.data().len() != b.data().len()
    {
        return false;
    }

    for i in 0..a.data().len()
    {
        for j in 0..a.data().len() 
        {
            if !(tools::equals(a.data()[i][j].re, b.data()[i][j].re) && tools::equals(a.data()[i][j].im, b.data()[i][j].im))
            {
                return false;
            }
        }
    }

    return true;
}