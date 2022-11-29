use egui::{hex_color, Color32};

pub struct Theme {
    pub background_rounding: f32,
    pub background_color: Color32,
    pub heading_and_volume_bar_height: f32,
    pub heading_font_size: f32,
    pub heading_totalmix_color: Color32,
    pub heading_volume_color: Color32,
    pub volume_readout_color_normal: Color32,
    pub volume_readout_color_dimmed: Color32,
    pub volume_readout_font_size: f32,
    pub volume_bar_height: f32,
    pub volume_bar_top_margin: f32,
    pub volume_bar_horizontal_margin: f32,
    pub volume_bar_background_color: Color32,
    pub volume_bar_foreground_color_normal: Color32,
    pub volume_bar_foreground_color_dimmed: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background_rounding: 10.0,
            background_color: hex_color!("#1e2328e2"),
            heading_and_volume_bar_height: 46.0,
            heading_font_size: 20.0,
            heading_totalmix_color: Color32::WHITE,
            heading_volume_color: hex_color!("#e06464"),
            volume_readout_color_normal: Color32::WHITE,
            volume_readout_color_dimmed: hex_color!("#ffa500"), // Orange
            volume_readout_font_size: 40.0,
            volume_bar_height: 10.0,
            volume_bar_top_margin: 7.0,
            volume_bar_horizontal_margin: 26.0,
            volume_bar_background_color: hex_color!("#333333"),
            volume_bar_foreground_color_normal: hex_color!("#999999"),
            volume_bar_foreground_color_dimmed: hex_color!("#996500"),
        }
    }
}

pub struct Timing {
    pub hide_delay: f64,
    pub fade_out_time: f32,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            hide_delay: 2.0,
            fade_out_time: 1.0,
        }
    }
}

pub struct Config {
    pub timing: Timing,
    pub theme: Theme,
}

impl Config {
    pub fn new() -> Self {
        Self {
            timing: Timing::default(),
            theme: Theme::default(),
        }
    }
}
