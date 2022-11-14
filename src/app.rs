use crate::{
    comms::{UdpReceiver, UdpSender},
    manager::Manager,
};
use eframe::{App, CreationContext};
use egui::{
    hex_color, style::DebugOptions, text::LayoutJob, vec2, Align, CentralPanel, Color32, Context,
    Direction, FontData, FontDefinitions, FontFamily, FontId, Layout, Rect, Rgba, RichText,
    Rounding, Sense, TextFormat, Vec2, Visuals,
};
use std::sync::Arc;

pub struct TotalMixVolumeControl {
    manager: Arc<Manager<UdpSender, UdpReceiver>>,
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
    volume_bar_background_color: Color32,
    volume_bar_foreground_color_normal: Color32,
    volume_bar_foreground_color_dimmed: Color32,
}

impl TotalMixVolumeControl {
    pub fn new(cc: &CreationContext<'_>, manager: Arc<Manager<UdpSender, UdpReceiver>>) -> Self {
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
            manager,
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
                    let volume_db = self.manager.volume_db().unwrap_or_else(|| "-".to_string());
                    (volume_db, self.manager.volume(), self.manager.dimmed())
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
                                .color(if dimmed {
                                    self.volume_readout_color_dimmed
                                } else {
                                    self.volume_readout_color_normal
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
                        let (volume_bar_background, _response) = ui.allocate_exact_size(
                            vec2(
                                ui.available_width() - self.volume_bar_horizontal_margin * 2.0,
                                self.volume_bar_height,
                            ),
                            Sense::hover(),
                        );

                        let volume_bar_foreground = Rect::from_min_size(
                            volume_bar_background.min,
                            vec2(
                                volume_bar_background.width() * volume,
                                volume_bar_background.height(),
                            ),
                        );

                        ui.painter().rect_filled(
                            volume_bar_background,
                            Rounding::none(),
                            self.volume_bar_background_color,
                        );

                        ui.painter().rect_filled(
                            volume_bar_foreground,
                            Rounding::none(),
                            if dimmed {
                                self.volume_bar_foreground_color_dimmed
                            } else {
                                self.volume_bar_foreground_color_normal
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
