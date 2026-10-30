use eframe::egui::{ColorImage, Context, TextureHandle, TextureOptions};

pub fn load_texture(ctx: &Context, path: &str) -> Option<TextureHandle> {
    let img = image::open(path).ok()?.to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(path, color_image, TextureOptions::LINEAR))
}