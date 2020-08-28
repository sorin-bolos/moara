const EPSILON:f32 = 0.000001;

pub fn equals(a:f32, b:f32) -> bool
{
    (b-a).abs() < EPSILON
}