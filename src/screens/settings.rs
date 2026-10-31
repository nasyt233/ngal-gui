use crate::app::{NgalApp, Screen};
use crate::texture::load_texture;
use eframe::egui::{self, Color32, Pos2, Rect, RichText, Stroke, StrokeKind, UiBuilder, Vec2};

impl NgalApp {
    pub(crate) fn draw_settings(&mut self, ctx: &egui::Context) {
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
                ui.painter().rect_filled(rect, 0.0, Color32::from_black_alpha(185));
            } else {
                ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(20, 24, 38));
            }

            // 2. 顶栏
            ui.allocate_new_ui(
                UiBuilder::new().max_rect(Rect::from_min_size(rect.min + Vec2::new(30.0, 20.0), Vec2::new(rect.width() - 60.0, 50.0))),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("游戏设置 (Settings)").size(28.0).strong().color(Color32::from_rgb(130, 220, 255)));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add_sized(Vec2::new(100.0, 36.0), egui::Button::new(RichText::new("保存并返回").size(16.0))).clicked() {
                                self.config.save(&self.config_path);
                                if self.last_screen == Screen::Menu {
                                    self.background = load_texture(ctx, &self.game_config.get_background_path());
                                    self.screen = Screen::Menu;
                                } else {
                                    self.screen = Screen::Game;
                                }
                            }
                        });
                    });
                },
            );

            // 3. 设置面板卡片
            let panel_w = 640.0_f32.min(rect.width() - 60.0);
            let panel_h = 500.0_f32.min(rect.height() - 100.0);
            let panel_rect = Rect::from_center_size(rect.center() + Vec2::new(0.0, 15.0), Vec2::new(panel_w, panel_h));

            ui.painter().rect_filled(panel_rect, 12.0, Color32::from_rgba_premultiplied(25, 30, 48, 235));
            ui.painter().rect_stroke(
                panel_rect,
                12.0,
                Stroke::new(1.5_f32, Color32::from_rgb(70, 130, 180)),
                StrokeKind::Inside,
            );

            ui.allocate_new_ui(
                UiBuilder::new().max_rect(panel_rect.shrink(25.0)),
                |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_space(5.0);
                        let row_height = 42.0;

                        draw_setting_row(ui, "全屏模式 (Fullscreen)", row_height, |ui| {
                            if ui.checkbox(&mut self.config.fullscreen, "").changed() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.config.fullscreen));
                            }
                        });

                        draw_setting_row(ui, "窗口宽度 (Width)", row_height, |ui| {
                            ui.add(egui::DragValue::new(&mut self.config.window_width).range(800..=3840).speed(10.0));
                        });

                        draw_setting_row(ui, "窗口高度 (Height)", row_height, |ui| {
                            ui.add(egui::DragValue::new(&mut self.config.window_height).range(600..=2160).speed(10.0));
                        });

                        draw_setting_row(ui, "文字速度 (Text Speed)", row_height, |ui| {
                            ui.add(egui::Slider::new(&mut self.config.text_speed, 5.0..=100.0).text("CPS"));
                        });

                        draw_setting_row(ui, "BGM 音量 (BGM Volume)", row_height, |ui| {
                            ui.add(egui::Slider::new(&mut self.config.bgm_volume, 0.0..=1.0).text("音量"));
                        });

                        draw_setting_row(ui, "语音音量 (Voice Volume)", row_height, |ui| {
                            ui.add(egui::Slider::new(&mut self.config.voice_volume, 0.0..=1.0).text("音量"));
                        });

                        draw_setting_row(ui, "对话框透明度 (Dialog Opacity)", row_height, |ui| {
                            ui.add(egui::Slider::new(&mut self.config.dialog_opacity, 0.1..=1.0).text("透明度"));
                        });
                    });
                },
            );
        });
    }
}

// 使用普通函数代替宏
fn draw_setting_row(ui: &mut egui::Ui, label: &str, row_height: f32, widget: impl FnOnce(&mut egui::Ui)) {
    let row_rect = ui.allocate_space(Vec2::new(ui.available_width(), row_height)).1;
    ui.allocate_new_ui(UiBuilder::new().max_rect(row_rect), |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).size(15.0).color(Color32::WHITE));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                widget(ui);
            });
        });
    });
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);
}
