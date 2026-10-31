use crate::app::{NgalApp, Screen};
use crate::save::SaveData;
use crate::texture::load_texture;
use eframe::egui::{self, Color32, FontId, Pos2, Rect, RichText, Stroke, Vec2};

impl NgalApp {
    pub(crate) fn draw_saveload(&mut self, ctx: &egui::Context, is_save: bool) {
        let title = if is_save { "保存游戏 (Save Game)" } else { "读取游戏 (Load Game)" };

        if self.saveload_cache_dirty {
            self.save_slots_cache.clear();
            self.save_thumb_cache.clear();
            
            let mut slots = vec![99];
            slots.extend(1..=16);

            for slot in slots {
                let data = SaveData::load_from_slot(slot);
                self.save_slots_cache.insert(slot, data);
                // 初始时将图片纹理设为 None，采用按需懒加载
                self.save_thumb_cache.insert(slot, None);
            }
            self.saveload_cache_dirty = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();

            // 纯色填充背景，零性能开销
            ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(14, 16, 26));

            // 顶栏 (标题与返回按钮)
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

            // 存档槽位网格渲染
            let content_rect = Rect::from_min_max(
                rect.min + Vec2::new(30.0, 85.0),
                rect.max - Vec2::new(30.0, 25.0),
            );

            ui.allocate_new_ui(
                egui::UiBuilder::new().max_rect(content_rect),
                |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_space(5.0);
                        
                        // 顺序保持：99 在前，1~16 在后
                        let mut slots = vec![99];
                        slots.extend(1..=16);

                        let available_w = ui.available_width();
                        let card_w = (available_w - 15.0).max(400.0) / 2.0;

                        for chunk in slots.chunks(2) {
                            ui.horizontal(|ui| {
                                for &slot in chunk {
                                    let slot_name = if slot == 99 { "快速存档 (Quick Save)".to_string() } else { format!("存档槽位 {:02}", slot) };
                                    
                                    let save_data = self.save_slots_cache.get(&slot).cloned().flatten();
                                    
                                    let thumb_tex = match self.save_thumb_cache.get(&slot) {
                                        Some(Some(tex)) => Some(tex.clone()),
                                        Some(None) => {
                                            // 尚未加载过，尝试按需加载
                                            if let Some(ref data) = save_data {
                                                if let Some(ref bg_path) = data.background_path {
                                                    let loaded = load_texture(ctx, bg_path);
                                                    self.save_thumb_cache.insert(slot, loaded.clone());
                                                    loaded
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                        None => None,
                                    };

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
                                                
                                                if let Some(tex) = thumb_tex {
                                                    ui.painter().image(
                                                        tex.id(),
                                                        thumb_rect,
                                                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                                                        Color32::WHITE,
                                                    );
                                                } else {
                                                    ui.painter().rect_filled(thumb_rect, 4.0, Color32::from_rgb(12, 15, 22));
                                                    let tip_text = if save_data.is_some() { "无预览图" } else { "空槽位" };
                                                    ui.painter().text(
                                                        thumb_rect.center(),
                                                        egui::Align2::CENTER_CENTER,
                                                        tip_text,
                                                        FontId::proportional(12.0),
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
                                                                self.saveload_cache_dirty = true;
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
                                                            self.saveload_cache_dirty = true;
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
