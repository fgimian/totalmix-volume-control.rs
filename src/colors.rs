use egui::Color32;
use hex_color::HexColor;

pub trait ToColor32 {
    fn to_colour32(&self) -> Color32;
    fn to_colour32_scaled(&self, scale: f32) -> Color32;
}

impl ToColor32 for HexColor {
    fn to_colour32(&self) -> Color32 {
        Color32::from_rgba_premultiplied(self.r, self.g, self.b, self.a)
    }

    fn to_colour32_scaled(&self, scale: f32) -> Color32 {
        let r = fast_round(self.r as f32 * scale);
        let g = fast_round(self.g as f32 * scale);
        let b = fast_round(self.b as f32 * scale);
        let a = fast_round(scale * 255.0);
        Color32::from_rgba_premultiplied(r, g, b, a)
    }
}

// See https://github.com/rust-lang/rust/issues/55107 and
// https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html
fn fast_round(r: f32) -> u8 {
    (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
}
