mod app;
mod config;
mod fonts;
mod parser;
mod save;
mod texture;
mod typewriter;
mod ui;

mod screens {
    pub mod game;
    pub mod menu;
    pub mod saveload;
    pub mod settings;
}

use app::NgalApp;
use config::{Config, GameConfig, CONFIG_PATH, GAME_CONFIG_PATH};
use std::env;
use std::path::Path;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let target_dir = &args[1];
        
        let resolved_dir = if target_dir.starts_with('~') {
            if let Ok(home) = env::var("HOME") {
                target_dir.replacen('~', &home, 1)
            } else {
                target_dir.clone()
            }
        } else {
            target_dir.clone()
        };

        let path = Path::new(&resolved_dir);
        if path.exists() && path.is_dir() {
            match env::set_current_dir(path) {
                Ok(_) => {
                    println!("[系统] 成功切换工作目录到: {}", path.display());
                }
                Err(e) => {
                    eprintln!("[错误] 无法切换工作目录到 {}: {}", resolved_dir, e);
                }
            }
        } else {
            eprintln!("[警告] 指定的路径不存在或不是有效目录: {}", resolved_dir);
        }
    }

    let game_config = GameConfig::load_or_default(GAME_CONFIG_PATH);

    // 加载 Logo 图标
    let icon = load_icon(&game_config.get_logo_path());

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_title(&game_config.title)
        .with_inner_size([1280.0, 720.0]);

    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(Arc::new(icon_data));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let config = Config::load_or_default(CONFIG_PATH);
    let config_path = CONFIG_PATH.to_string();

    eframe::run_native(
        &game_config.title,
        options,
        Box::new(move |cc| {
            Ok(Box::new(NgalApp::new(cc, config, config_path)))
        }),
    )
}

fn load_icon(path: &str) -> Option<egui::IconData> {
    let img = image::open(path).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    Some(egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}
