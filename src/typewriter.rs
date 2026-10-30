use eframe::egui::Context;

pub struct Typewriter {
    full_text: String,
    visible_count: f32,
    pub chars_per_sec: f32,
}

impl Typewriter {
    pub fn new(text: &str, cps: f32) -> Self {
        Self {
            full_text: text.to_owned(),
            visible_count: 0.0,
            chars_per_sec: cps,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.full_text.clear();
        self.full_text.push_str(text);
        self.visible_count = 0.0;
    }

    pub fn update(&mut self, ctx: &Context) {
        let total = self.full_text.chars().count() as f32;
        if self.visible_count < total {
            let dt = ctx.input(|i| i.stable_dt).min(0.1);
            self.visible_count = (self.visible_count + self.chars_per_sec * dt).min(total);
            ctx.request_repaint();
        }
    }

    pub fn visible_text(&self) -> String {
        self.full_text
            .chars()
            .take(self.visible_count as usize)
            .collect()
    }

    pub fn finish(&mut self) {
        self.visible_count = self.full_text.chars().count() as f32;
    }

    pub fn is_done(&self) -> bool {
        self.visible_count >= self.full_text.chars().count() as f32
    }
}