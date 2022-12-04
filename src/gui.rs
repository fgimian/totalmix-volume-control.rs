use std::sync::Arc;

use egui::{
    style::DebugOptions, text::LayoutJob, vec2, Align, CentralPanel, Context, Direction, FontData,
    FontDefinitions, FontFamily, FontId, Frame, Id, Layout, Rect, RichText, Rounding, Sense, Style,
    TextFormat, Ui, Vec2,
};

use crate::{
    colors::ToColor32,
    comms::{UdpReceiver, UdpSender},
    config::Config,
    manager::Manager,
};

pub struct VolumeControlApp {
    manager: Arc<Manager<UdpSender, UdpReceiver>>,
    config: Arc<Config>,
    id: Id,
    show_time: Option<f64>,
}

impl VolumeControlApp {
    pub fn new(
        egui_ctx: &Context,
        manager: Arc<Manager<UdpSender, UdpReceiver>>,
        config: Arc<Config>,
    ) -> Self {
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
        egui_ctx.set_fonts(fonts);

        let mut style = Style::default();
        style.spacing.item_spacing = Vec2::ZERO;
        if cfg!(debug_assertions) {
            style.debug = DebugOptions {
                debug_on_hover: true,
                show_expand_width: true,
                show_expand_height: true,
                show_resize: true,
            };
        }
        egui_ctx.set_style(style);

        Self {
            manager,
            config,
            id: Id::new("app"),
            show_time: None,
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, restart: bool) {
        let opacity = {
            // A global hotkey has been pressed so display the UI.
            if restart {
                self.show_time = Some(egui_ctx.input().time);
                egui_ctx.clear_animations();
                egui_ctx.animate_value_with_time(self.id, 1.0, 0.0);
                egui_ctx.request_repaint();
                1.0
            // The UI is currently visible.
            } else if let Some(show_time) = self.show_time {
                if egui_ctx.input().time - show_time >= self.config.interface.hide_delay {
                    self.show_time = None;
                }
                egui_ctx.request_repaint();
                1.0
            // The hide delay has completed and we need to animate the fade out.
            } else {
                egui_ctx.animate_value_with_time(self.id, 0.0, self.config.interface.fade_out_time)
            }
        };

        CentralPanel::default()
            .frame(Frame {
                rounding: Rounding::same(
                    self.config.theme.background_rounding * self.config.interface.scaling,
                ),
                fill: self
                    .config
                    .theme
                    .background_color
                    .to_colour32_scaled(opacity),
                ..Default::default()
            })
            .show(egui_ctx, |ui| {
                let volume_db = self.manager.volume_db().unwrap_or_else(|| "-".to_string());
                let volume = self.manager.volume();
                let dimmed = self.manager.dimmed();
                let scaling = self.config.interface.scaling;

                // Draw the TotalMix Volume heading.
                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        self.config.theme.heading_and_volume_bar_height * scaling,
                    ),
                    Layout::centered_and_justified(Direction::TopDown).with_main_align(Align::Max),
                    |ui| {
                        self.draw_heading(ui, opacity, scaling);
                    },
                );

                // Draw the volume read-out in decibels.
                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        ui.available_height()
                            - self.config.theme.heading_and_volume_bar_height * scaling,
                    ),
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| {
                        self.draw_volume_readout(ui, opacity, scaling, &volume_db, dimmed);
                    },
                );

                // Draw the volume bar that indicates the current volume.
                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        self.config.theme.heading_and_volume_bar_height * scaling,
                    ),
                    Layout::centered_and_justified(Direction::TopDown).with_main_align(Align::Min),
                    |ui| {
                        self.draw_volume_bar(ui, opacity, scaling, volume, dimmed);
                    },
                );
            });
    }

    fn draw_heading(&self, ui: &mut Ui, opacity: f32, scaling: f32) {
        let mut job = LayoutJob::default();
        job.append(
            "TotalMix ",
            0.0,
            TextFormat {
                font_id: FontId::proportional(self.config.theme.heading_font_size * scaling),
                color: self
                    .config
                    .theme
                    .heading_totalmix_color
                    .to_colour32_scaled(opacity),
                ..Default::default()
            },
        );
        job.append(
            "Volume",
            0.0,
            TextFormat {
                font_id: FontId::proportional(self.config.theme.heading_font_size * scaling),
                color: self
                    .config
                    .theme
                    .heading_volume_color
                    .to_colour32_scaled(opacity),
                ..Default::default()
            },
        );
        ui.label(job);
    }

    fn draw_volume_readout(
        &self,
        ui: &mut Ui,
        opacity: f32,
        scaling: f32,
        volume_db: &str,
        dimmed: bool,
    ) {
        let volume_readout_color = if dimmed {
            self.config.theme.volume_readout_color_dimmed
        } else {
            self.config.theme.volume_readout_color_normal
        };
        ui.label(
            RichText::new(volume_db)
                .size(self.config.theme.volume_readout_font_size * scaling)
                .color(volume_readout_color.to_colour32_scaled(opacity)),
        );
    }

    fn draw_volume_bar(&self, ui: &mut Ui, opacity: f32, scaling: f32, volume: f32, dimmed: bool) {
        // Add a little top padding to align with the text above which has a little
        // padding due to the font used.
        ui.add_space(self.config.theme.volume_bar_top_margin * scaling);

        // Ideas pinched from the implementation of ProgressBar.
        let (volume_bar_background, _response) = ui.allocate_exact_size(
            vec2(
                ui.available_width()
                    - self.config.theme.volume_bar_horizontal_margin * 2.0 * scaling,
                self.config.theme.volume_bar_height * scaling,
            ),
            Sense::hover(),
        );

        ui.painter().rect_filled(
            volume_bar_background,
            Rounding::none(),
            self.config
                .theme
                .volume_bar_background_color
                .to_colour32_scaled(opacity),
        );

        let volume_bar_foreground = Rect::from_min_size(
            volume_bar_background.min,
            vec2(
                volume_bar_background.width() * volume,
                volume_bar_background.height(),
            ),
        );
        let volume_bar_foreground_color = if dimmed {
            self.config.theme.volume_bar_foreground_color_dimmed
        } else {
            self.config.theme.volume_bar_foreground_color_normal
        };
        ui.painter().rect_filled(
            volume_bar_foreground,
            Rounding::none(),
            volume_bar_foreground_color.to_colour32_scaled(opacity),
        );
    }
}
