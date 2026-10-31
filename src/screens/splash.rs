use crate::app::{NgalApp, Screen};
use crate::texture::load_texture;
use eframe::egui::{self, Color32, Pos2, Rect};

impl NgalApp {
    /// 启动图动画渲染逻辑
    pub fn draw_splash(&mut self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.unstable_dt);
        self.splash_timer += dt;

        let clicked = ctx.input(|i| i.pointer.any_click() || !i.keys_down.is_empty());

        if self.game_config.splash_screens.is_empty() {
            self.screen = Screen::Menu;
            return;
        }

        let fade_in_time = 0.6;
        let hold_time = 1.8;
        let fade_out_time = 0.6;
        let total_per_image = fade_in_time + hold_time + fade_out_time;

        if clicked || self.splash_timer >= total_per_image {
            self.splash_index += 1;
            self.splash_timer = 0.0;
            if self.splash_index >= self.game_config.splash_screens.len() {
                self.screen = Screen::Menu;
                return;
            }
        }

        let t = self.splash_timer;
        let alpha = if t < fade_in_time {
            (t / fade_in_time * 255.0) as u8
        } else if t < fade_in_time + hold_time {
            255
        } else {
            let out_t = t - fade_in_time - hold_time;
            ((1.0 - (out_t / fade_out_time)) * 255.0).clamp(0.0, 255.0) as u8
        };

        let img_filename = self.game_config.splash_screens[self.splash_index].clone();
        
        let texture_opt = if let Some(tex) = self.splash_textures.get(&img_filename) {
            Some(tex.clone())
        } else {
            let loaded = load_texture(ctx, &format!("assets/portraits/{}", img_filename))
                .or_else(|| load_texture(ctx, &img_filename));
            
            if let Some(ref tex) = loaded {
                self.splash_textures.insert(img_filename.clone(), tex.clone());
            }
            loaded
        };

        if let Some(texture) = texture_opt {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(Color32::BLACK))
                .show(ctx, |ui| {
                    let rect = ui.max_rect();
                    ui.painter().rect_filled(rect, 0.0, Color32::BLACK);

                    let tex_size = texture.size_vec2();
                    if tex_size.x > 0.0 && tex_size.y > 0.0 {
                        let scale = (rect.width() / tex_size.x).min(rect.height() / tex_size.y).min(1.0);
                        let draw_size = tex_size * scale;
                        let draw_rect = Rect::from_center_size(rect.center(), draw_size);

                        ui.painter().image(
                            texture.id(),
                            draw_rect,
                            Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                            Color32::from_white_alpha(alpha),
                        );
                    }
                });
        } else {
            self.splash_index += 1;
            self.splash_timer = 0.0;
            if self.splash_index >= self.game_config.splash_screens.len() {
                self.screen = Screen::Menu;
            }
        }

        ctx.request_repaint();
    }
}
