/// Shared rendering functions for schematic elements.
///
/// Both the auto-generated converter schematic view and the
/// interactive schematic editor use this renderer.

use crate::schematic::primitives::SchematicElement;
use egui::{Color32, Pos2, Stroke, Vec2};

/// Draw a single schematic element on the canvas.
///
/// # Arguments
/// * `painter` - The egui painter
/// * `element` - The element to draw
/// * `origin` - Offset to add to all positions (for canvas panning)
/// * `highlight` - If true, draw a selection highlight around the element
pub fn draw_element(
    painter: &egui::Painter,
    element: &SchematicElement,
    origin: Pos2,
    highlight: bool,
) {
    let (pos, size) = element_bounds(element);

    // Selection highlight
    if highlight {
        let cx = origin.x + pos.x;
        let cy = origin.y + pos.y;
        painter.rect_stroke(
            egui::Rect::from_center_size(
                Pos2::new(cx, cy),
                Vec2::new(size.x + 20.0, size.y + 20.0),
            ),
            3.0,
            Stroke::new(1.5, Color32::YELLOW),
            egui::StrokeKind::Outside,
        );
    }

    match element {
        SchematicElement::Wire { from, to } => {
            painter.line_segment(
                [
                    Pos2::new(origin.x + from.x, origin.y + from.y),
                    Pos2::new(origin.x + to.x, origin.y + to.y),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );
        }
        SchematicElement::Source { pos, label, value } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;
            let r = 14.0;

            painter.circle_stroke(Pos2::new(cx, cy), r, Stroke::new(2.0, Color32::WHITE));

            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx - 4.0, cy - 8.0),
                egui::Align2::CENTER_CENTER,
                "+",
                egui::TextStyle::Monospace.resolve(&style),
                Color32::WHITE,
            );
            painter.text(
                Pos2::new(cx - 4.0, cy + 8.0),
                egui::Align2::CENTER_CENTER,
                "-",
                egui::TextStyle::Monospace.resolve(&style),
                Color32::WHITE,
            );

            painter.text(
                Pos2::new(cx + r + 15.0, cy - 6.0),
                egui::Align2::LEFT_CENTER,
                format!("{}: {}", label, value),
                egui::TextStyle::Monospace.resolve(&style),
                Color32::LIGHT_YELLOW,
            );
        }
        SchematicElement::Resistor { pos, label, value } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;
            let w = 30.0;
            let h = 14.0;

            painter.rect_stroke(
                egui::Rect::from_center_size(Pos2::new(cx, cy), Vec2::new(w, h)),
                2.0,
                Stroke::new(2.0, Color32::WHITE),
                egui::StrokeKind::Outside,
            );

            painter.line_segment(
                [
                    Pos2::new(cx - w / 2.0, cy),
                    Pos2::new(cx - w / 2.0 - 10.0, cy),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );
            painter.line_segment(
                [
                    Pos2::new(cx + w / 2.0, cy),
                    Pos2::new(cx + w / 2.0 + 10.0, cy),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );

            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx + w / 2.0 + 15.0, cy - 6.0),
                egui::Align2::LEFT_CENTER,
                format!("{}: {}", label, value),
                egui::TextStyle::Monospace.resolve(&style),
                Color32::LIGHT_YELLOW,
            );
        }
        SchematicElement::Inductor { pos, label, value } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;
            let segments = 4;
            let seg_w = 8.0;
            let seg_h = 10.0;
            let total_w = segments as f32 * seg_w;
            let start_x = cx - total_w / 2.0;

            let mut prev = Pos2::new(start_x, cy);
            for i in 0..segments {
                let x1 = start_x + (i as f32 + 0.5) * seg_w;
                let y1 = if i % 2 == 0 { cy - seg_h } else { cy + seg_h };
                let next = Pos2::new(x1, y1);
                painter.line_segment([prev, next], Stroke::new(2.0, Color32::WHITE));
                prev = next;
            }
            painter.line_segment(
                [prev, Pos2::new(start_x + total_w, cy)],
                Stroke::new(2.0, Color32::WHITE),
            );

            painter.line_segment(
                [Pos2::new(start_x - 10.0, cy), Pos2::new(start_x, cy)],
                Stroke::new(2.0, Color32::WHITE),
            );
            painter.line_segment(
                [
                    Pos2::new(start_x + total_w, cy),
                    Pos2::new(start_x + total_w + 10.0, cy),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );

            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx, cy - seg_h - 12.0),
                egui::Align2::CENTER_CENTER,
                format!("{}: {}", label, value),
                egui::TextStyle::Monospace.resolve(&style),
                Color32::LIGHT_YELLOW,
            );
        }
        SchematicElement::Capacitor { pos, label, value } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;
            let half_plate = 10.0;

            painter.line_segment(
                [
                    Pos2::new(cx - half_plate, cy),
                    Pos2::new(cx + half_plate, cy),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );
            painter.line_segment(
                [
                    Pos2::new(cx - half_plate, cy + 12.0),
                    Pos2::new(cx + half_plate, cy + 12.0),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );

            painter.line_segment(
                [Pos2::new(cx, cy), Pos2::new(cx, cy - 8.0)],
                Stroke::new(2.0, Color32::WHITE),
            );
            painter.line_segment(
                [Pos2::new(cx, cy + 12.0), Pos2::new(cx, cy + 20.0)],
                Stroke::new(2.0, Color32::WHITE),
            );

            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx + half_plate + 12.0, cy + 3.0),
                egui::Align2::LEFT_CENTER,
                format!("{}: {}", label, value),
                egui::TextStyle::Monospace.resolve(&style),
                Color32::LIGHT_YELLOW,
            );
        }
        SchematicElement::Diode { pos, label } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;

            let tri_top = Pos2::new(cx, cy - 8.0);
            let tri_bot = Pos2::new(cx, cy + 8.0);
            let tri_right = Pos2::new(cx + 12.0, cy);

            painter.line_segment([tri_top, tri_right], Stroke::new(2.0, Color32::WHITE));
            painter.line_segment([tri_bot, tri_right], Stroke::new(2.0, Color32::WHITE));
            painter.line_segment([tri_top, tri_bot], Stroke::new(2.0, Color32::WHITE));

            painter.line_segment(
                [
                    Pos2::new(cx + 14.0, cy - 10.0),
                    Pos2::new(cx + 14.0, cy + 10.0),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );

            painter.line_segment(
                [Pos2::new(cx - 8.0, cy), Pos2::new(cx, cy)],
                Stroke::new(2.0, Color32::WHITE),
            );
            painter.line_segment(
                [Pos2::new(cx + 14.0, cy), Pos2::new(cx + 22.0, cy)],
                Stroke::new(2.0, Color32::WHITE),
            );

            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx + 22.0 + 12.0, cy - 2.0),
                egui::Align2::LEFT_CENTER,
                label.clone(),
                egui::TextStyle::Monospace.resolve(&style),
                Color32::LIGHT_YELLOW,
            );
        }
        SchematicElement::Switch { pos, label } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;

            painter.line_segment(
                [Pos2::new(cx - 12.0, cy), Pos2::new(cx, cy)],
                Stroke::new(2.0, Color32::WHITE),
            );

            painter.line_segment(
                [Pos2::new(cx, cy), Pos2::new(cx + 6.0, cy - 8.0)],
                Stroke::new(2.0, Color32::WHITE),
            );

            painter.line_segment(
                [Pos2::new(cx + 12.0, cy), Pos2::new(cx + 16.0, cy)],
                Stroke::new(2.0, Color32::WHITE),
            );

            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx + 6.0, cy - 14.0),
                egui::Align2::CENTER_CENTER,
                label.clone(),
                egui::TextStyle::Monospace.resolve(&style),
                Color32::LIGHT_YELLOW,
            );
        }
        SchematicElement::Ground { pos } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;

            painter.line_segment(
                [Pos2::new(cx, cy), Pos2::new(cx, cy + 6.0)],
                Stroke::new(2.0, Color32::WHITE),
            );

            painter.line_segment(
                [
                    Pos2::new(cx - 10.0, cy + 6.0),
                    Pos2::new(cx + 10.0, cy + 6.0),
                ],
                Stroke::new(2.0, Color32::WHITE),
            );
            painter.line_segment(
                [
                    Pos2::new(cx - 6.0, cy + 10.0),
                    Pos2::new(cx + 6.0, cy + 10.0),
                ],
                Stroke::new(1.5, Color32::WHITE),
            );
            painter.line_segment(
                [
                    Pos2::new(cx - 3.0, cy + 14.0),
                    Pos2::new(cx + 3.0, cy + 14.0),
                ],
                Stroke::new(1.0, Color32::WHITE),
            );
        }
        SchematicElement::Node { pos, label } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;

            painter.circle_filled(Pos2::new(cx, cy), 3.0, Color32::WHITE);
            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx + 6.0, cy - 4.0),
                egui::Align2::LEFT_CENTER,
                label,
                egui::TextStyle::Monospace.resolve(&style),
                Color32::GRAY,
            );
        }
        SchematicElement::Label { pos, text } => {
            let cx = origin.x + pos.x;
            let cy = origin.y + pos.y;

            let style = egui::Style::default();
            painter.text(
                Pos2::new(cx, cy),
                egui::Align2::LEFT_CENTER,
                text,
                egui::TextStyle::Monospace.resolve(&style),
                Color32::GRAY,
            );
        }
    }
}

/// Get the bounding box center and size of an element (for hit testing).
pub fn element_bounds(element: &SchematicElement) -> (Pos2, Vec2) {
    match element {
        SchematicElement::Wire { from, to } => {
            let cx = (from.x + to.x) / 2.0;
            let cy = (from.y + to.y) / 2.0;
            let w = (from.x - to.x).abs() + 10.0;
            let h = (from.y - to.y).abs() + 10.0;
            (Pos2::new(cx, cy), Vec2::new(w.max(20.0), h.max(20.0)))
        }
        SchematicElement::Source { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(40.0, 40.0)),
        SchematicElement::Resistor { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(50.0, 24.0)),
        SchematicElement::Inductor { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(52.0, 32.0)),
        SchematicElement::Capacitor { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(32.0, 40.0)),
        SchematicElement::Diode { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(36.0, 24.0)),
        SchematicElement::Switch { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(32.0, 24.0)),
        SchematicElement::Ground { pos } => (Pos2::new(pos.x, pos.y), Vec2::new(24.0, 20.0)),
        SchematicElement::Node { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(12.0, 12.0)),
        SchematicElement::Label { pos, .. } => (Pos2::new(pos.x, pos.y), Vec2::new(60.0, 16.0)),
    }
}

/// Check if a point (in canvas coordinates) hits an element.
pub fn hit_test(element: &SchematicElement, point: Pos2) -> bool {
    let (center, size) = element_bounds(element);
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;
    point.x >= center.x - half_w
        && point.x <= center.x + half_w
        && point.y >= center.y - half_h
        && point.y <= center.y + half_h
}
