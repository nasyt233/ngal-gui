use crate::app::{NgalApp, Screen};
use crate::ui::dialog::draw_dialog;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect};

impl NgalApp {
    pub(crate) fn draw_game(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.screen = Screen::Menu;
            return;
        }

        self.typewriter.chars_per_sec = self.config.text_speed;
        self.typewriter.update(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let screen = ui.max_rect();

                // 背景层
                if let Some(tex) = &self.background {
                    ui.painter().image(
                        tex.id(),
                        screen,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                } else {
                    ui.painter()
                        .rect_filled(screen, 0.0, Color32::from_rgb(18, 18, 28));
                }

                // 立绘层
                if let Some(tex) = &self.portrait {
                    let size = tex.size_vec2() * self.portrait_scale;
                    let center = Pos2::new(
                        screen.left() + screen.width() * self.portrait_pos.x,
                        screen.top() + screen.height() * self.portrait_pos.y,
                    );
                    let rect = Rect::from_center_size(center, size);
                    ui.painter().image(
                        tex.id(),
                        rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }

                // 对话框层
                draw_dialog(ui, screen, &self.speaker, &self.typewriter);

                ui.painter().text(
                    Pos2::new(screen.right() - 16.0, screen.bottom() - 10.0),
                    Align2::RIGHT_BOTTOM,
                    "Esc 返回菜单",
                    FontId::proportional(14.0),
                    Color32::from_gray(160),
                );
            });

        let advance =
            ctx.input(|i| i.key_pressed(egui::Key::Enter) || i.pointer.primary_clicked());
        if advance {
            self.advance();
            ctx.request_repaint();
        }
    }
}