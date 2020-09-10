#[macro_export]
macro_rules! C {
    ($re:tt+$im:tt*i) => (Complex32::new($re as f32, $im as f32));
    ($re:tt-$im:tt*i) => (Complex32::new($re as f32, -$im as f32));
    ($re:tt) => (Complex32::new($re as f32, 0.0));
    (-$re:tt) => (Complex32::new(-$re as f32, 0.0));
    ($im:tt*i) => (Complex32::new(0.0, $im as f32));
    (-$im:tt*i) => (Complex32::new(0.0, -$im as f32));
}

#[cfg(test)]
mod test {
    use num_complex::Complex32;

    #[test]
    fn complex_macro_works() {
        assert_eq!(Complex32::new(2.4, 3.5), C!(2.4+3.5*i));
        assert_eq!(Complex32::new(2.4, -3.5), C!(2.4-3.5*i));
        assert_eq!(Complex32::new(4.8, 3.5), C!((2.4*2f32)+3.5*i));
        assert_eq!(Complex32::new(1.2, -3.5), C!((2.4/2f32)-3.5*i));
        assert_eq!(Complex32::new(12.0,0.0), C!(12));
        assert_eq!(Complex32::new(-12.0,0.0), C!(-12));
        assert_eq!(Complex32::new(0.0,12.0), C!(12*i));
        assert_eq!(Complex32::new(0.0,-12.0), C!(-12*i));

        let im = -3;
        assert_eq!(Complex32::new(0.0,-3.0), C!(im*i));
    }
}