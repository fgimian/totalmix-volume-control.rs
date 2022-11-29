use egui::{hex_color, Color32};

pub struct Osc {
    pub outgoing_hostname: String,
    pub outgoing_port: u16,
    pub incoming_hostname: String,
    pub incoming_port: u16,
}

impl Default for Osc {
    fn default() -> Self {
        Self {
            outgoing_hostname: "127.0.0.1".to_string(),
            outgoing_port: 7001,
            incoming_hostname: "127.0.0.1".to_string(),
            incoming_port: 9001,
        }
    }
}

// TODO: Implement usage of the volume configuration.
pub struct Volume {
    pub increment: f32,
    pub fine_increment: f32,
    pub max_volume: f32,
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            increment: 0.02,
            fine_increment: 0.01,
            max_volume: 1.0,
        }
    }
}

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

pub struct Interface {
    pub scaling: f32,
    pub position_offset: f64,
    pub hide_delay: f64,
    pub fade_out_time: f32,
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            scaling: 1.0,
            position_offset: 40.0,
            hide_delay: 2.0,
            fade_out_time: 1.0,
        }
    }
}

pub struct Config {
    pub osc: Osc,
    pub volume: Volume,
    pub theme: Theme,
    pub interface: Interface,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            osc: Osc::default(),
            volume: Volume::default(),
            interface: Interface::default(),
            theme: Theme::default(),
        }
    }
}
