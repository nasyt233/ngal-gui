use crate::app::{NgalApp, Screen};
use eframe::egui::{self, Color32, RichText};

impl NgalApp {
    pub(crate) fn draw_settings(&mut self, ctx: &egui::Context) {
        let mut changed = false;
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                ui.painter()
                    .rect_filled(rect, 0.0, Color32::from_rgb(16, 18, 30));

                ui.vertical_centered(|ui| {
                    ui.add_space(rect.height() * 0.18);
                    ui.label(
                        RichText::new("设置")
                            .size(40.0)
                            .strong()
                            .color(Color32::from_rgb(120, 220, 255)),
                    );
                    ui.add_space(40.0);

                    if ui
                        .add_sized(
                            egui::vec2(360.0, 24.0),
                            egui::Slider::new(&mut self.config.text_speed, 5.0..=80.0)
                                .text("文字速度（字/秒）"),
                        )
                        .changed()
                    {
                        changed = true;
                    }
                    ui.add_space(14.0);
                    if ui
                        .add_sized(
                            egui::vec2(360.0, 24.0),
                            egui::Slider::new(&mut self.config.bgm_volume, 0.0..=1.0)
                                .text("BGM 音量"),
                        )
                        .changed()
                    {
                        changed = true;
                    }
                    ui.add_space(14.0);
                    if ui
                        .add_sized(
                            egui::vec2(360.0, 24.0),
                            egui::Slider::new(&mut self.config.voice_volume, 0.0..=1.0)
                                .text("语音音量"),
                        )
                        .changed()
                    {
                        changed = true;
                    }

                    ui.add_space(36.0);
                    if ui
                        .add_sized(
                            egui::vec2(200.0, 42.0),
                            egui::Button::new(RichText::new("返回").size(20.0)),
                        )
                        .clicked()
                    {
                        self.screen = Screen::Menu;
                    }
                });

                if changed {
                    self.config.save(&self.config_path);
                }
            });
    }
}