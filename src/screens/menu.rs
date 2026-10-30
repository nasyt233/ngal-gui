use crate::app::{NgalApp, Screen};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, RichText, Stroke, Vec2};

impl NgalApp {
    pub(crate) fn draw_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                ui.painter()
                    .rect_filled(rect, 0.0, Color32::from_rgb(16, 18, 30));

                let line_y = rect.top() + rect.height() * 0.18;
                ui.painter().line_segment(
                    [
                        Pos2::new(rect.left() + rect.width() * 0.25, line_y),
                        Pos2::new(rect.right() - rect.width() * 0.25, line_y),
                    ],
                    Stroke::new(1.0_f32, Color32::from_rgb(60, 120, 180)),
                );

                ui.vertical_centered(|ui| {
                    ui.add_space(rect.height() * 0.22);
                    ui.label(
                        RichText::new("NGAL")
                            .size(64.0)
                            .strong()
                            .color(Color32::from_rgb(120, 220, 255)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("视觉小说引擎 · egui 版")
                            .size(20.0)
                            .color(Color32::from_rgb(180, 200, 220)),
                    );
                    ui.add_space(56.0);

                    let btn = egui::vec2(240.0, 46.0);
                    if menu_button(ui, btn, "开始游戏") {
                        self.start_game();
                    }
                    ui.add_space(10.0);
                    if menu_button(ui, btn, "设置") {
                        self.screen = Screen::Settings;
                    }
                    ui.add_space(10.0);
                    if menu_button(ui, btn, "退出") {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.painter().text(
                    Pos2::new(rect.right() - 16.0, rect.bottom() - 12.0),
                    Align2::RIGHT_BOTTOM,
                    "v0.1.0",
                    FontId::proportional(12.0),
                    Color32::from_gray(120),
                );
            });
    }
}

fn menu_button(ui: &mut egui::Ui, size: Vec2, text: &str) -> bool {
    ui.add_sized(size, egui::Button::new(RichText::new(text).size(22.0)))
        .clicked()
}