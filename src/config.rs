use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CONFIG_PATH: &str = "assets/config.json";
pub const GAME_CONFIG_PATH: &str = "assets/game.json";

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub fullscreen: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub text_speed: f32,
    pub bgm_volume: f32,
    pub voice_volume: f32,
    pub dialog_opacity: f32, // 新增：对话框窗口透明度 (0.0 ~ 1.0)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fullscreen: false,
            window_width: 1280,
            window_height: 720,
            text_speed: 25.0,
            bgm_volume: 0.7,
            voice_volume: 1.0,
            dialog_opacity: 0.85, // 默认半透明
        }
    }
}

impl Config {
    pub fn load_or_default(path: &str) -> Self {
        let cfg = if !Path::new(path).exists() {
            let c = Config::default();
            c.save(path);
            c
        } else {
            std::fs::read_to_string(path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        };
        // 自动补全缺失字段并写回文件
        cfg.save(path);
        cfg
    }

    pub fn save(&self, path: &str) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, s);
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GameConfig {
    pub title: String,
    pub logo: String,
    pub background: String,
    pub bgm: String,
    pub index: String,
    pub menu_layout: i32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: "Ngal 视觉小说引擎".to_string(),
            logo: "logo.png".to_string(),
            background: "bg.png".to_string(),
            bgm: "bg.mp3".to_string(),
            index: "dialogue.ng".to_string(),
            menu_layout: 1,
        }
    }
}

impl GameConfig {
    pub fn load_or_default(path: &str) -> Self {
        let cfg = if !Path::new(path).exists() {
            let c = GameConfig::default();
            c.save(path);
            c
        } else {
            std::fs::read_to_string(path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        };
        // 自动补全缺失字段并写回文件
        cfg.save(path);
        cfg
    }

    pub fn save(&self, path: &str) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, s);
        }
    }

    pub fn get_background_path(&self) -> String {
        format!("assets/portraits/{}", self.background)
    }

    #[allow(dead_code)]
    pub fn get_bgm_path(&self) -> String {
        format!("assets/music/{}", self.bgm)
    }

    pub fn get_index_path(&self) -> String {
        format!("assets/dialog/{}", self.index)
    }

    pub fn get_logo_path(&self) -> String {
        format!("assets/portraits/{}", self.logo)
    }
}
