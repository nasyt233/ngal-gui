mod app;
mod config;
mod fonts;
mod screens;
mod texture;
mod typewriter;
mod ui;

use app::NgalApp;
use config::{Config, CONFIG_PATH};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let config = Config::load_or_default(CONFIG_PATH);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(config.fullscreen)
            .with_decorations(true)
            .with_inner_size([config.window_width as f32, config.window_height as f32]),
        ..Default::default()
    };

    let cfg_for_app = config.clone();
    eframe::run_native(
        "ngal-egui",
        options,
        Box::new(move |cc| {
            Ok(Box::new(NgalApp::new(
                cc,
                cfg_for_app.clone(),
                CONFIG_PATH.to_string(),
            )))
        }),
    )
}