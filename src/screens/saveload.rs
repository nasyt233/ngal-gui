use crate::app::{NgalApp, Screen};
use crate::save::SaveData;
use crate::texture::load_texture;
use eframe::egui::{self, Color32, FontId, Pos2, Rect, RichText, Stroke, Vec2};

impl NgalApp {
    pub(crate) fn draw_saveload(&mut self, ctx: &egui::Context, is_save: bool) {
        let title = if is_save { "保存游戏 (Save Game)" } else { "读取游戏 (Load Game)" };

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();

            // 1. 绘制背景
            if let Some(tex) = &self.background {
                ui.painter().image(
                    tex.id(),
                    rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
                ui.painter().rect_filled(rect, 0.0, Color32::from_black_alpha(200));
            } else {
                ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(16, 18, 30));
            }

            // 2. 顶栏 (标题与返回按钮)
            ui.allocate_new_ui(
                egui::UiBuilder::new().max_rect(Rect::from_min_size(rect.min + Vec2::new(30.0, 20.0), Vec2::new(rect.width() - 60.0, 50.0))),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(title).size(28.0).strong().color(Color32::from_rgb(130, 220, 255)));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let back_btn = ui.add_sized(Vec2::new(100.0, 36.0), egui::Button::new(RichText::new("返回").size(16.0)));
                            if back_btn.clicked() {
                                self.screen = self.last_screen;
                            }
                        });
                    });
                },
            );

            // 3. 存档槽位卡片网格滚动区 (16个普通槽 + 1个快速存档)
            let content_rect = Rect::from_min_max(
                rect.min + Vec2::new(30.0, 85.0),
                rect.max - Vec2::new(30.0, 25.0),
            );

            ui.allocate_new_ui(
                egui::UiBuilder::new().max_rect(content_rect),
                |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_space(5.0);
                        let mut slots: Vec<usize> = (1..=16).collect();
                        slots.push(99); // 快速存档槽位

                        let available_w = ui.available_width();
                        let card_w = (available_w - 15.0).max(400.0) / 2.0;

                        for chunk in slots.chunks(2) {
                            ui.horizontal(|ui| {
                                for &slot in chunk {
                                    let slot_name = if slot == 99 { "快速存档 (Quick Save)".to_string() } else { format!("存档槽位 {:02}", slot) };
                                    let save_data = SaveData::load_from_slot(slot);

                                    // 使用 egui::Frame::NONE、corner_radius 及带 _f32 的 Stroke
                                    egui::Frame::NONE
                                        .fill(Color32::from_rgba_premultiplied(24, 30, 45, 235))
                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(60, 100, 150)))
                                        .corner_radius(8.0)
                                        .inner_margin(12.0)
                                        .show(ui, |ui| {
                                            ui.set_width(card_w - 24.0);
                                            ui.set_height(115.0);

                                            ui.horizontal(|ui| {
                                                // 左侧：缩略图预览框
                                                let thumb_size = Vec2::new(120.0, 95.0);
                                                let (thumb_rect, _) = ui.allocate_exact_size(thumb_size, egui::Sense::hover());
                                                
                                                if let Some(ref data) = save_data {
                                                    if let Some(ref bg_path) = data.background_path {
                                                        if let Some(tex) = load_texture(ctx, bg_path) {
                                                            ui.painter().image(
                                                                tex.id(),
                                                                thumb_rect,
                                                                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                                                                Color32::WHITE,
                                                            );
                                                        } else {
                                                            ui.painter().rect_filled(thumb_rect, 4.0, Color32::from_rgb(12, 15, 22));
                                                        }
                                                    } else {
                                                        ui.painter().rect_filled(thumb_rect, 4.0, Color32::from_rgb(12, 15, 22));
                                                    }
                                                } else {
                                                    ui.painter().rect_filled(thumb_rect, 4.0, Color32::from_rgb(12, 15, 22));
                                                    ui.painter().text(
                                                        thumb_rect.center(),
                                                        egui::Align2::CENTER_CENTER,
                                                        "空槽位",
                                                        FontId::proportional(14.0),
                                                        Color32::from_gray(130),
                                                    );
                                                }

                                                ui.add_space(12.0);

                                                // 右侧：存档信息与操作按钮
                                                ui.vertical(|ui| {
                                                    ui.label(RichText::new(slot_name).strong().size(15.0).color(Color32::from_rgb(255, 220, 100)));
                                                    ui.add_space(2.0);

                                                    if let Some(ref data) = save_data {
                                                        ui.label(RichText::new(&data.timestamp).size(11.0).color(Color32::from_gray(160)));
                                                        ui.add_space(4.0);
                                                        let preview = if data.preview_text.chars().count() > 16 {
                                                            format!("{}...", data.preview_text.chars().take(16).collect::<String>())
                                                        } else {
                                                            data.preview_text.clone()
                                                        };
                                                        ui.label(RichText::new(preview).size(13.0).color(Color32::WHITE));
                                                    } else {
                                                        ui.add_space(10.0);
                                                        ui.label(RichText::new("暂无存档数据").size(12.0).color(Color32::from_gray(110)));
                                                    }

                                                    ui.add_space(8.0);
                                                    ui.horizontal(|ui| {
                                                        let btn_label = if is_save { "保存" } else { "读取" };
                                                        if ui.add_sized(Vec2::new(60.0, 24.0), egui::Button::new(btn_label)).clicked() {
                                                            if is_save {
                                                                let new_save = self.create_save_data();
                                                                let _ = SaveData::save_to_slot(slot, &new_save);
                                                            } else {
                                                                if let Some(loaded) = SaveData::load_from_slot(slot) {
                                                                    self.load_save_data(loaded, ctx);
                                                                    self.screen = Screen::Game;
                                                                    self.last_screen = Screen::Game;
                                                                }
                                                            }
                                                        }

                                                        if save_data.is_some() && ui.add_sized(Vec2::new(50.0, 24.0), egui::Button::new("删除")).clicked() {
                                                            let _ = std::fs::remove_file(format!("save/slot_{}.json", slot));
                                                        }
                                                    });
                                                });
                                            });
                                        });

                                    ui.add_space(15.0);
                                }
                            });
                            ui.add_space(12.0);
                        }
                    });
                },
            );
        });
    }
}
