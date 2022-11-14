#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo,
    clippy::expect_used,
    clippy::unwrap_used
)]
mod comms;
mod hotkeys;
mod manager;

use comms::{UdpReceiver, UdpSender};
use eframe::{App, CreationContext, NativeOptions};
use egui::{
    hex_color, pos2, style::DebugOptions, text::LayoutJob, vec2, Align, CentralPanel, Color32,
    Context, Direction, FontData, FontDefinitions, FontFamily, FontId, Layout, Rect, Rgba,
    RichText, Rounding, Sense, TextFormat, Vec2, Visuals,
};
use hotkeys::{HotKey, HotKeys};
use manager::VolumeManager;
use std::{net::SocketAddrV4, sync::Arc, thread};

fn main() {
    let sender = UdpSender::new(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 7002)).unwrap();
    let receiver =
        UdpReceiver::bind(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 9002)).unwrap();

    let manager = Arc::new(VolumeManager::new(sender, receiver));
    // Create the thread that will receive volume changes from the device.
    {
        let manager = manager.clone();
        thread::Builder::new()
            .name("receiver".to_string())
            .spawn(move || {
                manager.request_volume().unwrap();
                loop {
                    if manager.recieve_volume().unwrap() {}
                }
            })
            .unwrap();
    }

    // Create the thread that will receive hotkeys and update the volume.  It will also send a
    // message to the main GUI thread.
    {
        let manager = manager.clone();
        thread::Builder::new()
            .name("hotkeys".to_string())
            .spawn(move || {
                let hotkeys = HotKeys::new().unwrap();
                loop {
                    let hotkey = hotkeys.receive().unwrap();
                    match hotkey {
                        HotKey::VolumeUp => manager.increase_volume().unwrap(),
                        HotKey::VolumeDown => manager.decrease_volume().unwrap(),
                        HotKey::VolumeUpFine => manager.increase_volume_fine().unwrap(),
                        HotKey::VolumeDownfine => manager.decrease_volume_fine().unwrap(),
                        HotKey::Mute => manager.toggle_dim().unwrap(),
                    };
                }
            })
            .unwrap();
    }

    let manager = manager.clone();

    let native_options = NativeOptions {
        always_on_top: true,
        decorated: false,
        drag_and_drop_support: false,
        initial_window_pos: Some(pos2(40.0, 40.0)),
        initial_window_size: Some(vec2(165.0, 165.0)),
        resizable: false,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(
        "TotalMix Volume Control",
        native_options,
        Box::new(|cc| Box::new(TotalMixVolumeControl::new(cc, manager))),
    );
}

struct TotalMixVolumeControl {
    volume_manager: Arc<VolumeManager<UdpSender, UdpReceiver>>,
    background_rounding: f32,
    background_color: Color32,
    heading_and_volume_bar_height: f32,
    heading_font_size: f32,
    heading_totalmix_color: Color32,
    heading_volume_color: Color32,
    volume_readout_color_normal: Color32,
    volume_readout_color_dimmed: Color32,
    volume_readout_font_size: f32,
    volume_bar_height: f32,
    volume_bar_top_margin: f32,
    volume_bar_horizontal_margin: f32,
    volume_bar_bg_color: Color32,
    volume_bar_fg_color_normal: Color32,
    volume_bar_fg_color_dimmed: Color32,
}

impl TotalMixVolumeControl {
    fn new(cc: &CreationContext<'_>, manager: Arc<VolumeManager<UdpSender, UdpReceiver>>) -> Self {
        // Set the default font.
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Segoe UI".to_string(),
            FontData::from_static(include_bytes!(r"C:\Windows\Fonts\segoeui.ttf")),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "Segoe UI".to_string());
        cc.egui_ctx.set_fonts(fonts);

        Self {
            volume_manager: manager,
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
            volume_bar_bg_color: hex_color!("#333333"),
            volume_bar_fg_color_normal: hex_color!("#999999"),
            volume_bar_fg_color_dimmed: hex_color!("#996500"),
        }
    }
}

impl App for TotalMixVolumeControl {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // println!("update called");
        // frame.set_visible(false);

        CentralPanel::default()
            .frame(egui::Frame {
                rounding: Rounding::same(self.background_rounding),
                fill: self.background_color,
                ..Default::default()
            })
            .show(ctx, |ui| {
                {
                    let mut style = ui.style_mut();
                    style.debug = DebugOptions {
                        debug_on_hover: true,
                        show_expand_width: true,
                        show_expand_height: true,
                        show_resize: true,
                    };

                    style.spacing.item_spacing = Vec2::ZERO;
                }

                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), self.heading_and_volume_bar_height),
                    Layout::centered_and_justified(Direction::TopDown).with_main_align(Align::Max),
                    |ui| {
                        let mut job = LayoutJob::default();
                        job.append(
                            "TotalMix ",
                            0.0,
                            TextFormat {
                                font_id: FontId::proportional(self.heading_font_size),
                                color: self.heading_totalmix_color,
                                ..Default::default()
                            },
                        );
                        job.append(
                            "Volume",
                            0.0,
                            TextFormat {
                                font_id: FontId::proportional(self.heading_font_size),
                                color: self.heading_volume_color,
                                ..Default::default()
                            },
                        );
                        ui.label(job);
                    },
                );

                let (volume_db, volume, dimmed) = {
                    let volume_db = self.volume_manager.volume_db().unwrap_or("-".to_string());
                    (
                        volume_db,
                        self.volume_manager.volume(),
                        self.volume_manager.dimmed(),
                    )
                };
                // ctx.request_repaint();

                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        ui.available_height() - self.heading_and_volume_bar_height,
                    ),
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| {
                        ui.label(
                            RichText::new(volume_db)
                                .size(self.volume_readout_font_size)
                                .color(match dimmed {
                                    true => self.volume_readout_color_dimmed,
                                    false => self.volume_readout_color_normal,
                                }),
                        );
                    },
                );

                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), self.heading_and_volume_bar_height),
                    Layout::centered_and_justified(Direction::TopDown).with_main_align(Align::Min),
                    |ui| {
                        // Add a little top padding to align with the text above which has a little
                        // padding due to the font used.
                        ui.add_space(self.volume_bar_top_margin);

                        // Ideas pinched from the implementation of ProgressBar.
                        let (volume_bar_bg, _response) = ui.allocate_exact_size(
                            vec2(
                                ui.available_width() - self.volume_bar_horizontal_margin * 2.0,
                                self.volume_bar_height,
                            ),
                            Sense::hover(),
                        );

                        let volume_bar_fg = Rect::from_min_size(
                            volume_bar_bg.min,
                            vec2(volume_bar_bg.width() * volume, volume_bar_bg.height()),
                        );

                        ui.painter().rect_filled(
                            volume_bar_bg,
                            Rounding::none(),
                            self.volume_bar_bg_color,
                        );

                        ui.painter().rect_filled(
                            volume_bar_fg,
                            Rounding::none(),
                            match dimmed {
                                true => self.volume_bar_fg_color_dimmed,
                                false => self.volume_bar_fg_color_normal,
                            },
                        );
                    },
                );
            });
    }

    // fn on_close_event(&mut self) -> bool {
    //     false
    // }

    fn clear_color(&self, _visuals: &Visuals) -> Rgba {
        Rgba::TRANSPARENT
    }
}
