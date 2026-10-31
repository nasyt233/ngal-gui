use crate::app::{NgalApp, Screen};
use crate::save::SaveData;
use crate::texture::load_texture;
use eframe::egui::{self, Color32, FontId, Pos2, Rect, RichText, Stroke, StrokeKind, Ui, UiBuilder, Vec2};

pub fn draw_dialog(ui: &mut Ui, screen: Rect, app: &mut NgalApp, ctx: &egui::Context) -> bool {
    if app.line_index >= app.parser.nodes.len() {
        app.screen = Screen::Menu;
        app.last_screen = Screen::Menu;
        let default_bg_path = app.game_config.get_background_path();
        app.background = load_texture(ctx, &default_bg_path);
        app.current_bg_path = Some(default_bg_path);
        return false;
    }

    let mut interacted = false;

    // ==========================================
    // 0. 如果开启了历史记录面板（Log / Backlog）
    // ==========================================
    if app.show_log {
        let log_rect = Rect::from_min_max(
            Pos2::new(screen.left() + 40.0, screen.top() + 40.0),
            Pos2::new(screen.right() - 40.0, screen.bottom() - 40.0),
        );

        // 半透明毛玻璃背景遮罩
        ui.painter().rect_filled(screen, 0.0, Color32::from_black_alpha(180));
        ui.painter().rect_filled(log_rect, 12.0, Color32::from_rgba_premultiplied(18, 22, 38, 245));
        ui.painter().rect_stroke(
            log_rect,
            12.0,
            Stroke::new(1.5_f32, Color32::from_rgb(70, 130, 180)),
            StrokeKind::Inside,
        );

        let mut close_log = false;
        let mut jump_target_index: Option<usize> = None;

        ui.allocate_new_ui(
            UiBuilder::new().max_rect(log_rect.shrink(24.0)),
            |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("对话历史 (Backlog) - 点击记录可直接跳转回溯").size(16.0).strong().color(Color32::from_rgb(255, 220, 90)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("✖ 关闭").size(14.0).color(Color32::WHITE)).clicked() {
                            close_log = true;
                        }
                    });
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // 滚动区域展示历史记录
                egui::ScrollArea::vertical().max_height(log_rect.height() - 80.0).show(ui, |ui| {
                    for (idx, (speaker, text)) in app.history.iter().enumerate() {
                        ui.horizontal(|ui| {
                            let speaker_text = if speaker.is_empty() { "旁白".to_string() } else { speaker.clone() };
                            
                            let item_text = format!("【{}】: {}", speaker_text, text);
                            if ui.add(egui::Button::new(RichText::new(item_text).size(14.0).color(Color32::from_rgb(220, 220, 220))).frame(false)).clicked() {
                                jump_target_index = Some(idx);
                            }
                        });
                        ui.add_space(6.0);
                    }
                });
            },
        );

        if close_log {
            app.show_log = false;
        }

        if let Some(history_idx) = jump_target_index {
            if history_idx < app.history.len() {
                let target_text = &app.history[history_idx].1;
                for (n_idx, node) in app.parser.nodes.iter().enumerate() {
                    if let Some(txt) = &node.text {
                        if txt == target_text {
                            app.line_index = n_idx;
                            app.execute_node(ctx);
                            app.show_log = false;
                            break;
                        }
                    }
                }
            }
        }

        return interacted;
    }

    // ==========================================
    // 1. 底层：始终渲染底部的常规对话框与右上角功能工具栏
    // ==========================================
    let box_height = screen.height() * 0.22;
    let margin = 20.0;
    let dialog_rect = Rect::from_min_max(
        Pos2::new(screen.left() + margin, screen.bottom() - box_height - margin),
        Pos2::new(screen.right() - margin, screen.bottom() - margin),
    );

    // 角色名字
    if !app.speaker.is_empty() {
        let font_id = FontId::proportional(16.0);
        let galley = ui.painter().layout_no_wrap(
            app.speaker.clone(),
            font_id,
            Color32::from_rgb(255, 220, 90),
        );
        let padding = 16.0;
        let name_size = Vec2::new(galley.size().x + padding * 2.0, galley.size().y + 10.0);
        let name_pos = Pos2::new(dialog_rect.left(), dialog_rect.top() - name_size.y - 6.0);
        let name_rect = Rect::from_min_size(name_pos, name_size);

        ui.painter().rect_filled(name_rect, 6.0, Color32::from_rgba_premultiplied(30, 40, 65, 230));
        ui.painter().rect_stroke(
            name_rect,
            6.0,
            Stroke::new(1.0_f32, Color32::from_rgb(70, 130, 180)),
            StrokeKind::Inside,
        );

        let text_pos = name_rect.min + Vec2::new(padding, 5.0);
        ui.painter().galley(text_pos, galley, Color32::from_rgb(255, 220, 90));
    }

    // 对话框背景与边框
    let alpha = (app.config.dialog_opacity * 255.0).clamp(0.0, 255.0) as u8;
    let bg_color = Color32::from_rgba_premultiplied(15, 18, 28, alpha);
    let border_color = Color32::from_rgba_premultiplied(70, 130, 180, alpha.max(100));

    ui.painter().rect_filled(dialog_rect, 10.0, bg_color);
    ui.painter().rect_stroke(
        dialog_rect,
        10.0,
        Stroke::new(1.5_f32, border_color),
        StrokeKind::Inside,
    );

    // 底部对话框内部内容区
    ui.allocate_new_ui(
        UiBuilder::new().max_rect(dialog_rect.shrink(18.0)),
        |ui| {
            ui.horizontal(|ui| {
                let available_w = ui.available_width() - 480.0;
                ui.allocate_new_ui(
                    UiBuilder::new().max_rect(Rect::from_min_size(ui.cursor().min, Vec2::new(available_w.max(100.0), ui.available_height()))),
                    |ui| {
                        // 在闭包内部按需获取当前节点信息，避免全局借用冲突
                        let current_node = &app.parser.nodes[app.line_index];
                        let display_text = if app.waiting_input {
                            app.input_prompt.clone()
                        } else if !current_node.choices.is_empty() {
                            current_node.text.clone().unwrap_or_else(|| "请选择分支".to_string())
                        } else {
                            app.typewriter.visible_text().to_string()
                        };

                        ui.add(
                            egui::Label::new(
                                RichText::new(display_text)
                                    .size(16.0)
                                    .color(Color32::from_rgb(240, 240, 240)),
                            )
                            .wrap(),
                        );
                    },
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let toolbar_btn = |ui: &mut Ui, label: &str, active: bool| -> bool {
                        let display_label = if active { format!("{} [ON]", label) } else { label.to_string() };
                        let font_id = FontId::proportional(13.0);
                        let galley = ui.painter().layout_no_wrap(display_label, font_id, Color32::WHITE);
                        let desired_size = galley.size() + Vec2::new(14.0, 6.0);
                        let (rect, resp) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                        let is_hovered = resp.hovered();
                        let text_color = if is_hovered || active { Color32::WHITE } else { Color32::from_rgb(200, 215, 235) };
                        let bg_color = if is_hovered { 
                            Color32::from_rgba_premultiplied(70, 130, 180, 80) 
                        } else if active { 
                            Color32::from_rgba_premultiplied(70, 130, 180, 50) 
                        } else { 
                            Color32::TRANSPARENT 
                        };

                        if bg_color != Color32::TRANSPARENT {
                            ui.painter().rect_filled(rect, 4.0, bg_color);
                        }
                        let text_pos = rect.min + (rect.size() - galley.size()) * 0.5;
                        ui.painter().galley(text_pos, galley, text_color);

                        resp.clicked()
                    };

                    if toolbar_btn(ui, "Auto", app.auto_play) {
                        app.auto_play = !app.auto_play;
                    }
                    ui.add_space(4.0);

                    if toolbar_btn(ui, "Skip", app.is_skipping) {
                        app.is_skipping = !app.is_skipping;
                    }
                    ui.add_space(4.0);

                    if toolbar_btn(ui, "History", false) {
                        app.show_log = !app.show_log;
                    }
                    ui.add_space(4.0);

                    if toolbar_btn(ui, "Load", false) {
                        app.last_screen = Screen::Game;
                        app.screen = Screen::Load;
                        app.saveload_cache_dirty = true;
                    }
                    ui.add_space(4.0);

                    if toolbar_btn(ui, "Q.L", false) {
                        if let Some(loaded) = SaveData::load_from_slot(99) {
                            app.load_save_data(loaded, ctx);
                            app.screen = Screen::Game;
                            app.last_screen = Screen::Game;
                        }
                    }
                    ui.add_space(4.0);

                    if toolbar_btn(ui, "Q.S", false) {
                        let new_save = app.create_save_data();
                        let _ = SaveData::save_to_slot(99, &new_save);
                    }
                    ui.add_space(4.0);

                    if toolbar_btn(ui, "Save", false) {
                        app.last_screen = Screen::Game;
                        app.screen = Screen::Save;
                        app.saveload_cache_dirty = true;
                    }
                    ui.add_space(4.0);

                    if toolbar_btn(ui, "Settings", false) {
                        app.screen = Screen::Settings;
                    }
                });
            });
        },
    );

    // ==========================================
    // 2. Choose 选择支或 Input 输入状态
    // ==========================================
    let has_choices = !app.parser.nodes[app.line_index].choices.is_empty();
    if has_choices {
        let choices = app.parser.nodes[app.line_index].choices.clone();
        let box_height = (choices.len() as f32 * 46.0) + 70.0;
        let center_rect = Rect::from_center_size(screen.center(), Vec2::new(480.0, box_height));

        ui.painter().rect_filled(screen, 0.0, Color32::from_black_alpha(100));

        ui.painter().rect_filled(center_rect, 12.0, Color32::from_rgba_premultiplied(18, 22, 38, 245));
        ui.painter().rect_stroke(
            center_rect,
            12.0,
            Stroke::new(1.5_f32, Color32::from_rgb(70, 130, 180)),
            StrokeKind::Inside,
        );

        ui.allocate_new_ui(
            UiBuilder::new().max_rect(center_rect.shrink(20.0)),
            |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("请做出你的选择").size(18.0).strong().color(Color32::from_rgb(255, 220, 90)));
                    ui.add_space(14.0);
                    for (text, target) in choices {
                        let btn = ui.add_sized(
                            Vec2::new(400.0, 38.0),
                            egui::Button::new(RichText::new(&text).size(16.0).color(Color32::WHITE)),
                        );
                        if btn.clicked() {
                            interacted = true;
                            app.jump_to_label(&target, ctx);
                        }
                        ui.add_space(6.0);
                    }
                });
            },
        );
        return interacted;
    }

    if app.waiting_input {
        let center_rect = Rect::from_center_size(screen.center(), Vec2::new(440.0, 160.0));

        ui.painter().rect_filled(screen, 0.0, Color32::from_black_alpha(100));

        ui.painter().rect_filled(center_rect, 12.0, Color32::from_rgba_premultiplied(18, 22, 38, 245));
        ui.painter().rect_stroke(
            center_rect,
            12.0,
            Stroke::new(1.5_f32, Color32::from_rgb(70, 130, 180)),
            StrokeKind::Inside,
        );

        ui.allocate_new_ui(
            UiBuilder::new().max_rect(center_rect.shrink(20.0)),
            |ui| {
                ui.vertical_centered(|ui| {
                    let prompt_title = if app.input_prompt.is_empty() { "请输入内容" } else { &app.input_prompt };
                    ui.label(RichText::new(prompt_title).size(18.0).strong().color(Color32::from_rgb(255, 220, 90)));
                    ui.add_space(16.0);
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut app.input_buffer)
                            .desired_width(360.0)
                            .font(FontId::proportional(16.0)),
                    );
                    ui.add_space(16.0);
                    if ui.add_sized(Vec2::new(120.0, 32.0), egui::Button::new(RichText::new("确定").size(15.0))).clicked() 
                        || (resp.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))) 
                    {
                        if !app.input_buffer.is_empty() {
                            app.parser.variables.insert(app.input_var_name.clone(), app.input_buffer.clone());
                        }
                        app.waiting_input = false;
                        app.advance(ctx);
                        interacted = true;
                    }
                });
            },
        );
        return interacted;
    }
    interacted
}
