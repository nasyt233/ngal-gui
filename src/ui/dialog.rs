use crate::typewriter::Typewriter;
use eframe::egui::{
    self, Align, Align2, Color32, FontId, Layout, Pos2, Rect, RichText, Stroke, StrokeKind, Vec2,
};

pub fn draw_dialog(ui: &mut egui::Ui, screen: Rect, speaker: &str, tw: &Typewriter) {
    let margin = 48.0;
    let dialog_h = screen.height() * 0.28;
    let dialog_rect = Rect::from_min_max(
        Pos2::new(screen.left() + margin, screen.bottom() - dialog_h - margin),
        Pos2::new(screen.right() - margin, screen.bottom() - margin),
    );

    {
        let painter = ui.painter();
        painter.rect_filled(dialog_rect, 6.0, Color32::from_black_alpha(180));

        let outer = Stroke::new(2.0_f32, Color32::from_rgb(120, 220, 255));
        let inner = Stroke::new(1.0_f32, Color32::from_rgb(60, 120, 180));
        painter.rect_stroke(dialog_rect, 6.0, outer, StrokeKind::Outside);
        let inner_rect = dialog_rect.shrink(4.0);
        painter.rect_stroke(inner_rect, 4.0, inner, StrokeKind::Inside);

        let name_pos = dialog_rect.left_top() + Vec2::new(16.0, -28.0);
        painter.text(
            name_pos,
            Align2::LEFT_BOTTOM,
            speaker,
            FontId::proportional(20.0),
            Color32::from_rgb(255, 220, 120),
        );
    }

    let inner_rect = dialog_rect.shrink(4.0);
    let text_rect = inner_rect.shrink(16.0);
    let mut text_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(text_rect)
            .layout(Layout::top_down(Align::LEFT)),
    );
    text_ui.add(
        egui::Label::new(
            RichText::new(tw.visible_text())
                .size(22.0)
                .color(Color32::WHITE),
        )
        .wrap(),
    );
}