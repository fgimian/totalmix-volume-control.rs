use std::{fs, path::PathBuf, str::FromStr};

use anyhow::Result;
use hex_color::HexColor;
use serde::Deserialize;
use windows::Win32::UI::Shell::{FOLDERID_RoamingAppData, SHGetKnownFolderPath, KF_FLAG_DEFAULT};

pub fn get_user_config() -> Result<Config> {
    let config_path = get_default_config_path()?;
    let config = fs::read_to_string(&config_path)?;
    Ok(toml::from_str::<Config>(&config)?)
}

fn get_default_config_path() -> Result<PathBuf> {
    let path = unsafe {
        SHGetKnownFolderPath(&FOLDERID_RoamingAppData, KF_FLAG_DEFAULT, None)?.to_string()?
    };
    Ok(PathBuf::from_str(&path)?
        .join("TotalMix Volume Control")
        .join("Config.toml"))
}

#[derive(Debug, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub background_rounding: f32,
    pub background_color: HexColor,
    pub heading_and_volume_bar_height: f32,
    pub heading_totalmix_color: HexColor,
    pub heading_volume_color: HexColor,
    pub heading_font_size: f32,
    pub volume_readout_color_normal: HexColor,
    pub volume_readout_color_dimmed: HexColor,
    pub volume_readout_font_size: f32,
    pub volume_bar_height: f32,
    pub volume_bar_top_margin: f32,
    pub volume_bar_horizontal_margin: f32,
    pub volume_bar_background_color: HexColor,
    pub volume_bar_foreground_color_normal: HexColor,
    pub volume_bar_foreground_color_dimmed: HexColor,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background_rounding: 10.0,
            background_color: HexColor::from_str("#1e2328e2").unwrap(),
            heading_and_volume_bar_height: 46.0,
            heading_totalmix_color: HexColor::WHITE,
            heading_volume_color: HexColor::from_str("#e06464").unwrap(),
            heading_font_size: 20.0,
            volume_readout_color_normal: HexColor::WHITE,
            volume_readout_color_dimmed: HexColor::from_str("#ffa500").unwrap(), // Orange
            volume_readout_font_size: 40.0,
            volume_bar_height: 10.0,
            volume_bar_top_margin: 7.0,
            volume_bar_horizontal_margin: 26.0,
            volume_bar_background_color: HexColor::from_str("#333333").unwrap(),
            volume_bar_foreground_color_normal: HexColor::from_str("#999999").unwrap(),
            volume_bar_foreground_color_dimmed: HexColor::from_str("#996500").unwrap(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub osc: Osc,
    pub volume: Volume,
    pub theme: Theme,
    pub interface: Interface,
}
