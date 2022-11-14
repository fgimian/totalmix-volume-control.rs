pub fn roughly_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}

pub fn roughly_ne(a: f32, b: f32) -> bool {
    !roughly_eq(a, b)
}
