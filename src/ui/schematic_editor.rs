/// Interactive schematic editor — a free-draw canvas for electrical schematics.
///
/// Features:
/// - Tool palette with 10 component types
/// - Drag-to-move elements + pan canvas
/// - Snap-to-grid (toggle)
/// - Double-click to edit component labels/values
/// - Scroll-to-zoom
/// - Orthogonal (90°) wire routing
/// - Copy/paste (Ctrl+C/V)
/// - Properties panel for selected element

use crate::app::state::{AppState, SchematicTool};
use crate::schematic::primitives::{Pos, SchematicElement};
use crate::schematic::renderer::{draw_element, hit_test};
use egui::{Color32, Pos2, Stroke, Ui};

/// Grid spacing in element-space (not screen-space).
const GRID_SPACING: f32 = 40.0;

/// Show the schematic editor tab.
///
/// Note: we pass `state` but borrow `state.editor` only in scoped blocks
/// to satisfy the borrow checker (no long-lived `&mut state.editor`).
pub fn show_schematic_editor(ui: &mut Ui, state: &mut AppState) {
    // ── Tool palette (top) ──
    draw_toolbar(ui, state);

    // ── Properties panel (right, shown when element selected) ──
    let show_props = {
        let e = &state.editor;
        e.selected_element.is_some() && e.selected_tool == SchematicTool::Select
    };
    if show_props {
        egui::Panel::right("properties_panel")
            .resizable(true)
            .default_size(180.0)
            .min_size(150.0)
            .show_inside(ui, |ui| {
                show_properties_panel(ui, state);
            });
    }

    // ── Canvas (center) ──
    egui::CentralPanel::default().show_inside(ui, |ui| {
        handle_canvas(ui, state);
    });

    // ── Edit component popup ──
    let editing_idx = state.editor.editing_element;
    if let Some(edit_idx) = editing_idx {
        if edit_idx < state.editor.elements.len() {
            show_edit_popup(ui, state, edit_idx);
        } else {
            state.editor.editing_element = None;
        }
    }

    // ── Label text input popup ──
    if state.editor.typing_label {
        show_label_popup(ui, state);
    }
}

// ── Toolbar ──────────────────────────────────────────────────────────

fn draw_toolbar(ui: &mut Ui, state: &mut AppState) {
    let editor = &mut state.editor;

    egui::Panel::top("editor_toolbar")
        .min_size(40.0)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let tools = [
                    (SchematicTool::Select, "🖱 Select"),
                    (SchematicTool::Source, "🔋 Source"),
                    (SchematicTool::Resistor, "⬛ Res"),
                    (SchematicTool::Inductor, "〰 Ind"),
                    (SchematicTool::Capacitor, "‖‖ Cap"),
                    (SchematicTool::Diode, "▶ Diode"),
                    (SchematicTool::Switch, "⚡ SW"),
                    (SchematicTool::Ground, "⏚ GND"),
                    (SchematicTool::Wire, "〰 Wire"),
                    (SchematicTool::Label, "Aa Label"),
                ];

                for (tool, label) in &tools {
                    let selected = editor.selected_tool == *tool;
                    if ui.selectable_label(selected, *label).clicked() {
                        editor.selected_tool = *tool;
                        if *tool != SchematicTool::Wire {
                            editor.wire_start = None;
                        }
                        editor.typing_label = false;
                    }
                }

                ui.separator();

                if ui.button("🗑 Delete").clicked() { editor.delete_selected(); }
                if ui.button("🧹 Clear all").clicked() { editor.clear(); }

                ui.separator();

                ui.checkbox(&mut editor.snap_to_grid, "Snap");
                ui.checkbox(&mut editor.orthogonal_wires, "90°");

                ui.separator();

                if ui.button("🔍+").clicked() {
                    editor.zoom = (editor.zoom * 1.2).min(5.0);
                }
                if ui.button("🔍-").clicked() {
                    editor.zoom = (editor.zoom / 1.2).max(0.2);
                }
                ui.label(format!("{:.0}%", editor.zoom * 100.0));

                if ui.button("⟲ Reset view").clicked() {
                    editor.pan_offset = (0.0, 0.0);
                    editor.zoom = 1.0;
                }
            });
        });
}

// ── Canvas ────────────────────────────────────────────────────────────

fn handle_canvas(ui: &mut Ui, state: &mut AppState) {
    let available = ui.available_size();
    let (response, painter) = ui.allocate_painter(available, egui::Sense::click_and_drag());

    let (origin, zoom) = {
        let e = &state.editor;
        let o = Pos2::new(
            response.rect.min.x + e.pan_offset.0 * e.zoom,
            response.rect.min.y + e.pan_offset.1 * e.zoom,
        );
        (o, e.zoom)
    };

    // ── Keyboard shortcuts (read-only snapshot of editor state first) ──
    let selected_idx = state.editor.selected_element;
    let has_clipboard = state.editor.clipboard.is_some();
    let typing = state.editor.typing_label;

    ui.input(|i| {
        // Copy
        if i.key_pressed(egui::Key::C) && i.modifiers.ctrl {
            if let Some(idx) = selected_idx {
                if idx < state.editor.elements.len() {
                    state.editor.clipboard = Some(Box::new(state.editor.elements[idx].clone()));
                    state.status_message = "Copied".to_owned();
                }
            }
        }
        // Paste
        if i.key_pressed(egui::Key::V) && i.modifiers.ctrl && has_clipboard {
            if let Some(ref clip) = state.editor.clipboard.clone() {
                let mut new_elem = clip.as_ref().clone();
                offset_element(&mut new_elem, 30.0, 30.0);
                state.editor.elements.push(new_elem);
                state.editor.selected_element = Some(state.editor.elements.len() - 1);
                state.status_message = "Pasted".to_owned();
            }
        }
        // Delete
        if (i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) && !typing {
            // Borrow checker workaround: we can't call delete_selected inside input()
            // So we set a flag
        }
    });

    // Handle delete outside input() closure
    let delete_pressed = ui.input(|i| {
        (i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) && !typing
    });
    if delete_pressed {
        state.editor.delete_selected();
        ui.ctx().request_repaint();
    }

    // ── Scroll-to-zoom ──
    let scroll_delta = ui.input(|i| i.smooth_scroll_delta().y);
    if scroll_delta != 0.0 {
        if let Some(cursor) = response.hover_pos() {
            let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            let new_zoom = (state.editor.zoom * zoom_factor).clamp(0.2, 5.0);
            if (new_zoom - state.editor.zoom).abs() > 0.001 {
                let cursor_rel = Pos2::new(
                    cursor.x - response.rect.min.x,
                    cursor.y - response.rect.min.y,
                );
                let world_before = Pos2::new(
                    (cursor_rel.x - state.editor.pan_offset.0) / state.editor.zoom,
                    (cursor_rel.y - state.editor.pan_offset.1) / state.editor.zoom,
                );
                state.editor.zoom = new_zoom;
                state.editor.pan_offset.0 = cursor_rel.x - world_before.x * new_zoom;
                state.editor.pan_offset.1 = cursor_rel.y - world_before.y * new_zoom;
                ui.ctx().request_repaint();
            }
        } else {
            state.editor.zoom = (state.editor.zoom * if scroll_delta > 0.0 { 1.1 } else { 0.9 })
                .clamp(0.2, 5.0);
            ui.ctx().request_repaint();
        }
    }

    // ── Handle mouse input ──
    let mut need_repaint = false;

    // Right-click: delete element under cursor
    if response.clicked_by(egui::PointerButton::Secondary) {
        if let Some(cursor) = response.interact_pointer_pos() {
            let canvas_pos = canvas_to_element(cursor, origin, zoom);
            // Scope borrow of state.editor briefly
            let hit = {
                find_element_at(&state.editor.elements, canvas_pos)
            };
            if let Some(idx) = hit {
                state.editor.selected_element = Some(idx);
                state.editor.delete_selected();
                need_repaint = true;
            }
        }
    }

    // Double-click: edit element properties
    if response.double_clicked_by(egui::PointerButton::Primary) {
        if let Some(cursor) = response.interact_pointer_pos() {
            let canvas_pos = canvas_to_element(cursor, origin, zoom);
            let hit = find_element_at(&state.editor.elements, canvas_pos);
            if let Some(idx) = hit {
                let can_edit = match &state.editor.elements[idx] {
                    SchematicElement::Source { .. }
                    | SchematicElement::Resistor { .. }
                    | SchematicElement::Inductor { .. }
                    | SchematicElement::Capacitor { .. }
                    | SchematicElement::Diode { .. }
                    | SchematicElement::Switch { .. }
                    | SchematicElement::Label { .. } => true,
                    _ => false,
                };
                if can_edit {
                    state.editor.editing_element = Some(idx);
                    state.editor.selected_element = Some(idx);
                    need_repaint = true;
                }
            }
        }
    }

    // Selection + drag + place
    if let Some(cursor) = response.interact_pointer_pos() {
        let canvas_pos = canvas_to_element(cursor, origin, zoom);

        match state.editor.selected_tool {
            SchematicTool::Select => {
                if response.dragged_by(egui::PointerButton::Primary) {
                    let delta = response.drag_delta();
                    if state.editor.selected_element.is_some()
                        && !drag_started_on_empty(&response, &state.editor.elements, origin, zoom)
                    {
                        // Drag-to-move
                        let snap_grid = if state.editor.snap_to_grid { GRID_SPACING } else { 0.0 };
                        let dx = delta.x / zoom;
                        let dy = delta.y / zoom;
                        move_selected_element(&mut state.editor, dx, dy, snap_grid);
                        need_repaint = true;
                    } else {
                        // Pan
                        state.editor.pan_offset.0 += delta.x;
                        state.editor.pan_offset.1 += delta.y;
                        need_repaint = true;
                    }
                }
                if response.clicked_by(egui::PointerButton::Primary) {
                    let hit = find_element_at(&state.editor.elements, canvas_pos);
                    state.editor.selected_element = hit;
                    if hit.is_none() {
                        state.editor.editing_element = None;
                    }
                    need_repaint = true;
                }
            }
            SchematicTool::Wire => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let pos = if state.editor.snap_to_grid { snap(canvas_pos, GRID_SPACING) } else { canvas_pos };
                    if let Some(start) = state.editor.wire_start {
                        if state.editor.orthogonal_wires {
                            route_orthogonal_wire(&mut state.editor.elements, start, pos);
                        } else {
                            state.editor.elements.push(SchematicElement::Wire { from: start, to: pos });
                        }
                        state.editor.wire_start = None;
                        state.status_message = "Wire placed".to_owned();
                    } else {
                        state.editor.wire_start = Some(pos);
                        state.status_message = "Click second point to complete wire".to_owned();
                    }
                    need_repaint = true;
                }
            }
            SchematicTool::Source => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let snap = state.editor.snap_to_grid;
                    let pos = snap_if(canvas_pos, snap);
                    state.editor.elements.push(SchematicElement::Source {
                        pos, label: "V".to_owned(), value: "".to_owned(),
                    });
                    state.status_message = "Source placed".to_owned();
                    need_repaint = true;
                }
            }
            SchematicTool::Resistor => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let snap = state.editor.snap_to_grid;
                    let pos = snap_if(canvas_pos, snap);
                    state.editor.elements.push(SchematicElement::Resistor {
                        pos, label: "R".to_owned(), value: "".to_owned(),
                    });
                    state.status_message = "Resistor placed".to_owned();
                    need_repaint = true;
                }
            }
            SchematicTool::Inductor => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let snap = state.editor.snap_to_grid;
                    let pos = snap_if(canvas_pos, snap);
                    state.editor.elements.push(SchematicElement::Inductor {
                        pos, label: "L".to_owned(), value: "".to_owned(),
                    });
                    state.status_message = "Inductor placed".to_owned();
                    need_repaint = true;
                }
            }
            SchematicTool::Capacitor => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let snap = state.editor.snap_to_grid;
                    let pos = snap_if(canvas_pos, snap);
                    state.editor.elements.push(SchematicElement::Capacitor {
                        pos, label: "C".to_owned(), value: "".to_owned(),
                    });
                    state.status_message = "Capacitor placed".to_owned();
                    need_repaint = true;
                }
            }
            SchematicTool::Diode => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let snap = state.editor.snap_to_grid;
                    let pos = snap_if(canvas_pos, snap);
                    state.editor.elements.push(SchematicElement::Diode {
                        pos, label: "D".to_owned(),
                    });
                    state.status_message = "Diode placed".to_owned();
                    need_repaint = true;
                }
            }
            SchematicTool::Switch => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let snap = state.editor.snap_to_grid;
                    let pos = snap_if(canvas_pos, snap);
                    state.editor.elements.push(SchematicElement::Switch {
                        pos, label: "SW".to_owned(),
                    });
                    state.status_message = "Switch placed".to_owned();
                    need_repaint = true;
                }
            }
            SchematicTool::Ground => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    let snap = state.editor.snap_to_grid;
                    let pos = snap_if(canvas_pos, snap);
                    state.editor.elements.push(SchematicElement::Ground { pos });
                    state.status_message = "Ground placed".to_owned();
                    need_repaint = true;
                }
            }
            SchematicTool::Label => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    if state.editor.typing_label {
                        if !state.editor.pending_label_text.is_empty() {
                            let pos = snap_if(canvas_pos, state.editor.snap_to_grid);
                            state.editor.elements.push(SchematicElement::Label {
                                pos, text: state.editor.pending_label_text.clone(),
                            });
                        }
                        state.editor.typing_label = false;
                        state.editor.pending_label_text.clear();
                    } else {
                        state.editor.typing_label = true;
                        state.editor.pending_label_text.clear();
                        state.status_message = "Type label text and click again".to_owned();
                    }
                    need_repaint = true;
                }
            }
        }
    }

    // ── Draw grid ──
    draw_grid(&painter, origin, response.rect, zoom);

    // ── Draw wire preview ──
    if state.editor.selected_tool == SchematicTool::Wire {
        if let Some(start) = state.editor.wire_start {
            if let Some(cursor) = response.hover_pos() {
                let canvas_cursor = canvas_to_element(cursor, origin, zoom);
                let sx = origin.x + start.x;
                let sy = origin.y + start.y;
                let cx = origin.x + canvas_cursor.x;
                let cy = origin.y + canvas_cursor.y;

                if state.editor.orthogonal_wires {
                    let mid_x = (start.x + canvas_cursor.x) / 2.0;
                    let m1 = Pos2::new(origin.x + mid_x, sy);
                    let m2 = Pos2::new(origin.x + mid_x, cy);
                    painter.line_segment([Pos2::new(sx, sy), m1], Stroke::new(1.0, Color32::LIGHT_GREEN));
                    painter.line_segment([m1, m2], Stroke::new(1.0, Color32::LIGHT_GREEN));
                    painter.line_segment([m2, Pos2::new(cx, cy)], Stroke::new(1.0, Color32::LIGHT_GREEN));
                } else {
                    painter.line_segment(
                        [Pos2::new(sx, sy), Pos2::new(cx, cy)],
                        Stroke::new(1.5, Color32::LIGHT_GREEN),
                    );
                }
            }
        }
    }

    // ── Draw all elements ──
    for (idx, element) in state.editor.elements.iter().enumerate() {
        let is_selected = state.editor.selected_element == Some(idx);
        draw_element(&painter, element, origin, is_selected);
    }

    // ── Snap indicator ──
    if state.editor.snap_to_grid && state.editor.selected_tool != SchematicTool::Select {
        if let Some(cursor) = response.hover_pos() {
            let cp = canvas_to_element(cursor, origin, zoom);
            let snapped = snap(cp, GRID_SPACING);
            painter.circle_filled(
                Pos2::new(origin.x + snapped.x, origin.y + snapped.y),
                2.5,
                Color32::YELLOW,
            );
        }
    }

    // ── Bottom status bar ──
    {
        let e = &state.editor;
        let mode_info = if e.snap_to_grid { "Snap ON" } else { "Snap OFF" };
        let wire_info = if e.orthogonal_wires { "90°" } else { "Direct" };
        let style = egui::Style::default();
        painter.text(
            Pos2::new(response.rect.min.x + 8.0, response.rect.max.y - 12.0),
            egui::Align2::LEFT_BOTTOM,
            format!(
                "Tool: {} | {} elem | {:.0}% | {} | {}   (Scroll=zoom, RClick=del, DCtrl=edit)",
                e.selected_tool.name(), e.elements.len(), e.zoom * 100.0, mode_info, wire_info,
            ),
            egui::TextStyle::Monospace.resolve(&style),
            Color32::GRAY,
        );
    }

    if need_repaint {
        ui.ctx().request_repaint();
    }
}

// ── Place helper ─────────────────────────────────────────────────────

fn snap_if(pos: Pos, enabled: bool) -> Pos {
    if enabled { snap(pos, GRID_SPACING) } else { pos }
}

// ── Properties Panel ────────────────────────────────────────────────

fn show_properties_panel(ui: &mut Ui, state: &mut AppState) {
    ui.heading("Properties");
    ui.separator();

    let idx = match state.editor.selected_element {
        Some(i) if i < state.editor.elements.len() => i,
        _ => return,
    };

    let elem = &state.editor.elements[idx];
    let type_name = match elem {
        SchematicElement::Source { .. } => "Voltage Source",
        SchematicElement::Resistor { .. } => "Resistor",
        SchematicElement::Inductor { .. } => "Inductor",
        SchematicElement::Capacitor { .. } => "Capacitor",
        SchematicElement::Diode { .. } => "Diode",
        SchematicElement::Switch { .. } => "Switch",
        SchematicElement::Ground { .. } => "Ground",
        SchematicElement::Wire { .. } => "Wire",
        SchematicElement::Node { .. } => "Node",
        SchematicElement::Label { .. } => "Label",
    };

    ui.label(format!("Type: {}", type_name));
    let pos = element_position(elem);
    ui.label(format!("Pos: ({:.0}, {:.0})", pos.x, pos.y));

    // Show label/value if applicable
    match elem {
        SchematicElement::Source { label, value, .. }
        | SchematicElement::Resistor { label, value, .. }
        | SchematicElement::Inductor { label, value, .. }
        | SchematicElement::Capacitor { label, value, .. } => {
            ui.label(format!("Label: {}", label));
            if !value.is_empty() {
                ui.label(format!("Value: {}", value));
            }
        }
        SchematicElement::Diode { label, .. } | SchematicElement::Switch { label, .. } => {
            ui.label(format!("Label: {}", label));
        }
        SchematicElement::Label { text, .. } => {
            ui.label(format!("Text: {}", text));
        }
        _ => {}
    }

    ui.add_space(8.0);
    if ui.button("✏️ Edit").clicked() {
        state.editor.editing_element = Some(idx);
    }
}

// ── Edit Popup ────────────────────────────────────────────────────────

fn show_edit_popup(ui: &mut Ui, state: &mut AppState, idx: usize) {
    let elem = &mut state.editor.elements[idx];
    let title = match elem {
        SchematicElement::Source { .. } => "Edit Source",
        SchematicElement::Resistor { .. } => "Edit Resistor",
        SchematicElement::Inductor { .. } => "Edit Inductor",
        SchematicElement::Capacitor { .. } => "Edit Capacitor",
        SchematicElement::Diode { .. } => "Edit Diode",
        SchematicElement::Switch { .. } => "Edit Switch",
        SchematicElement::Label { .. } => "Edit Label",
        _ => "Edit Component",
    };

    let mut close = false;

    egui::Window::new(title)
        .anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            match elem {
                SchematicElement::Source { ref mut label, ref mut value, .. }
                | SchematicElement::Resistor { ref mut label, ref mut value, .. }
                | SchematicElement::Inductor { ref mut label, ref mut value, .. }
                | SchematicElement::Capacitor { ref mut label, ref mut value, .. } => {
                    ui.horizontal(|ui| { ui.label("Label:"); ui.text_edit_singleline(label); });
                    ui.horizontal(|ui| { ui.label("Value:"); ui.text_edit_singleline(value); });
                }
                SchematicElement::Diode { ref mut label, .. }
                | SchematicElement::Switch { ref mut label, .. } => {
                    ui.horizontal(|ui| { ui.label("Label:"); ui.text_edit_singleline(label); });
                }
                SchematicElement::Label { ref mut text, .. } => {
                    ui.horizontal(|ui| { ui.label("Text:"); ui.text_edit_singleline(text); });
                }
                _ => { ui.label("No editable properties."); }
            }
            ui.add_space(8.0);
            if ui.button("Close").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                close = true;
            }
        });

    if close {
        state.editor.editing_element = None;
        state.status_message = "Properties updated".to_owned();
    }
}

// ── Label Popup ───────────────────────────────────────────────────────

fn show_label_popup(ui: &mut Ui, state: &mut AppState) {
    egui::Window::new("Label Text")
        .anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            ui.label("Enter the label text:");
            ui.text_edit_singleline(&mut state.editor.pending_label_text);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    state.editor.typing_label = false;
                    state.status_message = "Click on canvas to place the label".to_owned();
                }
                if ui.button("Cancel").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    state.editor.typing_label = false;
                    state.editor.pending_label_text.clear();
                    state.status_message = "Label cancelled".to_owned();
                }
            });
        });
}

// ── Helpers ───────────────────────────────────────────────────────────

fn snap(pos: Pos, grid: f32) -> Pos {
    Pos::new(
        (pos.x / grid).round() * grid,
        (pos.y / grid).round() * grid,
    )
}

fn canvas_to_element(screen_pos: Pos2, origin: Pos2, zoom: f32) -> Pos {
    Pos::new(
        (screen_pos.x - origin.x) / zoom,
        (screen_pos.y - origin.y) / zoom,
    )
}

fn find_element_at(elements: &[SchematicElement], point: Pos) -> Option<usize> {
    let p = Pos2::new(point.x, point.y);
    for (idx, element) in elements.iter().enumerate().rev() {
        if hit_test(element, p) {
            return Some(idx);
        }
    }
    None
}

fn element_position(elem: &SchematicElement) -> Pos {
    match elem {
        SchematicElement::Source { pos, .. } | SchematicElement::Resistor { pos, .. }
        | SchematicElement::Inductor { pos, .. } | SchematicElement::Capacitor { pos, .. }
        | SchematicElement::Diode { pos, .. } | SchematicElement::Switch { pos, .. }
        | SchematicElement::Ground { pos } | SchematicElement::Node { pos, .. }
        | SchematicElement::Label { pos, .. } => *pos,
        SchematicElement::Wire { from, .. } => *from,
    }
}

fn move_selected_element(
    editor: &mut crate::app::state::SchematicEditorState,
    dx: f32, dy: f32, snap_grid: f32,
) {
    let idx = match editor.selected_element {
        Some(i) => i,
        None => return,
    };
    if idx >= editor.elements.len() { return; }

    let elem = &mut editor.elements[idx];
    let apply = |pos: &mut Pos| {
        let nx = pos.x + dx;
        let ny = pos.y + dy;
        if snap_grid > 0.0 {
            pos.x = (nx / snap_grid).round() * snap_grid;
            pos.y = (ny / snap_grid).round() * snap_grid;
        } else {
            pos.x = nx;
            pos.y = ny;
        }
    };

    match elem {
        SchematicElement::Source { ref mut pos, .. }
        | SchematicElement::Resistor { ref mut pos, .. }
        | SchematicElement::Inductor { ref mut pos, .. }
        | SchematicElement::Capacitor { ref mut pos, .. }
        | SchematicElement::Diode { ref mut pos, .. }
        | SchematicElement::Switch { ref mut pos, .. }
        | SchematicElement::Ground { ref mut pos }
        | SchematicElement::Node { ref mut pos, .. }
        | SchematicElement::Label { ref mut pos, .. } => apply(pos),
        SchematicElement::Wire { ref mut from, ref mut to } => {
            apply(from);
            apply(to);
        }
    }
}

fn offset_element(elem: &mut SchematicElement, dx: f32, dy: f32) {
    match elem {
        SchematicElement::Source { ref mut pos, .. }
        | SchematicElement::Resistor { ref mut pos, .. }
        | SchematicElement::Inductor { ref mut pos, .. }
        | SchematicElement::Capacitor { ref mut pos, .. }
        | SchematicElement::Diode { ref mut pos, .. }
        | SchematicElement::Switch { ref mut pos, .. }
        | SchematicElement::Ground { ref mut pos }
        | SchematicElement::Node { ref mut pos, .. }
        | SchematicElement::Label { ref mut pos, .. } => {
            pos.x += dx; pos.y += dy;
        }
        SchematicElement::Wire { ref mut from, ref mut to } => {
            from.x += dx; from.y += dy;
            to.x += dx; to.y += dy;
        }
    }
}

fn route_orthogonal_wire(elements: &mut Vec<SchematicElement>, from: Pos, to: Pos) {
    let mid_x = (from.x + to.x) / 2.0;
    elements.push(SchematicElement::Wire { from, to: Pos::new(mid_x, from.y) });
    elements.push(SchematicElement::Wire { from: Pos::new(mid_x, from.y), to: Pos::new(mid_x, to.y) });
    elements.push(SchematicElement::Wire { from: Pos::new(mid_x, to.y), to });
}

fn draw_grid(painter: &egui::Painter, origin: Pos2, rect: egui::Rect, zoom: f32) {
    let gs = GRID_SPACING * zoom;
    let gc = Color32::from_rgba_premultiplied(80, 80, 80, 30);
    let gx = (origin.x % gs) - gs;
    let gy = (origin.y % gs) - gs;
    let mut x = gx;
    while x < rect.max.x {
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], Stroke::new(0.5, gc));
        x += gs;
    }
    let mut y = gy;
    while y < rect.max.y {
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], Stroke::new(0.5, gc));
        y += gs;
    }
}

fn drag_started_on_empty(
    response: &egui::Response,
    elements: &[SchematicElement],
    origin: Pos2, zoom: f32,
) -> bool {
    if let Some(cursor) = response.interact_pointer_pos() {
        let cp = canvas_to_element(cursor, origin, zoom);
        find_element_at(elements, cp).is_none()
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_to_element() {
        let r = canvas_to_element(Pos2::new(110.0, 210.0), Pos2::new(10.0, 10.0), 2.0);
        assert!((r.x - 50.0).abs() < 0.001);
        assert!((r.y - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_find_element_at() {
        let elements = vec![
            SchematicElement::Resistor { pos: Pos::new(50.0, 50.0), label: "R1".to_owned(), value: "100".to_owned() },
            SchematicElement::Ground { pos: Pos::new(100.0, 100.0) },
        ];
        assert!(find_element_at(&elements, Pos::new(50.0, 50.0)).is_some());
        assert!(find_element_at(&elements, Pos::new(0.0, 0.0)).is_none());
    }

    #[test]
    fn test_snap() {
        let s = snap(Pos::new(47.0, 73.0), 40.0);
        assert!((s.x - 40.0).abs() < 0.001);
        assert!((s.y - 80.0).abs() < 0.001);
    }

    #[test]
    fn test_route_orthogonal_wire() {
        let mut elements = Vec::new();
        route_orthogonal_wire(&mut elements, Pos::new(0.0, 0.0), Pos::new(100.0, 100.0));
        assert_eq!(elements.len(), 3);
    }

    #[test]
    fn test_offset_element() {
        let mut elem = SchematicElement::Resistor { pos: Pos::new(10.0, 20.0), label: "R".to_owned(), value: "".to_owned() };
        offset_element(&mut elem, 30.0, 40.0);
        if let SchematicElement::Resistor { pos, .. } = &elem {
            assert!((pos.x - 40.0).abs() < 0.001);
            assert!((pos.y - 60.0).abs() < 0.001);
        }
    }
}
