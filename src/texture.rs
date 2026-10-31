use eframe::egui::{Context, TextureHandle, TextureOptions, ColorImage};
use std::path::Path;

pub fn load_texture(ctx: &Context, path: &str) -> Option<TextureHandle> {
    if path.is_empty() {
        return None;
    }

    // 检查文件是否存在
    let p = Path::new(path);
    if !p.exists() {
        // 如果不存在尝试在 assets/ 目录下寻找
        let alt_path = format!("assets/{}", path);
        if !Path::new(&alt_path).exists() {
            return None;
        }
        return load_texture_from_path(ctx, &alt_path);
    }

    load_texture_from_path(ctx, path)
}

fn load_texture_from_path(ctx: &Context, path: &str) -> Option<TextureHandle> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => return None,
    };

    let image = match image::load_from_memory(&bytes) {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            eprintln!("⚠️ [图片加载警告] 无法解码图片 '{}': {}", path, e);
            return None;
        }
    };

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.as_flat_samples();
    
    Some(ctx.load_texture(
        path,
        ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
        TextureOptions::default(),
    ))
}
