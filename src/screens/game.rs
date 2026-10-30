use crate::app::{NgalApp, Screen};
use crate::texture::load_texture;
use crate::save::SaveData;
use crate::ui::dialog::draw_dialog;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText};

impl NgalApp {
    pub(crate) fn draw_game(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.background = load_texture(ctx, &self.game_config.get_background_path());
            self.screen = Screen::Menu;
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.show_log = !self.show_log;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::H)) {
            self.hide_ui = !self.hide_ui;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::F5)) {
            let save_data = self.create_save_data();
            let _ = SaveData::save_to_slot(99, &save_data);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F9)) {
            if let Some(save_data) = SaveData::load_from_slot(99) {
                self.load_save_data(save_data, ctx);
            }
        }

        if self.sleep_timer > 0.0 {
            self.sleep_timer -= ctx.input(|i| i.stable_dt);
            ctx.request_repaint();
            if self.sleep_timer <= 0.0 {
                self.sleep_timer = 0.0;
                self.advance(ctx);
            }
        }

        if self.is_skipping {
            self.typewriter.chars_per_sec = self.config.text_speed * 5.0;
        } else {
            self.typewriter.chars_per_sec = self.config.text_speed;
        }
        self.typewriter.update(ctx);

        if self.auto_play && self.typewriter.is_done() && !self.waiting_input && self.sleep_timer <= 0.0 && !self.hide_ui {
            self.auto_timer += ctx.input(|i| i.stable_dt);
            if self.auto_timer > 2.0 {
                self.auto_timer = 0.0;
                self.advance(ctx);
            }
            ctx.request_repaint();
        }

        let mut ui_interacted = false;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let screen = ui.max_rect();

                // 1. 背景层
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

                // 2. 立绘层（优化：等比缩放 + 底部精准锚定在对话框上方）
                if let Some(tex) = &self.portrait {
                    let tex_size = tex.size_vec2();
                    // 核心优化：以屏幕高度的 80% 为基准进行等比缩放，避免巨幅图片溢出
                    let target_height = screen.height() * 0.80 * self.portrait_scale;
                    let aspect = if tex_size.y > 0.0 { tex_size.x / tex_size.y } else { 0.5 };
                    let scaled_height = target_height;
                    let scaled_width = target_height * aspect;

                    let ratio_x = match self.portrait_align {
                        1 => 0.25,
                        3 => 0.75,
                        _ => 0.5,
                    };
                    let center_x = screen.left() + screen.width() * ratio_x;

                    // 如果显示对话框，立绘底部对齐对话框上方；隐藏UI时对齐屏幕底端
                    let bottom_y = if !self.hide_ui {
                        let dialog_box_height = screen.height() * 0.22;
                        screen.bottom() - dialog_box_height - 15.0
                    } else {
                        screen.bottom()
                    };

                    let min_x = center_x - scaled_width / 2.0;
                    let max_x = center_x + scaled_width / 2.0;
                    let min_y = bottom_y - scaled_height;
                    let max_y = bottom_y;

                    let portrait_rect = Rect::from_min_max(Pos2::new(min_x, min_y), Pos2::new(max_x, max_y));

                    ui.painter().image(
                        tex.id(),
                        portrait_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }

                // 3. 对话框与UI层
                if !self.hide_ui {
                    ui_interacted = draw_dialog(ui, screen, self, ctx);

                    ui.painter().text(
                        Pos2::new(screen.right() - 16.0, screen.bottom() - 10.0),
                        Align2::RIGHT_BOTTOM,
                        "Esc 菜单 | H 隐藏UI | L 历史 | F5/F9 快存取",
                        FontId::proportional(14.0),
                        Color32::from_gray(160),
                    );
                } else {
                    ui.painter().text(
                        Pos2::new(screen.center().x, screen.bottom() - 20.0),
                        Align2::CENTER_BOTTOM,
                        "【已进入纯净模式，按 H 键恢复对话框】",
                        FontId::proportional(14.0),
                        Color32::from_white_alpha(180),
                    );
                }
            });

        if self.show_log {
            egui::Window::new("对话历史 Log")
                .collapsible(false)
                .resizable(true)
                .default_size([600.0, 400.0])
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (speaker, text) in &self.history {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!("{}: ", speaker))
                                        .strong()
                                        .color(Color32::from_rgb(255, 220, 120)),
                                );
                                ui.label(text);
                            });
                            ui.separator();
                        }
                    });
                    if ui.button("关闭").clicked() {
                        self.show_log = false;
                    }
                });
        }

        let advance_input = ctx.input(|i| i.key_pressed(egui::Key::Enter) || i.pointer.primary_clicked());
        if advance_input && !ui_interacted && !self.show_log && !self.waiting_input && self.sleep_timer <= 0.0 {
            if self.hide_ui {
                self.hide_ui = false;
            } else {
                self.advance(ctx);
            }
            ctx.request_repaint();
        }
    }
}
