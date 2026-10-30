use crate::app::{NgalApp, Screen};
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
    // 1. 分支选择项
    // ==========================================
    if !app.parser.nodes[app.line_index].choices.is_empty() {
        let choices = app.parser.nodes[app.line_index].choices.clone();
        let box_height = (choices.len() as f32 * 46.0) + 70.0;
        let center_rect = Rect::from_center_size(screen.center(), Vec2::new(480.0, box_height));

        ui.painter().rect_filled(center_rect, 12.0, Color32::from_rgba_premultiplied(18, 22, 38, 235));
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

    // ==========================================
    // 2. 用户输入框
    // ==========================================
    if app.waiting_input {
        let center_rect = Rect::from_center_size(screen.center(), Vec2::new(440.0, 160.0));

        ui.painter().rect_filled(center_rect, 12.0, Color32::from_rgba_premultiplied(18, 22, 38, 235));
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
                    // 动态显示脚本中传入的提示文字（如 "请输入你的名字"）
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

    // ==========================================
    // 3. 常规对话框（底部区域）
    // ==========================================
    let box_height = screen.height() * 0.22;
    let margin = 20.0;
    let dialog_rect = Rect::from_min_max(
        Pos2::new(screen.left() + margin, screen.bottom() - box_height - margin),
        Pos2::new(screen.right() - margin, screen.bottom() - margin),
    );

    // 角色名字：位于对话框左上角外侧上方
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

    // 对话框内部内容
    ui.allocate_new_ui(
        UiBuilder::new().max_rect(dialog_rect.shrink(18.0)),
        |ui| {
            ui.horizontal(|ui| {
                // 左侧文本正文区域（左上对齐）
                let available_w = ui.available_width() - 380.0;
                ui.allocate_new_ui(
                    UiBuilder::new().max_rect(Rect::from_min_size(ui.cursor().min, Vec2::new(available_w.max(100.0), ui.available_height()))),
                    |ui| {
                        let visible_text = app.typewriter.visible_text();
                        ui.add(
                            egui::Label::new(
                                RichText::new(visible_text)
                                    .size(16.0)
                                    .color(Color32::from_rgb(240, 240, 240)),
                            )
                            .wrap(),
                        );
                    },
                );

                // 右上角快捷工具栏
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let btn_size = Vec2::new(54.0, 26.0);
                    
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new("设置").size(12.0))).clicked() {
                        app.screen = Screen::Settings;
                    }
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new("存档").size(12.0))).clicked() {
                        app.last_screen = Screen::Game;
                        app.screen = Screen::Save;
                    }
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new("读档").size(12.0))).clicked() {
                        app.last_screen = Screen::Game;
                        app.screen = Screen::Load;
                    }
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new("历史").size(12.0))).clicked() {
                        app.show_log = !app.show_log;
                    }
                    let skip_text = if app.is_skipping { "跳过[ON]" } else { "跳过" };
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new(skip_text).size(12.0))).clicked() {
                        app.is_skipping = !app.is_skipping;
                    }
                    let auto_text = if app.auto_play { "自动[ON]" } else { "自动" };
                    if ui.add_sized(btn_size, egui::Button::new(RichText::new(auto_text).size(12.0))).clicked() {
                        app.auto_play = !app.auto_play;
                    }
                });
            });
        },
    );

    interacted
}