pub trait RoughEq<T> {
    fn roughly_eq(&self, other: T) -> bool;

    fn roughly_ne(&self, other: T) -> bool {
        !self.roughly_eq(other)
    }
}

impl RoughEq<f32> for f32 {
    fn roughly_eq(&self, other: f32) -> bool {
        (self - other).abs() < f32::EPSILON
    }
}

impl RoughEq<f64> for f64 {
    fn roughly_eq(&self, other: f64) -> bool {
        (self - other).abs() < f64::EPSILON
    }
}
