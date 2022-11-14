use crate::{
    comms::{UdpReceiver, UdpSender},
    config::Config,
    manager::Manager,
};
use eframe::{App, CreationContext};
use egui::{
    style::DebugOptions, text::LayoutJob, vec2, Align, CentralPanel, Context, Direction, FontData,
    FontDefinitions, FontFamily, FontId, Layout, Rect, Rgba, RichText, Rounding, Sense, Style,
    TextFormat, Ui, Vec2, Visuals,
};
use std::sync::Arc;

pub struct VolumeControlApp {
    manager: Arc<Manager<UdpSender, UdpReceiver>>,
    config: Config,
}

impl VolumeControlApp {
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
        cc.egui_ctx.set_style(style);

        Self {
            manager,
            config: Config::new(),
        }
    }

    fn draw_heading(&self, ui: &mut Ui) {
        let mut job = LayoutJob::default();
        job.append(
            "TotalMix ",
            0.0,
            TextFormat {
                font_id: FontId::proportional(self.config.theme.heading_font_size),
                color: self.config.theme.heading_totalmix_color,
                ..Default::default()
            },
        );
        job.append(
            "Volume",
            0.0,
            TextFormat {
                font_id: FontId::proportional(self.config.theme.heading_font_size),
                color: self.config.theme.heading_volume_color,
                ..Default::default()
            },
        );
        ui.label(job);
    }

    fn draw_volume_readout(&self, ui: &mut Ui, volume_db: String, dimmed: bool) {
        ui.label(
            RichText::new(volume_db)
                .size(self.config.theme.volume_readout_font_size)
                .color(if dimmed {
                    self.config.theme.volume_readout_color_dimmed
                } else {
                    self.config.theme.volume_readout_color_normal
                }),
        );
    }

    fn draw_volume_bar(&self, ui: &mut Ui, volume: f32, dimmed: bool) {
        // Add a little top padding to align with the text above which has a little
        // padding due to the font used.
        ui.add_space(self.config.theme.volume_bar_top_margin);

        // Ideas pinched from the implementation of ProgressBar.
        let (volume_bar_background, _response) = ui.allocate_exact_size(
            vec2(
                ui.available_width() - self.config.theme.volume_bar_horizontal_margin * 2.0,
                self.config.theme.volume_bar_height,
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
            self.config.theme.volume_bar_background_color,
        );

        ui.painter().rect_filled(
            volume_bar_foreground,
            Rounding::none(),
            if dimmed {
                self.config.theme.volume_bar_foreground_color_dimmed
            } else {
                self.config.theme.volume_bar_foreground_color_normal
            },
        );
    }
}

impl App for VolumeControlApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // frame.set_visible(false);

        CentralPanel::default()
            .frame(egui::Frame {
                rounding: Rounding::same(self.config.theme.background_rounding),
                fill: self.config.theme.background_color,
                ..Default::default()
            })
            .show(ctx, |ui| {
                let (volume_db, volume, dimmed) = {
                    let volume_db = self.manager.volume_db().unwrap_or_else(|| "-".to_string());
                    (volume_db, self.manager.volume(), self.manager.dimmed())
                };
                // ctx.request_repaint();

                // Draw the TotalMix Volume heading.
                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        self.config.theme.heading_and_volume_bar_height,
                    ),
                    Layout::centered_and_justified(Direction::TopDown).with_main_align(Align::Max),
                    |ui| {
                        self.draw_heading(ui);
                    },
                );

                // Draw the volume read-out in decibels.
                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        ui.available_height() - self.config.theme.heading_and_volume_bar_height,
                    ),
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| {
                        self.draw_volume_readout(ui, volume_db, dimmed);
                    },
                );

                // Draw the volume bar that indicates the current volume.
                ui.allocate_ui_with_layout(
                    vec2(
                        ui.available_width(),
                        self.config.theme.heading_and_volume_bar_height,
                    ),
                    Layout::centered_and_justified(Direction::TopDown).with_main_align(Align::Min),
                    |ui| {
                        self.draw_volume_bar(ui, volume, dimmed);
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
