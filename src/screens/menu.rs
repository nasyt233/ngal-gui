use crate::app::{NgalApp, Screen};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Stroke, StrokeKind, UiBuilder, Vec2};

#[derive(Clone, Copy)]
enum MenuAction {
    StartGame,
    LoadGame,
    Settings,
    Exit,
}

impl NgalApp {
    pub(crate) fn draw_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let opacity_alpha = (self.config.menu_opacity * 255.0).clamp(10.0, 255.0) as u8;

                // 1. 绘制背景及透明度遮罩
                if let Some(tex) = &self.background {
                    ui.painter().image(
                        tex.id(),
                        rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        Color32::from_black_alpha((self.config.menu_opacity * 180.0) as u8),
                    );
                } else {
                    ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(16, 18, 30));
                }

                // 辅助闭包：统一渲染标题与副标题
                let draw_title = |ui: &mut egui::Ui, title_size: f32| {
                    ui.label(
                        RichText::new(&self.game_config.title)
                            .size(title_size)
                            .strong()
                            .color(Color32::from_rgb(120, 220, 255)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("ngal视觉小说引擎")
                            .size(16.0)
                            .color(Color32::from_rgb(180, 200, 220)),
                    );
                };

                // 辅助闭包：统一渲染菜单按钮组
                let render_buttons = |ui: &mut egui::Ui, btn_w: f32, action: &mut Option<MenuAction>| {
                    let btn_size = egui::vec2(btn_w, 46.0);
                    if menu_button(ui, btn_size, "开始游戏") { *action = Some(MenuAction::StartGame); }
                    ui.add_space(10.0);
                    if menu_button(ui, btn_size, "读取存档") { *action = Some(MenuAction::LoadGame); }
                    ui.add_space(10.0);
                    if menu_button(ui, btn_size, "设置") { *action = Some(MenuAction::Settings); }
                    ui.add_space(10.0);
                    if menu_button(ui, btn_size, "退出") { *action = Some(MenuAction::Exit); }
                };

                let mut action: Option<MenuAction> = None;
                let layout = self.game_config.menu_layout;

                // 2. 根据布局方案渲染对应界面
                match layout {
                    // 方案 1: 经典居中
                    1 => {
                        let line_y = rect.top() + rect.height() * 0.18;
                        ui.painter().line_segment(
                            [
                                Pos2::new(rect.left() + rect.width() * 0.25, line_y),
                                Pos2::new(rect.right() - rect.width() * 0.25, line_y),
                            ],
                            Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(60, 120, 180, opacity_alpha)),
                        );

                        ui.vertical_centered(|ui| {
                            ui.add_space(rect.height() * 0.22);
                            draw_title(ui, 54.0);
                            ui.add_space(56.0);
                            render_buttons(ui, 240.0, &mut action);
                        });
                    },

                    // 方案 2: 左侧对齐
                    2 => {
                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(Rect::from_min_size(
                                Pos2::new(rect.left() + 90.0, rect.top() + 150.0),
                                Vec2::new(400.0, 500.0),
                            )),
                            |ui| {
                                draw_title(ui, 48.0);
                                ui.add_space(45.0);
                                render_buttons(ui, 240.0, &mut action);
                            },
                        );
                    },

                    // 方案 3: 右侧悬浮
                    3 => {
                        let panel_width = 380.0;
                        let panel_rect = Rect::from_min_max(
                            Pos2::new(rect.right() - panel_width, rect.top()),
                            rect.max,
                        );
                        ui.painter().rect_filled(panel_rect, 0.0, Color32::from_rgba_premultiplied(10, 14, 24, opacity_alpha));

                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(panel_rect.shrink(35.0)),
                            |ui| {
                                ui.add_space(100.0);
                                draw_title(ui, 38.0);
                                ui.add_space(60.0);
                                render_buttons(ui, 310.0, &mut action);
                            },
                        );
                    },

                    // 方案 4: 下方横向
                    4 => {
                        ui.vertical_centered(|ui| {
                            ui.add_space(rect.height() * 0.28);
                            draw_title(ui, 52.0);
                        });

                        let bar_height = 85.0;
                        let bar_rect = Rect::from_min_size(
                            Pos2::new(rect.left(), rect.bottom() - bar_height),
                            Vec2::new(rect.width(), bar_height),
                        );
                        ui.painter().rect_filled(bar_rect, 0.0, Color32::from_rgba_premultiplied(10, 14, 24, opacity_alpha));

                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(bar_rect.shrink(18.0)),
                            |ui| {
                                ui.horizontal(|ui| {
                                    let btn_w = 200.0;
                                    let total_w = 4.0 * btn_w;
                                    let spacing = (rect.width() - total_w).max(40.0) / 5.0;
                                    ui.add_space(spacing);
                                    render_buttons(ui, btn_w, &mut action);
                                });
                            },
                        );
                    },

                    // 方案 5 / 默认: 中央卡片
                    _ => {
                        let card_size = Vec2::new(520.0, 420.0);
                        let card_rect = Rect::from_center_size(rect.center(), card_size);
                        ui.painter().rect_filled(card_rect, 12.0, Color32::from_rgba_premultiplied(18, 22, 38, opacity_alpha));
                        ui.painter().rect_stroke(
                            card_rect,
                            12.0,
                            Stroke::new(1.5_f32, Color32::from_rgba_premultiplied(70, 130, 190, opacity_alpha)),
                            StrokeKind::Inside,
                        );

                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(card_rect.shrink(30.0)),
                            |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(10.0);
                                    draw_title(ui, 38.0);
                                    ui.add_space(35.0);
                                    render_buttons(ui, 240.0, &mut action);
                                });
                            },
                        );
                    }
                }

                // 3. 统一分发按钮触发的动作
                if let Some(act) = action {
                    match act {
                        MenuAction::StartGame => self.start_game(ctx),
                        MenuAction::LoadGame => {
                            self.last_screen = Screen::Menu;
                            self.screen = Screen::Load;
                            self.saveload_cache_dirty = true;
                        },
                        MenuAction::Settings => {
                            self.last_screen = Screen::Menu;
                            self.screen = Screen::Settings;
                        },
                        MenuAction::Exit => {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                }

                // 4. 版本号标记（自动读取 Cargo.toml 版本）
                let version_str = format!("v{}", env!("CARGO_PKG_VERSION"));
                ui.painter().text(
                    Pos2::new(rect.right() - 16.0, rect.bottom() - 12.0),
                    Align2::RIGHT_BOTTOM,
                    version_str,
                    FontId::proportional(12.0),
                    Color32::from_gray(160),
                );
            });
    }
}

fn menu_button(ui: &mut egui::Ui, size: Vec2, text: &str) -> bool {
    ui.add_sized(size, egui::Button::new(RichText::new(text).size(22.0)))
        .clicked()
}
