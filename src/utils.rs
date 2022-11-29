use egui::{color, Color32};

pub fn roughly_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}

pub fn roughly_ne(a: f32, b: f32) -> bool {
    !roughly_eq(a, b)
}

pub fn apply_alpha(color: Color32, alpha: f32) -> Color32 {
    let r = color::linear_f32_from_linear_u8(color.r()) * alpha;
    let r = color::linear_u8_from_linear_f32(r);

    let g = color::linear_f32_from_linear_u8(color.g()) * alpha;
    let g = color::linear_u8_from_linear_f32(g);

    let b = color::linear_f32_from_linear_u8(color.b()) * alpha;
    let b = color::linear_u8_from_linear_f32(b);

    let a = color::linear_f32_from_linear_u8(color.a()) * alpha;
    let a = color::linear_u8_from_linear_f32(a);

    Color32::from_rgba_premultiplied(r, g, b, a)
}
