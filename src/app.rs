use crate::config::{Config, GameConfig, GAME_CONFIG_PATH};
use crate::fonts::{apply_text_styles, load_system_cjk_font};
use crate::parser::StoryParser;
use crate::texture::load_texture;
use crate::typewriter::Typewriter;
use crate::save::SaveData;
use crate::utils::ensure_asset_structure;
use eframe::egui::{self, Pos2};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum Screen {
    Splash,
    Menu,
    Settings,
    Game,
    Save,
    Load,
}

pub struct NgalApp {
    pub(crate) screen: Screen,
    pub last_screen: Screen,
    pub(crate) config: Config,
    pub(crate) game_config: GameConfig,
    pub(crate) config_path: String,
    pub(crate) background: Option<egui::TextureHandle>,
    pub(crate) portrait: Option<egui::TextureHandle>,
    pub(crate) current_bg_path: Option<String>,
    pub(crate) current_portrait_path: Option<String>,
    pub(crate) current_script_file: String,
    pub(crate) current_label: String,
    pub(crate) current_bgm: Option<String>,
    pub(crate) texture_cache: HashMap<String, egui::TextureHandle>,
    pub(crate) save_slots_cache: HashMap<usize, Option<SaveData>>,
    pub(crate) save_thumb_cache: HashMap<usize, Option<egui::TextureHandle>>,
    pub(crate) saveload_cache_dirty: bool,
    #[allow(dead_code)]
    pub(crate) portrait_pos: Pos2,
    pub(crate) portrait_scale: f32,
    pub(crate) portrait_align: u8,
    pub(crate) hide_ui: bool,
    pub(crate) speaker: String,
    pub(crate) typewriter: Typewriter,
    
    pub(crate) parser: StoryParser,
    pub(crate) line_index: usize,
    pub(crate) history: Vec<(String, String)>,
    
    pub(crate) auto_play: bool,
    pub(crate) auto_timer: f32,
    pub(crate) is_skipping: bool,
    pub(crate) show_log: bool,
    pub(crate) waiting_input: bool,
    pub(crate) input_prompt: String,
    pub(crate) input_buffer: String,
    pub(crate) input_var_name: String,
    
    pub(crate) sleep_timer: f32,

    pub(crate) splash_index: usize,
    pub(crate) splash_timer: f32,
    pub(crate) splash_textures: HashMap<String, egui::TextureHandle>,
}

impl NgalApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config, config_path: String) -> Self {
        let ctx = &cc.egui_ctx;
    
        ensure_asset_structure(GAME_CONFIG_PATH);
        let game_config = GameConfig::load_or_default(GAME_CONFIG_PATH);
        load_system_cjk_font(ctx);
        apply_text_styles(ctx);
    
        let initial_screen = if !game_config.splash_screens.is_empty() {
            Screen::Splash
        } else {
            Screen::Menu
        };

        let default_bg_path = game_config.get_background_path();
        let script_path = game_config.get_index_path();
        
        let parser = StoryParser::load_from_file(&script_path);
        let default_bg = load_texture(ctx, &default_bg_path);
    
        let speed = config.text_speed;
        Self {
            screen: initial_screen,
            last_screen: Screen::Menu,
            config,
            game_config,
            config_path,
            background: default_bg,
            current_bg_path: Some(default_bg_path),
            current_portrait_path: None,
            current_script_file: script_path,
            current_label: "index".to_string(),
            current_bgm: None,
            texture_cache: HashMap::new(),
            save_slots_cache: HashMap::new(),
            save_thumb_cache: HashMap::new(),
            saveload_cache_dirty: true,
            portrait: None,
            portrait_pos: Pos2::new(0.5, 0.85),
            portrait_scale: 0.5,
            portrait_align: 2,
            hide_ui: false,
            speaker: "".to_owned(),
            typewriter: Typewriter::new("", speed),
            parser,
            line_index: 0,
            history: vec![],
            auto_play: false,
            auto_timer: 0.0,
            is_skipping: false,
            show_log: false,
            waiting_input: false,
            input_buffer: String::new(),
            input_var_name: String::new(),
            input_prompt: String::new(),
            sleep_timer: 0.0,
            splash_index: 0,
            splash_timer: 0.0,
            splash_textures: HashMap::new(),
        }
    }
}

impl eframe::App for NgalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Splash => self.draw_splash(ctx),
            Screen::Menu => self.draw_menu(ctx),
            Screen::Settings => self.draw_settings(ctx),
            Screen::Game => {
                self.preload_upcoming_images(ctx);
                self.draw_game(ctx);
            },
            Screen::Save => self.draw_saveload(ctx, true),
            Screen::Load => self.draw_saveload(ctx, false),
        }

        if self.config.show_fps {
            self.draw_performance_overlay(ctx);
        }
    }
}
