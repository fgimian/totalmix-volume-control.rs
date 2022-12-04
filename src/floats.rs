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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roughly_eq_f32() {
        let float = 10.54f32;
        assert!(!float.roughly_eq(10.53));
        assert!(float.roughly_eq(10.54));
        assert!(!float.roughly_eq(10.55));
    }

    #[test]
    fn roughly_ne_f32() {
        let float = 10.54f32;
        assert!(float.roughly_ne(10.53));
        assert!(!float.roughly_ne(10.54));
        assert!(float.roughly_ne(10.55));
    }

    #[test]
    fn roughly_eq_f64() {
        let float = 10.54f64;
        assert!(!float.roughly_eq(10.53));
        assert!(float.roughly_eq(10.54));
        assert!(!float.roughly_eq(10.55));
    }

    #[test]
    fn roughly_ne_f64() {
        let float = 10.54f64;
        assert!(float.roughly_ne(10.53));
        assert!(!float.roughly_ne(10.54));
        assert!(float.roughly_ne(10.55));
    }
}
