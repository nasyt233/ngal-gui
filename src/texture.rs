use eframe::egui::{Context, TextureHandle, ColorImage};

pub fn load_texture(ctx: &Context, path: &str) -> Option<TextureHandle> {
    let img = match image::open(path) {
        Ok(i) => i.to_rgba8(),
        Err(e) => {
            eprintln!("[图片加载失败] 目标路径: '{}', 错误原因: {}", path, e);
            return None;
        }
    };
    
    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(path, color_image, Default::default()))
}
