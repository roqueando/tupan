use crate::app::state::{AppState, CanvasComponentType};
use crate::schematic::primitives::Pos;
use egui::{Align2, Color32, CornerRadius, Pos2, Rect, Stroke, Ui, Vec2};
use egui_plot::{Legend, Line, Plot, PlotPoints};

/// Size of a placed component block on the canvas (in element-space).
/// Taller blocks for editable components to fit slider + input.
const BLOCK_W: f32 = 180.0;
const BLOCK_H: f32 = 90.0;
const PLOT_BLOCK_W: f32 = 280.0;
const PLOT_BLOCK_H: f32 = 200.0;

/// Grid spacing in element-space.
const GRID_SPACING: f32 = 40.0;

/// Main entry point for the Component Canvas tab.
pub fn show_component_canvas(ui: &mut Ui, state: &mut AppState) {
    // ── Sidebar (left) with palette and params ──
    egui::Panel::left("canvas_sidebar")
        .resizable(true)
        .default_size(200.0)
        .min_size(160.0)
        .show_inside(ui, |ui| {
            draw_sidebar(ui, state);
        });

    // ── Canvas (center) ──
    egui::CentralPanel::default().show_inside(ui, |ui| {
        handle_canvas(ui, state);
    });

    // ── Keyboard shortcuts ──
    handle_keyboard(ui, state);
}

// ── Sidebar ──────────────────────────────────────────────────────────

fn draw_sidebar(ui: &mut Ui, state: &mut AppState) {
    let cc = &mut state.component_canvas;

    ui.heading("🧩 Components");
    ui.separator();

    // Palette of components to place on canvas
    let palette_items = [
        (CanvasComponentType::Vin, "⚡ Vin"),
        (CanvasComponentType::Vout, "🔌 Vout"),
        (CanvasComponentType::DutyCycle, "〰 Duty Cycle"),
        (CanvasComponentType::Frequency, "📡 Frequency"),
        (CanvasComponentType::DeltaIl, "📉 ΔiL"),
        (CanvasComponentType::IoutMax, "💧 Iout,max"),
        (CanvasComponentType::DeltaVo, "📊 ΔVo"),
    ];

    for (ctype, label) in &palette_items {
        let selected = cc.palette_selection == Some(*ctype);
        if ui.selectable_label(selected, *label).clicked() {
            cc.palette_selection = Some(*ctype);
            state.status_message = format!("Click on canvas to place {}", ctype.name());
        }
    }

    // Computed components (read-only, can still be placed)
    ui.add_space(8.0);
    ui.separator();
    ui.label("Computed (drag to canvas):");

    let computed_items = [
        (CanvasComponentType::Inductor, "〰 Inductor (L)"),
        (CanvasComponentType::Capacitor, "‖‖ Capacitor (C)"),
    ];

    for (ctype, label) in &computed_items {
        let selected = cc.palette_selection == Some(*ctype);
        if ui.selectable_label(selected, *label).clicked() {
            cc.palette_selection = Some(*ctype);
            state.status_message = format!("Click on canvas to place {}", ctype.name());
        }
    }

    // Plot component
    ui.add_space(8.0);
    ui.separator();
    ui.label("Visualization:");
    let plot_selected = cc.palette_selection == Some(CanvasComponentType::Plot);
    if ui.selectable_label(plot_selected, "📈 Curve Plot").clicked() {
        cc.palette_selection = Some(CanvasComponentType::Plot);
        state.status_message = "Click on canvas to place a Plot".to_owned();
    }

    // Quick-params panel (edit shared params directly)
    ui.add_space(12.0);
    ui.separator();
    ui.heading("⚙ Quick Params");
    ui.separator();

    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Vin:");
        if ui
            .add(
                egui::DragValue::new(&mut cc.shared_params.vin)
                    .speed(0.5)
                    .suffix(" V"),
            )
            .changed()
        {
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Vout:");
        if ui
            .add(
                egui::DragValue::new(&mut cc.shared_params.vout)
                    .speed(0.5)
                    .suffix(" V"),
            )
            .changed()
        {
            // Recalculate duty cycle when Vout changes
            if cc.shared_params.vin > 0.0 {
                cc.shared_params.duty_cycle =
                    (cc.shared_params.vout / cc.shared_params.vin).clamp(0.0, 1.0);
            }
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("D:");
        let mut dc_pct = cc.shared_params.duty_cycle * 100.0;
        if ui
            .add(
                egui::DragValue::new(&mut dc_pct)
                    .speed(0.5)
                    .suffix(" %"),
            )
            .changed()
        {
            cc.shared_params.duty_cycle = (dc_pct / 100.0).clamp(0.0, 1.0);
            cc.shared_params.vout = cc.shared_params.vin * cc.shared_params.duty_cycle;
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Freq:");
        if ui
            .add(
                egui::DragValue::new(&mut cc.shared_params.frequency)
                    .speed(1000.0)
                    .suffix(" Hz"),
            )
            .changed()
        {
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("ΔiL:");
        let mut pct = cc.shared_params.delta_il * 100.0;
        if ui
            .add(
                egui::DragValue::new(&mut pct)
                    .speed(0.5)
                    .suffix(" %"),
            )
            .changed()
        {
            cc.shared_params.delta_il = (pct / 100.0).max(0.001);
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Iout,max:");
        if ui
            .add(
                egui::DragValue::new(&mut cc.shared_params.iout_max)
                    .speed(0.2)
                    .suffix(" A"),
            )
            .changed()
        {
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("ΔVo:");
        let mut pct = cc.shared_params.delta_vo * 100.0;
        if ui
            .add(
                egui::DragValue::new(&mut pct)
                    .speed(0.1)
                    .suffix(" %"),
            )
            .changed()
        {
            cc.shared_params.delta_vo = (pct / 100.0).max(0.0001);
            changed = true;
        }
    });

    // Show computed values
    ui.add_space(12.0);
    ui.separator();
    ui.heading("📐 Computed");
    ui.separator();

    let l_val = cc.shared_params.calc_inductance();
    let c_val = cc.shared_params.calc_capacitance();
    let delta_il_amps = cc.shared_params.calc_delta_il_amps();
    let delta_il_pct = cc.shared_params.delta_il * 100.0;

    ui.label(format!(
        "L = {}",
        format_eng_small(l_val, "H")
    ));
    ui.label(format!(
        "C = {}",
        format_eng_small(c_val, "F")
    ));
    ui.label(format!(
        "ΔiL = {:.1}% ({:.3} A)",
        delta_il_pct, delta_il_amps
    ));
    // Also show what L formula used
    let l_formula_val = if cc.shared_params.delta_il > 0.0 && cc.shared_params.frequency > 0.0 && cc.shared_params.iout_max > 0.0 {
        let delta_il_a = cc.shared_params.delta_il * cc.shared_params.iout_max;
        (cc.shared_params.vout * (1.0 - cc.shared_params.duty_cycle)) / (delta_il_a * cc.shared_params.frequency)
    } else { 0.0 };
    ui.label(format!(
        "L = Vout(1-D) / (ΔiL_A·f) = {:.6} H",
        l_formula_val
    ));

    // Canvas control buttons
    ui.add_space(12.0);
    ui.separator();
    if ui.button("🧹 Clear canvas").clicked() {
        cc.clear();
        state.status_message = "Canvas cleared".to_owned();
    }
    if ui.button("⟲ Reset view").clicked() {
        cc.pan_offset = (0.0, 0.0);
        cc.zoom = 1.0;
        state.status_message = "View reset".to_owned();
    }

    if changed {
        state.status_message = "Parameters updated — placed L/C blocks recomputed".to_owned();
    }
}

// ── Canvas ────────────────────────────────────────────────────────────

fn handle_canvas(ui: &mut Ui, state: &mut AppState) {
    let available = ui.available_size();
    let (response, painter) = ui.allocate_painter(available, egui::Sense::click_and_drag());

    // ── First, draw plot components as actual egui widgets (they need full ui context) ──
    // We'll render plots separately from the painter-based rendering.
    // Actually, plots need Ui, not Painter. So we handle them in a separate pass.

    let cc = &mut state.component_canvas;

    let origin = Pos2::new(
        response.rect.min.x + cc.pan_offset.0,
        response.rect.min.y + cc.pan_offset.1,
    );
    let zoom = cc.zoom;

    // ── Scroll-to-zoom ──
    let scroll_delta = ui.input(|i| i.smooth_scroll_delta().y);
    if scroll_delta != 0.0 {
        if let Some(cursor) = response.hover_pos() {
            let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            let new_zoom = (cc.zoom * zoom_factor).clamp(0.2, 5.0);
            if (new_zoom - cc.zoom).abs() > 0.001 {
                let cursor_rel = cursor - response.rect.min.to_vec2();
                let world_before = Vec2::new(
                    (cursor_rel.x - cc.pan_offset.0) / cc.zoom,
                    (cursor_rel.y - cc.pan_offset.1) / cc.zoom,
                );
                cc.zoom = new_zoom;
                cc.pan_offset.0 = cursor_rel.x - world_before.x * new_zoom;
                cc.pan_offset.1 = cursor_rel.y - world_before.y * new_zoom;
                ui.ctx().request_repaint();
            }
        } else {
            cc.zoom = (cc.zoom * if scroll_delta > 0.0 { 1.1 } else { 0.9 }).clamp(0.2, 5.0);
            ui.ctx().request_repaint();
        }
    }

    // ── Drag to pan (when no component is selected/dragged) ──
    if response.dragged_by(egui::PointerButton::Primary) && cc.selected_index.is_none() {
        let delta = response.drag_delta();
        cc.pan_offset.0 += delta.x;
        cc.pan_offset.1 += delta.y;
        ui.ctx().request_repaint();
    }

    // ── Click to place/select ──
    if response.clicked_by(egui::PointerButton::Primary) {
        if let Some(cursor) = response.interact_pointer_pos() {
            let canvas_pos = screen_to_element(cursor, origin, zoom);

            // Check if we clicked on an existing component
            let hit = find_component_at(&cc.placed_components, canvas_pos);

            if let Some(idx) = hit {
                // If we have a palette selection, deselect it
                cc.palette_selection = None;
                cc.selected_index = Some(idx);
                state.status_message = format!("Selected {}", cc.placed_components[idx].component_type.name());
            } else if let Some(ctype) = cc.palette_selection {
                // Place a new component
                let pos = snap(canvas_pos, GRID_SPACING);
                cc.place_component(ctype, pos);
                cc.palette_selection = None;
                state.status_message = format!("Placed {}", ctype.name());
            } else {
                // Deselect
                cc.selected_index = None;
            }
            ui.ctx().request_repaint();
        }
    }

    // ── Right-click to delete ──
    if response.clicked_by(egui::PointerButton::Secondary) {
        if let Some(cursor) = response.interact_pointer_pos() {
            let canvas_pos = screen_to_element(cursor, origin, zoom);
            let hit = find_component_at(&cc.placed_components, canvas_pos);
            if let Some(idx) = hit {
                cc.selected_index = Some(idx);
                cc.delete_selected();
                state.status_message = "Component deleted".to_owned();
                ui.ctx().request_repaint();
            }
        }
    }

    // ── Double-click to edit value inline ──
    if response.double_clicked_by(egui::PointerButton::Primary) {
        if let Some(cursor) = response.interact_pointer_pos() {
            let canvas_pos = screen_to_element(cursor, origin, zoom);
            let hit = find_component_at(&cc.placed_components, canvas_pos);
            if let Some(idx) = hit {
                let ctype = cc.placed_components[idx].component_type;
                if ctype.is_editable() {
                    cc.selected_index = Some(idx);
                    state.status_message = format!("Editing {} — use slider or type value", ctype.name());
                } else if ctype.is_plot() {
                    state.status_message = "Plot component selected — parameters adjust the curve".to_owned();
                } else {
                    state.status_message = format!("{} is read-only (computed)", ctype.name());
                }
                ui.ctx().request_repaint();
            }
        }
    }

    // ── Draw grid ──
    draw_grid(&painter, origin, response.rect, zoom);

    // ── Draw all placed components ──
    // First pass: draw editable selected components using Ui (needs &mut cc)
    // We need to know which index is selected before iterating to avoid borrow issues.
    let selected_idx = cc.selected_index;
    if let Some(s_idx) = selected_idx {
        if s_idx < cc.placed_components.len() {
            let ctype = cc.placed_components[s_idx].component_type;
            if ctype.is_editable() && !ctype.is_plot() {
                draw_editable_component_ui(ui, s_idx, cc, origin, zoom, &response.rect);
            }
        }
    }

    // Second pass: draw all other components with painter (read-only access)
    for (idx, component) in cc.placed_components.iter().enumerate() {
        if component.component_type.is_plot() {
            continue;
        }
        let is_selected = cc.selected_index == Some(idx);
        if is_selected && component.component_type.is_editable() {
            continue; // already drawn above
        }
        let value = cc.get_value(component.component_type);
        draw_component_block(
            &painter,
            component,
            origin,
            zoom,
            is_selected,
            value,
        );
    }

    // ── Draw plot components (inside an allocated area using Ui) ──
    for (idx, component) in cc.placed_components.iter().enumerate() {
        if !component.component_type.is_plot() {
            continue;
        }
        let is_selected = cc.selected_index == Some(idx);
        let rect = plot_block_rect(component.pos, origin, zoom);
        // Clip within the canvas area
        let clipped_rect = rect.intersect(response.rect);
        if clipped_rect.is_positive() {
            // Allocate a Ui child for the plot
            let plot_rect = clipped_rect.shrink(4.0);
            // Reserve space at the plot's position by allocating a blank rect
            let (_plot_response, plot_painter) =
                ui.allocate_painter(plot_rect.size(), egui::Sense::click());

            // Draw the plot background/border
            let bg_color = Color32::from_rgba_premultiplied(20, 20, 40, 220);
            plot_painter.rect_filled(
                Rect::from_min_size(Pos2::ZERO, plot_rect.size()),
                CornerRadius::same(6),
                bg_color,
            );
            plot_painter.rect_stroke(
                Rect::from_min_size(Pos2::ZERO, plot_rect.size()),
                CornerRadius::same(6),
                Stroke::new(if is_selected { 2.5 } else { 1.0 }, if is_selected { Color32::YELLOW } else { Color32::from_gray(100) }),
                egui::StrokeKind::Outside,
            );

            // Draw the actual plot using egui_plot (allocate a child Ui in the plot area)
            // We use the plot_painter's response rect as the location
            let plot_origin_on_screen = plot_rect.min;
            let plot_size = plot_rect.size();

            // Use a child area for the actual egui plot widget
            let child_rect = egui::Rect::from_min_size(Pos2::ZERO, plot_size - Vec2::new(8.0, 8.0));
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(egui::Rect::from_min_size(
                        plot_origin_on_screen + Vec2::new(4.0, 4.0),
                        child_rect.size(),
                    ))
                    .layout(*ui.layout()),
            );

            draw_plot(&mut child_ui, &cc.shared_params);
        }
    }

    // ── Draw placement preview ──
    if let Some(ctype) = cc.palette_selection {
        if let Some(cursor) = response.hover_pos() {
            let canvas_pos = screen_to_element(cursor, origin, zoom);
            let snapped = snap(canvas_pos, GRID_SPACING);

            if ctype.is_plot() {
                let rect = plot_block_rect(snapped, origin, zoom);
                painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.5, Color32::LIGHT_GREEN), egui::StrokeKind::Outside);
                painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgba_premultiplied(0, 255, 0, 20));
                painter.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "📈 Plot: ΔiL vs Duty Cycle",
                    egui::TextStyle::Monospace.resolve(ui.style()),
                    Color32::LIGHT_GREEN,
                );
            } else {
                let rect = block_rect(snapped, origin, zoom);
                painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.5, Color32::LIGHT_GREEN), egui::StrokeKind::Outside);
                painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgba_premultiplied(0, 255, 0, 20));
                let value = cc.get_value(ctype);
                let label = format!("{}: {}", ctype.name(), format_value(value, ctype.unit()));
                painter.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    label,
                    egui::TextStyle::Monospace.resolve(ui.style()),
                    Color32::LIGHT_GREEN,
                );
            }
        }
    }

    // ── Status bar ──
    {
        let style = egui::Style::default();
        painter.text(
            Pos2::new(response.rect.min.x + 8.0, response.rect.max.y - 12.0),
            Align2::LEFT_BOTTOM,
            format!(
                "{} components | {:.0}% zoom | Click=place/select, DCtrl=edit, RClick=del, Scroll=zoom",
                cc.placed_components.len(),
                cc.zoom * 100.0,
            ),
            egui::TextStyle::Monospace.resolve(&style),
            Color32::GRAY,
        );
    }
}

// ── Plot rendering ──────────────────────────────────────────────────

/// Draw the actual plot content using egui_plot.
fn draw_plot(ui: &mut Ui, params: &crate::app::state::SharedParams) {
    let l = params.calc_inductance();
    let c = params.calc_capacitance();

    let n_points = 200;

    // Plot 1: Inductor current ripple (ΔiL_pp) vs Duty Cycle
    let mut il_ripple_data: Vec<[f64; 2]> = Vec::with_capacity(n_points);
    for i in 0..=n_points {
        let duty = i as f64 / n_points as f64;
        let ripple = params.calc_il_ripple_for_duty(duty, l);
        il_ripple_data.push([duty, ripple]);
    }

    // Plot 2: Output voltage ripple (ΔVo_pp) vs Duty Cycle
    let mut vo_ripple_data: Vec<[f64; 2]> = Vec::with_capacity(n_points);
    for i in 0..=n_points {
        let duty = i as f64 / n_points as f64;
        let ripple = params.calc_vo_ripple_for_duty(duty, l, c);
        vo_ripple_data.push([duty, ripple]);
    }

    // Plot 3: Inductance (L) vs Duty Cycle at various frequencies
    let mut l_vs_duty_data: Vec<[f64; 2]> = Vec::with_capacity(n_points);
    for i in 0..=n_points {
        let duty = i as f64 / n_points as f64;
        let delta_il_amps = params.delta_il * params.iout_max;
        if delta_il_amps > 0.0 {
            let l_val = (params.vout * (1.0 - duty)).abs() / (delta_il_amps * params.frequency);
            l_vs_duty_data.push([duty, l_val * 1e6]); // in μH
        }
    }

    let line_il = Line::new(
        "ΔiL (pp) [A]",
        PlotPoints::from(il_ripple_data),
    )
    .color(Color32::from_rgb(255, 165, 0))
    .width(1.5);

    let line_vo = Line::new(
        "ΔVo (pp) [V]",
        PlotPoints::from(vo_ripple_data),
    )
    .color(Color32::from_rgb(100, 200, 255))
    .width(1.5);

    let line_l = Line::new(
        "L [μH]",
        PlotPoints::from(l_vs_duty_data),
    )
    .color(Color32::from_rgb(100, 255, 100))
    .width(1.5);

    Plot::new("canvas_curve_plot")
        .legend(Legend::default())
        .height(ui.available_height().max(100.0))
        .width(ui.available_width().max(150.0))
        .x_axis_label("Duty Cycle")
        .y_axis_label("Value")
        .default_x_bounds(0.0, 1.0)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .show(ui, |plot_ui| {
            plot_ui.line(line_il);
            plot_ui.line(line_vo);
            plot_ui.line(line_l);
        });
}

// ── Keyboard handling ────────────────────────────────────────────────

fn handle_keyboard(ui: &mut Ui, state: &mut AppState) {
    let cc = &mut state.component_canvas;

    // Delete/Backspace to remove selected
    let delete_pressed = ui.input(|i| {
        i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)
    });
    if delete_pressed && cc.selected_index.is_some() {
        cc.delete_selected();
        state.status_message = "Component deleted".to_owned();
        ui.ctx().request_repaint();
    }
}

// ── Drawing helpers ──────────────────────────────────────────────────

/// Draw a selected editable component as an interactive Ui area with input + slider.
fn draw_editable_component_ui(
    ui: &mut Ui,
    idx: usize,
    cc: &mut crate::app::state::ComponentCanvasState,
    origin: Pos2,
    zoom: f32,
    canvas_rect: &Rect,
) {
    let component = &cc.placed_components[idx];
    let rect = block_rect(component.pos, origin, zoom);
    let clipped = rect.intersect(*canvas_rect);
    if !clipped.is_positive() {
        return;
    }

    let ctype = component.component_type;

    // Allocate a Ui child in the block's screen area
    let block_screen_rect = egui::Rect::from_min_size(clipped.min, Vec2::new(clipped.size().x, clipped.size().y));
    let mut child_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(block_screen_rect)
            .layout(egui::Layout::top_down_justified(egui::Align::Center)),
    );

    // Draw background
    let bg_color = Color32::from_rgba_premultiplied(30, 60, 120, 220);
    let painter = child_ui.painter();
    painter.rect_filled(
        Rect::from_min_size(Pos2::ZERO, clipped.size()),
        CornerRadius::same(6),
        bg_color,
    );
    painter.rect_stroke(
        Rect::from_min_size(Pos2::ZERO, clipped.size()),
        CornerRadius::same(6),
        Stroke::new(2.5, Color32::YELLOW),
        egui::StrokeKind::Outside,
    );

    // Title
    child_ui.add_space(4.0);
    child_ui.label(
        egui::RichText::new(ctype.name())
            .color(Color32::WHITE)
            .monospace()
            .size(12.0),
    );

    // ── Interactive controls ──
    // Get the current value (in display units — duty cycle as %, delta_il as %, etc.)
    let current_value = cc.get_value(ctype);

    // Define range and speed per component type
    let (range_min, range_max, speed, suffix): (f64, f64, f64, &str) = match ctype {
        CanvasComponentType::Vin => (1.0, 500.0, 1.0, "V"),
        CanvasComponentType::Vout => (0.5, 500.0, 1.0, "V"),
        CanvasComponentType::DutyCycle => (1.0, 99.0, 0.5, "%"),
        CanvasComponentType::Frequency => (100.0, 1_000_000.0, 1000.0, "Hz"),
        CanvasComponentType::DeltaIl => (0.1, 100.0, 0.5, "%"),
        CanvasComponentType::IoutMax => (0.1, 100.0, 0.2, "A"),
        CanvasComponentType::DeltaVo => (0.01, 50.0, 0.1, "%"),
        _ => return,
    };

    // Use a copy that we can modify
    let mut value_copy = current_value;

    // Slider row
    child_ui.add(
        egui::Slider::new(&mut value_copy, range_min..=range_max)
            .suffix(suffix)
            .show_value(false)
            .clamping(egui::SliderClamping::Never),
    );

    // DragValue (input field) row
    child_ui.horizontal(|ui| {
        ui.add_space(8.0);
        // Show the formatted value before the input
        ui.label(
            egui::RichText::new("=")
                .color(Color32::GRAY)
                .monospace(),
        );

        let response = ui.add(
            egui::DragValue::new(&mut value_copy)
                .speed(speed)
                .suffix(suffix),
        );

        // Show computed values below for L/C affected components
        if ctype == CanvasComponentType::Vin || ctype == CanvasComponentType::Vout
            || ctype == CanvasComponentType::DutyCycle || ctype == CanvasComponentType::Frequency
            || ctype == CanvasComponentType::DeltaIl || ctype == CanvasComponentType::IoutMax
            || ctype == CanvasComponentType::DeltaVo
        {
            // We'll show the resulting L/C values below instead
        }

        if response.changed() {
            // Apply the value to shared params
            let changed = cc.set_value(ctype, value_copy);
            if changed {
                // Show computed L/C result in status bar
                ui.ctx().request_repaint();
            }
        }
    });

    // Show computed L and C values below the controls
    child_ui.add_space(2.0);
    let l_val = cc.shared_params.calc_inductance();
    let c_val = cc.shared_params.calc_capacitance();
    child_ui.label(
        egui::RichText::new(format!(
            "L = {}  C = {}",
            format_eng_small(l_val, "H"),
            format_eng_small(c_val, "F"),
        ))
        .color(Color32::from_rgb(150, 200, 255))
        .size(9.0)
        .monospace(),
    );

    // Ensure repaint is active while dragging
    if child_ui.ctx().is_pointer_over_egui() {
        child_ui.ctx().request_repaint();
    }
}

/// Draw a single component block on the canvas (non-plot, non-interactive).
fn draw_component_block(
    painter: &egui::Painter,
    component: &crate::app::state::PlacedComponent,
    origin: Pos2,
    zoom: f32,
    selected: bool,
    value: f64,
) {
    let rect = block_rect(component.pos, origin, zoom);

    // Background color based on type
    let bg_color = if component.component_type.is_editable() {
        Color32::from_rgba_premultiplied(30, 60, 120, 200) // blue-ish for inputs
    } else {
        Color32::from_rgba_premultiplied(80, 40, 20, 200) // brown-ish for computed
    };

    let border_color = if selected {
        Color32::YELLOW
    } else {
        Color32::from_gray(100)
    };
    let border_width = if selected { 2.5 } else { 1.0 };

    // Background
    painter.rect_filled(rect, CornerRadius::same(6), bg_color);
    painter.rect_stroke(rect, CornerRadius::same(6), Stroke::new(border_width, border_color), egui::StrokeKind::Outside);

    // Title
    let title = component.component_type.name();
    painter.text(
        Pos2::new(rect.min.x + 8.0, rect.min.y + 10.0),
        Align2::LEFT_TOP,
        title,
        egui::TextStyle::Monospace.resolve(&egui::Style::default()),
        Color32::WHITE,
    );

    // Value display
    let unit = component.component_type.unit();
    let value_text = format_value(value, unit);
    painter.text(
        Pos2::new(rect.center().x, rect.max.y - 10.0),
        Align2::CENTER_BOTTOM,
        &value_text,
        egui::TextStyle::Monospace.resolve(&egui::Style::default()),
        if selected {
            Color32::YELLOW
        } else {
            Color32::from_rgb(150, 200, 255)
        },
    );

    // Hint for editable but not selected
    if component.component_type.is_editable() && !selected {
        painter.text(
            Pos2::new(rect.right() - 4.0, rect.min.y + 10.0),
            Align2::RIGHT_TOP,
            "click to edit",
            egui::TextStyle::Monospace.resolve(&egui::Style::default()),
            Color32::from_rgba_premultiplied(150, 150, 150, 120),
        );
    }
}

/// Compute the screen-space rectangle for a normal component block.
fn block_rect(pos: Pos, origin: Pos2, zoom: f32) -> Rect {
    let x = origin.x + pos.x * zoom;
    let y = origin.y + pos.y * zoom;
    Rect::from_min_size(Pos2::new(x, y), Vec2::new(BLOCK_W * zoom, BLOCK_H * zoom))
}

/// Compute the screen-space rectangle for a plot block (larger).
fn plot_block_rect(pos: Pos, origin: Pos2, zoom: f32) -> Rect {
    let x = origin.x + pos.x * zoom;
    let y = origin.y + pos.y * zoom;
    Rect::from_min_size(Pos2::new(x, y), Vec2::new(PLOT_BLOCK_W * zoom, PLOT_BLOCK_H * zoom))
}

/// Draw the infinite grid.
fn draw_grid(painter: &egui::Painter, origin: Pos2, rect: Rect, zoom: f32) {
    let gs = GRID_SPACING * zoom;
    let gc = Color32::from_rgba_premultiplied(80, 80, 80, 30);
    let gx = (origin.x % gs) - gs;
    let gy = (origin.y % gs) - gs;
    let mut x = gx;
    while x < rect.max.x {
        painter.line_segment(
            [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
            Stroke::new(0.5, gc),
        );
        x += gs;
    }
    let mut y = gy;
    while y < rect.max.y {
        painter.line_segment(
            [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
            Stroke::new(0.5, gc),
        );
        y += gs;
    }
}

// ── Helpers ──────────────────────────────────────────────────────────

fn screen_to_element(screen_pos: Pos2, origin: Pos2, zoom: f32) -> Pos {
    Pos::new(
        (screen_pos.x - origin.x) / zoom,
        (screen_pos.y - origin.y) / zoom,
    )
}

fn find_component_at(
    components: &[crate::app::state::PlacedComponent],
    point: Pos,
) -> Option<usize> {
    // Hit-test: use appropriate block size based on type
    for (idx, comp) in components.iter().enumerate().rev() {
        let (half_w, half_h) = if comp.component_type.is_plot() {
            (PLOT_BLOCK_W / 2.0, PLOT_BLOCK_H / 2.0)
        } else {
            (BLOCK_W / 2.0, BLOCK_H / 2.0)
        };
        let left = comp.pos.x - half_w;
        let right = comp.pos.x + half_w;
        let top = comp.pos.y - half_h;
        let bottom = comp.pos.y + half_h;
        if point.x >= left && point.x <= right && point.y >= top && point.y <= bottom {
            return Some(idx);
        }
    }
    None
}

fn snap(pos: Pos, grid: f32) -> Pos {
    Pos::new(
        (pos.x / grid).round() * grid,
        (pos.y / grid).round() * grid,
    )
}

/// Format a value with SI prefix (for small numbers use engineering notation).
fn format_value(value: f64, unit: &str) -> String {
    let abs_val = value.abs();
    if abs_val == 0.0 {
        return format!("0 {}", unit);
    }
    let (scaled, prefix) = if abs_val >= 1_000_000.0 {
        (value / 1_000_000.0, "M")
    } else if abs_val >= 1_000.0 {
        (value / 1_000.0, "k")
    } else if abs_val >= 1.0 {
        (value, "")
    } else if abs_val >= 0.001 {
        (value * 1_000.0, "m")
    } else if abs_val >= 0.000_001 {
        (value * 1_000_000.0, "μ")
    } else if abs_val >= 1e-9 {
        (value * 1e9, "n")
    } else {
        (value * 1e12, "p")
    };
    let decimals = if scaled.abs() >= 100.0 {
        1
    } else if scaled.abs() >= 10.0 {
        2
    } else {
        3
    };
    format!("{:.prec$} {}{}", scaled, prefix, unit, prec = decimals)
}

/// Format with small-suffix for sidebar display (same as format_value).
fn format_eng_small(value: f64, unit: &str) -> String {
    format_value(value, unit)
}
