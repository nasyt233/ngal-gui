use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CONFIG_PATH: &str = "config.json";

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub fullscreen: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub text_speed: f32,
    pub bgm_volume: f32,
    pub voice_volume: f32,
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
        }
    }
}

impl Config {
    pub fn load_or_default(path: &str) -> Self {
        if !Path::new(path).exists() {
            let cfg = Config::default();
            cfg.save(path);
            return cfg;
        }
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &str) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, s);
        }
    }
}