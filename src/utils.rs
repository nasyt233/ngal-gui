use crate::app::NgalApp;
use crate::texture::load_texture;
use eframe::egui::{self, Color32, Pos2, RichText, Stroke};
use std::path::Path;

impl NgalApp {
    /// 向后预加载未来剧情将要出现的最多 3 张图片
    pub fn preload_upcoming_images(&mut self, ctx: &egui::Context) {
        let start = self.line_index.saturating_add(1);
        let end = (start + 40).min(self.parser.nodes.len());
        let mut preloaded_count = 0;
        let max_preload = 3;

        for i in start..end {
            if preloaded_count >= max_preload {
                break;
            }
            let node = &self.parser.nodes[i];
            let mut paths_to_check = Vec::new();

            if let Some(ref text) = node.text {
                if text.starts_with("bg:") {
                    let path = format!("assets/portraits/{}", text.strip_prefix("bg:").unwrap().trim());
                    paths_to_check.push(path);
                } else if text.starts_with("img:") {
                    let parts: Vec<&str> = text.strip_prefix("img:").unwrap().split(':').collect();
                    if !parts.is_empty() && !parts[0].is_empty() {
                        let path = format!("assets/portraits/{}", parts[0].trim());
                        paths_to_check.push(path);
                    }
                }
            }

            for path in paths_to_check {
                if !self.texture_cache.contains_key(&path) && Path::new(&path).exists() {
                    if let Some(tex) = load_texture(ctx, &path) {
                        self.texture_cache.insert(path, tex);
                        preloaded_count += 1;
                        if preloaded_count >= max_preload {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn get_memory_usage_mb() -> f32 {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/self/statm") {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(pages) = parts[1].parse::<u64>() {
                        return (pages * 4096) as f32 / (1024.0 * 1024.0);
                    }
                }
            }
        }
        0.0
    }

    pub fn draw_performance_overlay(&self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.unstable_dt);
        let fps = if dt > 0.0 { 1.0 / dt } else { 0.0 };
        let frame_time = dt * 1000.0;
        let mem_mb = Self::get_memory_usage_mb();

        egui::Area::new(egui::Id::new("perf_overlay"))
            .fixed_pos(Pos2::new(12.0, 12.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(Color32::from_rgba_premultiplied(15, 18, 28, 210))
                    .corner_radius(6.0)
                    .inner_margin(10.0)
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(70, 130, 180)))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let fps_color = if fps >= 55.0 {
                                Color32::from_rgb(100, 255, 100)
                            } else if fps >= 30.0 {
                                Color32::from_rgb(255, 255, 100)
                            } else {
                                Color32::from_rgb(255, 100, 100)
                            };

                            ui.label(
                                RichText::new(format!("FPS: {:.1} ({:.1} ms)", fps, frame_time))
                                    .size(13.0)
                                    .strong()
                                    .color(fps_color),
                            );

                            if mem_mb > 0.0 {
                                ui.label(
                                    RichText::new(format!("内存 (RAM): {:.1} MB", mem_mb))
                                        .size(12.0)
                                        .color(Color32::from_rgb(200, 220, 250)),
                                );
                            }

                            ui.label(
                                RichText::new(format!("显存纹理缓存: {} 项", self.texture_cache.len()))
                                    .size(12.0)
                                    .color(Color32::from_rgb(200, 220, 250)),
                            );
                        });
                    });
            });
    }
}

pub fn ensure_asset_structure(game_config_path: &str) {
    for d in [
        "assets/dialog",
        "assets/music",
        "assets/portraits",
        "assets/voices",
        "save",
    ] {
        let _ = std::fs::create_dir_all(d);
    }

    if !Path::new(game_config_path).exists() {
        let default_game_config = r#"{
  "title": "Ngal 视觉小说引擎",
  "logo": "logo.png",
  "background": "bg.png",
  "bgm": "bg.mp3",
  "index": "dialogue.ng",
  "menu_layout": 1,
  "splash_screens": [
    "logo.png"
  ]
}"#;
        let _ = std::fs::write(game_config_path, default_game_config);
    }

    let dialogue_ng_path = "assets/dialog/dialogue.ng";
    if !Path::new(dialogue_ng_path).exists() || std::fs::metadata(dialogue_ng_path).map(|m| m.len() == 0).unwrap_or(true) {
        let default_story = r#"# ngal 示例教学默认剧情
[welcome]
第一章
load:index

[index]
name = 嘉豪
bg:bg.png
music:bg.mp3
img:logo.png:2:50%
系统:欢迎来到 ngal 引擎！
系统:按回车键，等待2秒
sleep:2
系统:2秒等待结束。
系统:你好用户:hello.mp3
系统:当前时间 $(date)
系统:当前用户 $(whoami)\n当前目录 $(pwd)
img:logo2.png
img:
系统:当前默认名字：{name}
input:请输入你的名字:name
{name}：我的名字是：{name}！
变量运算
a = 13
系统:a的值为{a}
b = 78
系统:b的值为{b}
c = a + b
系统:相加计算结果为 {c}
系统:下面是选项选择
score = 10
系统:当前分数为{score}
choose:接受冒险(分数+8):accept|拒绝冒险(分数-5):refuse

[accept]
系统:你接受了冒险！
score = score + 8
系统:当前分数 {score}
load:jx

[refuse]
系统:你拒绝了冒险！
score = score - 5
系统:当前分数 {score}
load:jx

[jx]
system:if判断功能
if score >= 10: good_end
load:bad_end

[good_end]
系统:当前分数大于或等于10
系统:🤓 完美结局！得分 {score}
load:exit

[bad_end]
系统:分数小于10
系统:😭 遗憾结局。得分 {score}
load:exit

[exit]
系统:游戏结束
bg:
music:
end
"#;
        let _ = std::fs::write(dialogue_ng_path, default_story);
    }
}
