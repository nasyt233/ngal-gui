use crate::app::{NgalApp, Screen};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Stroke, StrokeKind, UiBuilder, Vec2};

impl NgalApp {
    pub(crate) fn draw_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let rect = ui.max_rect();

                // 1. 绘制背景图片或默认暗色背景
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
                        Color32::from_black_alpha(150),
                    );
                } else {
                    ui.painter()
                        .rect_filled(rect, 0.0, Color32::from_rgb(16, 18, 30));
                }

                // 2. 根据 game.json 中的 menu_layout 选择 5 种不同的菜单布局
                let layout = self.game_config.menu_layout;
                let btn_size = egui::vec2(240.0, 46.0);

                match layout {
                    // ---------------------------------------------------------
                    // 方案 1: 经典居中布局 (Classic Center)
                    // ---------------------------------------------------------
                    1 => {
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
                                RichText::new(&self.game_config.title)
                                    .size(54.0)
                                    .strong()
                                    .color(Color32::from_rgb(120, 220, 255)),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new("视觉小说引擎 · 经典居中")
                                    .size(18.0)
                                    .color(Color32::from_rgb(180, 200, 220)),
                            );
                            ui.add_space(56.0);

                            if menu_button(ui, btn_size, "开始游戏") { self.start_game(ctx); }
                            ui.add_space(10.0);
                            if menu_button(ui, btn_size, "读取存档") { self.last_screen = Screen::Menu; self.screen = Screen::Load; }
                            ui.add_space(10.0);
                            if menu_button(ui, btn_size, "设置") { self.last_screen = Screen::Menu; self.screen = Screen::Settings; }
                            ui.add_space(10.0);
                            if menu_button(ui, btn_size, "退出") { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                        });
                    },

                    // ---------------------------------------------------------
                    // 方案 2: 左侧对齐布局 (Left Aligned / Galgame 经典风)
                    // ---------------------------------------------------------
                    2 => {
                        let left_margin = rect.left() + 90.0;
                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(Rect::from_min_size(
                                Pos2::new(left_margin, rect.top() + 150.0),
                                Vec2::new(400.0, 500.0),
                            )),
                            |ui| {
                                ui.label(
                                    RichText::new(&self.game_config.title)
                                        .size(48.0)
                                        .strong()
                                        .color(Color32::from_rgb(255, 255, 255)),
                                );
                                ui.add_space(8.0);
                                ui.label(
                                    RichText::new("视觉小说引擎 · 左侧风格")
                                        .size(16.0)
                                        .color(Color32::from_rgb(180, 200, 220)),
                                );
                                ui.add_space(45.0);

                                if menu_button(ui, btn_size, "开始游戏") { self.start_game(ctx); }
                                ui.add_space(12.0);
                                if menu_button(ui, btn_size, "读取存档") { self.last_screen = Screen::Menu; self.screen = Screen::Load; }
                                ui.add_space(12.0);
                                if menu_button(ui, btn_size, "设置") { self.last_screen = Screen::Menu; self.screen = Screen::Settings; }
                                ui.add_space(12.0);
                                if menu_button(ui, btn_size, "退出") { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                            },
                        );
                    },

                    // ---------------------------------------------------------
                    // 方案 3: 右侧悬浮侧边栏布局 (Right Floating Panel)
                    // ---------------------------------------------------------
                    3 => {
                        let panel_width = 380.0;
                        let panel_rect = Rect::from_min_max(
                            Pos2::new(rect.right() - panel_width, rect.top()),
                            rect.max,
                        );
                        ui.painter().rect_filled(panel_rect, 0.0, Color32::from_black_alpha(210));

                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(panel_rect.shrink(35.0)),
                            |ui| {
                                ui.add_space(100.0);
                                ui.label(
                                    RichText::new(&self.game_config.title)
                                        .size(38.0)
                                        .strong()
                                        .color(Color32::from_rgb(100, 200, 255)),
                                );
                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new("视觉小说引擎 · 右侧悬浮栏")
                                        .size(14.0)
                                        .color(Color32::from_gray(160)),
                                );
                                ui.add_space(60.0);

                                let right_btn = egui::vec2(310.0, 46.0);
                                if menu_button(ui, right_btn, "开始游戏") { self.start_game(ctx); }
                                ui.add_space(12.0);
                                if menu_button(ui, right_btn, "读取存档") { self.last_screen = Screen::Menu; self.screen = Screen::Load; }
                                ui.add_space(12.0);
                                if menu_button(ui, right_btn, "设置") { self.last_screen = Screen::Menu; self.screen = Screen::Settings; }
                                ui.add_space(12.0);
                                if menu_button(ui, right_btn, "退出") { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                            },
                        );
                    },

                    // ---------------------------------------------------------
                    // 方案 4: 下方一排横向菜单栏布局 (Bottom Row / Footer Bar)
                    // ---------------------------------------------------------
                    4 => {
                        ui.vertical_centered(|ui| {
                            ui.add_space(rect.height() * 0.28);
                            ui.label(
                                RichText::new(&self.game_config.title)
                                    .size(52.0)
                                    .strong()
                                    .color(Color32::from_rgb(120, 220, 255)),
                            );
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("视觉小说引擎 · 下方一排风格")
                                    .size(16.0)
                                    .color(Color32::from_rgb(180, 200, 220)),
                            );
                        });

                        let bar_height = 85.0;
                        let bar_rect = Rect::from_min_size(
                            Pos2::new(rect.left(), rect.bottom() - bar_height),
                            Vec2::new(rect.width(), bar_height),
                        );
                        ui.painter().rect_filled(bar_rect, 0.0, Color32::from_black_alpha(200));

                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(bar_rect.shrink(18.0)),
                            |ui| {
                                ui.horizontal(|ui| {
                                    let btn_w = 200.0;
                                    let total_w = 4.0 * btn_w;
                                    let spacing = (rect.width() - total_w).max(40.0) / 5.0;
                                    ui.add_space(spacing);

                                    let b_size = egui::vec2(btn_w, 45.0);
                                    if menu_button(ui, b_size, "开始游戏") { self.start_game(ctx); }
                                    ui.add_space(spacing);
                                    if menu_button(ui, b_size, "读取存档") { self.last_screen = Screen::Menu; self.screen = Screen::Load; }
                                    ui.add_space(spacing);
                                    if menu_button(ui, b_size, "设置") { self.last_screen = Screen::Menu; self.screen = Screen::Settings; }
                                    ui.add_space(spacing);
                                    if menu_button(ui, b_size, "退出") { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                                });
                            },
                        );
                    },

                    // ---------------------------------------------------------
                    // 方案 5: 中央卡片毛玻璃框布局 (Central Card Box)
                    // ---------------------------------------------------------
                    _ => {
                        let card_size = Vec2::new(520.0, 420.0);
                        let card_rect = Rect::from_center_size(rect.center(), card_size);
                        ui.painter().rect_filled(card_rect, 12.0, Color32::from_rgba_premultiplied(18, 22, 38, 230));
                        ui.painter().rect_stroke(
                            card_rect,
                            12.0,
                            Stroke::new(1.5_f32, Color32::from_rgb(70, 130, 190)),
                            StrokeKind::Inside,
                        );

                        ui.allocate_new_ui(
                            UiBuilder::new().max_rect(card_rect.shrink(30.0)),
                            |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(10.0);
                                    ui.label(
                                        RichText::new(&self.game_config.title)
                                            .size(38.0)
                                            .strong()
                                            .color(Color32::from_rgb(130, 220, 255)),
                                    );
                                    ui.add_space(4.0);
                                    ui.label(
                                        RichText::new("视觉小说引擎 · 中央卡片风")
                                            .size(14.0)
                                            .color(Color32::from_gray(180)),
                                    );
                                    ui.add_space(35.0);

                                    if menu_button(ui, btn_size, "开始游戏") { self.start_game(ctx); }
                                    ui.add_space(10.0);
                                    if menu_button(ui, btn_size, "读取存档") { self.last_screen = Screen::Menu; self.screen = Screen::Load; }
                                    ui.add_space(10.0);
                                    if menu_button(ui, btn_size, "设置") { self.last_screen = Screen::Menu; self.screen = Screen::Settings; }
                                    ui.add_space(10.0);
                                    if menu_button(ui, btn_size, "退出") { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                                });
                            },
                        );
                    }
                }

                // 版本号标记
                ui.painter().text(
                    Pos2::new(rect.right() - 16.0, rect.bottom() - 12.0),
                    Align2::RIGHT_BOTTOM,
                    "v0.1.0",
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
