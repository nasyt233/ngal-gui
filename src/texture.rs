use eframe::egui::{ColorImage, Context, TextureHandle, TextureOptions};
use image::GenericImageView;
use std::path::Path;

pub fn load_texture(ctx: &Context, path: &str) -> Option<TextureHandle> {
    if path.is_empty() {
        return None;
    }

    let p = Path::new(path);
    if !p.exists() {
        let alt_path = format!("assets/{}", path);
        if !Path::new(&alt_path).exists() {
            return None;
        }
        return load_texture_from_path(ctx, &alt_path);
    }

    load_texture_from_path(ctx, path)
}

pub fn load_thumbnail_texture(ctx: &egui::Context, path: &str, max_w: u32, max_h: u32) -> Option<egui::TextureHandle> {
    let possible_paths = vec![
        path.to_string(),
        format!("assets/portraits/{}", path),
        format!("assets/{}", path),
    ];

    let mut target_path = None;
    for p in &possible_paths {
        if std::path::Path::new(p).exists() {
            target_path = Some(p.clone());
            break;
        }
    }

    let real_path = match target_path {
        Some(p) => p,
        None => {
            eprintln!("⚠️ [缩略图] 找不到文件，尝试过的路径: {:?}", possible_paths);
            return None;
        }
    };
    
    let img = match image::open(&real_path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("❌ [缩略图] 解码图片失败 [{}] -> 错误: {:?}", real_path, err);
            return None;
        }
    };

    let resized = img.thumbnail_exact(max_w, max_h); 
    let size = [resized.width() as usize, resized.height() as usize];
    let image_buffer = resized.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    Some(ctx.load_texture(format!("thumb_{}", real_path), color_image, egui::TextureOptions::default()))
}

fn load_texture_from_path(ctx: &Context, path: &str) -> Option<TextureHandle> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => return None,
    };

    let img = match image::load_from_memory(&bytes) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("⚠️ [图片加载警告] 无法解码图片 '{}': {}", path, e);
            return None;
        }
    };

    let (width, height) = img.dimensions();
    let max_dim = 1920;
    
    let resized_img = if width > max_dim || height > max_dim {
        img.resize(max_dim, max_dim, image::imageops::FilterType::Triangle)
    } else {
        img
    };

    let rgba = resized_img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.as_flat_samples();
    
    Some(ctx.load_texture(
        path,
        ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
        TextureOptions::default(),
    ))
}
