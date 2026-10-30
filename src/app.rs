use crate::config::Config;
use crate::fonts::{apply_text_styles, load_system_cjk_font};
use crate::texture::load_texture;
use crate::typewriter::Typewriter;
use eframe::egui::{self, Pos2};

#[derive(PartialEq, Clone, Copy)]
pub enum Screen {
    Menu,
    Settings,
    Game,
}

pub struct NgalApp {
    pub(crate) screen: Screen,
    pub(crate) config: Config,
    pub(crate) config_path: String,
    pub(crate) background: Option<egui::TextureHandle>,
    pub(crate) portrait: Option<egui::TextureHandle>,
    pub(crate) portrait_pos: Pos2,
    pub(crate) portrait_scale: f32,
    pub(crate) speaker: String,
    pub(crate) typewriter: Typewriter,
    pub(crate) lines: Vec<(String, String)>,
    pub(crate) line_index: usize,
}

impl NgalApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config, config_path: String) -> Self {
        let ctx = &cc.egui_ctx;

        ensure_asset_dirs();
        load_system_cjk_font(ctx);
        apply_text_styles(ctx);

        let background = load_texture(ctx, "assets/backgrounds/bg.png");
        let portrait = load_texture(ctx, "assets/portraits/logo.png");

        let lines = vec![
            (
                "系统".to_owned(),
                "欢迎来到 ngal 引擎。按回车或点击继续。".to_owned(),
            ),
            (
                "旁白".to_owned(),
                "这是一个用 egui 仿写的终端视觉小说 UI 骨架。".to_owned(),
            ),
            (
                "角色A".to_owned(),
                "背景、立绘、对话框都已分层，按 Esc 可以回到主菜单。".to_owned(),
            ),
        ];

        let speed = config.text_speed;
        Self {
            screen: Screen::Menu,
            config,
            config_path,
            background,
            portrait,
            portrait_pos: Pos2::new(0.5, 0.85),
            portrait_scale: 0.5,
            speaker: lines[0].0.clone(),
            typewriter: Typewriter::new(&lines[0].1, speed),
            lines,
            line_index: 0,
        }
    }

    pub fn start_game(&mut self) {
        self.line_index = 0;
        self.speaker = self.lines[0].0.clone();
        self.typewriter = Typewriter::new(&self.lines[0].1, self.config.text_speed);
        self.screen = Screen::Game;
    }

    pub fn advance(&mut self) {
        if !self.typewriter.is_done() {
            self.typewriter.finish();
            return;
        }
        if self.line_index + 1 < self.lines.len() {
            self.line_index += 1;
            let (speaker, text) = &self.lines[self.line_index];
            self.speaker = speaker.clone();
            self.typewriter.set_text(text);
        }
    }
}

/// 首次启动时自动创建 ngal 风格资源目录
fn ensure_asset_dirs() {
    for d in [
        "assets/dialog",
        "assets/backgrounds",
        "assets/portraits",
        "assets/music",
        "assets/voices",
    ] {
        let _ = std::fs::create_dir_all(d);
    }
}

impl eframe::App for NgalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Menu => self.draw_menu(ctx),
            Screen::Settings => self.draw_settings(ctx),
            Screen::Game => self.draw_game(ctx),
        }
    }
}