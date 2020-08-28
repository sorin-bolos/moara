pub fn aprox_equals(a:i32, b:i32, fraction:f32) -> bool
{
    let diff = (b-a).abs();
    let sum = (b+a).abs();

    (diff as f32) <= (sum as f32) * fraction
}

pub fn check_distribution(counts:[i32;2], distribution:[f32;2], tolerance:f32) -> bool
{
    let sum = (counts[0]+counts[1]).abs();
    let prob0 = counts[0] as f32/sum as f32;
    let prob1 = counts[1] as f32/sum as f32;

    (distribution[0]-prob0).abs() < tolerance 
      && (distribution[1]-prob1).abs() < tolerance
}