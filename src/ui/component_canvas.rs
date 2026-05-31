use crate::app::state::{AppState, CanvasComponentType, PlacedComponent, Theme};
use crate::schematic::primitives::Pos;
use crate::tupan_ui::UiExt;
use egui::{Align2, Color32, CornerRadius, CursorIcon, Pos2, Rect, Stroke, Ui, Vec2};
use egui_plot::{Legend, Line, LineStyle, Plot, PlotPoints};

// ── Default sizing ──
const BLOCK_W: f32 = 180.0;
const BLOCK_H: f32 = 90.0;
const PLOT_BLOCK_W: f32 = 280.0;
const PLOT_BLOCK_H: f32 = 200.0;
const MIN_COMP_W: f32 = 100.0;
const MIN_COMP_H: f32 = 60.0;
const MIN_PLOT_W: f32 = 120.0;
const MIN_PLOT_H: f32 = 100.0;
const GRID_SPACING: f32 = 40.0;
const RESIZE_HANDLE_SIZE: f32 = 12.0;

fn component_size(comp: &PlacedComponent) -> (f32, f32) {
    if let Some((w, h)) = comp.size_override { (w, h) }
    else if comp.component_type.is_plot() { (PLOT_BLOCK_W, PLOT_BLOCK_H) }
    else { (BLOCK_W, BLOCK_H) }
}

fn min_size_for(comp: &PlacedComponent) -> (f32, f32) {
    if comp.component_type.is_plot() { (MIN_PLOT_W, MIN_PLOT_H) }
    else { (MIN_COMP_W, MIN_COMP_H) }
}

fn block_rect(pos: Pos, origin: Pos2, zoom: f32, w: f32, h: f32) -> Rect {
    Rect::from_min_size(Pos2::new(origin.x + pos.x * zoom, origin.y + pos.y * zoom), Vec2::new(w * zoom, h * zoom))
}

fn resize_handle_rect(rect: Rect) -> Rect {
    Rect::from_min_size(Pos2::new(rect.max.x - RESIZE_HANDLE_SIZE, rect.max.y - RESIZE_HANDLE_SIZE), Vec2::splat(RESIZE_HANDLE_SIZE))
}

pub fn show_component_canvas(ui: &mut Ui, state: &mut AppState) {
    let tokens = ui.tokens();
    egui::Panel::left("canvas_sidebar").resizable(true).default_size(220.0).min_size(180.0)
        .frame(egui::Frame { fill: tokens.sidebar_bg_color, inner_margin: egui::Margin::symmetric(12, 8), ..Default::default() })
        .show_inside(ui, |ui| { draw_sidebar(ui, state); });
    egui::CentralPanel::default()
        .frame(egui::Frame { fill: tokens.canvas_bg_color, ..Default::default() })
        .show_inside(ui, |ui| { handle_canvas(ui, state); });
    handle_keyboard(ui, state);
}

fn draw_sidebar(ui: &mut Ui, state: &mut AppState) {
    let cc = &mut state.component_canvas;
    ui.section_header("INPUTS"); ui.add_space(6.0);
    for &(ct, n, i) in &[(CanvasComponentType::Vin,"Vin","⚡"),(CanvasComponentType::Vout,"Vout","🔌"),(CanvasComponentType::DutyCycle,"Duty Cycle","〰"),(CanvasComponentType::Frequency,"Frequency","📡"),(CanvasComponentType::DeltaIl,"ΔiL","📉"),(CanvasComponentType::IoutMax,"Iout,max","💧"),(CanvasComponentType::DeltaVo,"ΔVo","📊")] { draw_palette_card(ui, cc, ct, n, i); }
    ui.add_space(12.0); ui.section_header("COMPUTED"); ui.add_space(6.0);
    for &(ct, n, i) in &[(CanvasComponentType::Inductor,"Inductor (L)","〰"),(CanvasComponentType::Capacitor,"Capacitor (C)","‖‖")] { draw_palette_card(ui, cc, ct, n, i); }
    ui.add_space(12.0); ui.section_header("VIZ"); ui.add_space(6.0);
    draw_palette_card(ui, cc, CanvasComponentType::Plot, "Curve Plot", "📈");
    ui.add_space(16.0); ui.section_header("PARAMETERS"); ui.add_space(6.0);
    let mut changed = false;
    changed |= ui.param_row("Vin", &mut cc.shared_params.vin, 1.0, 500.0, 1.0, "V");
    changed |= ui.param_row("Vout", &mut cc.shared_params.vout, 0.5, 500.0, 1.0, "V");
    if changed && cc.shared_params.vin > 0.0 { cc.shared_params.duty_cycle = (cc.shared_params.vout / cc.shared_params.vin).clamp(0.0, 1.0); }
    let mut dc = cc.shared_params.duty_cycle * 100.0;
    if ui.param_row("D", &mut dc, 1.0, 99.0, 0.5, "%") { cc.shared_params.duty_cycle = (dc / 100.0).clamp(0.0, 1.0); cc.shared_params.vout = cc.shared_params.vin * cc.shared_params.duty_cycle; changed = true; }
    changed |= ui.param_row("Freq", &mut cc.shared_params.frequency, 100.0, 1_000_000.0, 1000.0, "Hz");
    changed |= ui.param_pct("ΔiL", &mut cc.shared_params.delta_il, 0.001, 1.0);
    changed |= ui.param_row("Iout,max", &mut cc.shared_params.iout_max, 0.1, 100.0, 0.2, "A");
    changed |= ui.param_pct("ΔVo", &mut cc.shared_params.delta_vo, 0.0001, 0.5);
    ui.add_space(16.0); ui.section_header("RESULTS"); ui.add_space(6.0);
    ui.result_row("L", cc.shared_params.calc_inductance(), "H");
    ui.result_row("C", cc.shared_params.calc_capacitance(), "F");
    ui.result_row("ΔiL(A)", cc.shared_params.calc_delta_il_amps(), "A");
    if changed { state.status_message = "Parameters updated".to_owned(); }
}

fn draw_palette_card(ui: &mut Ui, cc: &mut crate::app::state::ComponentCanvasState, ctype: CanvasComponentType, name: &str, icon: &str) {
    let tk = ui.tokens();
    let sel = cc.palette_selection == Some(ctype);
    let (r, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), egui::Sense::click());
    let bg = if sel { tk.card_selected_color } else if resp.hovered() { tk.card_hover_color } else { tk.card_bg_color };
    ui.painter().rect_filled(r, CornerRadius::same(4), bg);
    if sel { ui.painter().rect_filled(Rect::from_min_size(Pos2::new(r.min.x, r.min.y), Vec2::new(3.0, r.height())), CornerRadius::same(2), tk.accent_color); }
    ui.painter().text(Pos2::new(r.min.x + 12.0, r.center().y), Align2::LEFT_CENTER, &format!("{}  {}", icon, name),
        egui::TextStyle::Body.resolve(ui.style()), tk.text_primary);
    if resp.clicked() { cc.palette_selection = Some(ctype); }
}

fn handle_canvas(ui: &mut Ui, state: &mut AppState) {
    let tk = ui.tokens();
    let avail = ui.available_size();
    let (resp, painter) = ui.allocate_painter(avail, egui::Sense::click_and_drag());
    let cc = &mut state.component_canvas;
    let origin = Pos2::new(resp.rect.min.x + cc.pan_offset.0, resp.rect.min.y + cc.pan_offset.1);
    let zoom = cc.zoom;

    // zoom
    let scroll = ui.input(|i| i.smooth_scroll_delta().y);
    if scroll != 0.0 {
        if let Some(cursor) = resp.hover_pos() {
            let factor = if scroll > 0.0 { 1.1 } else { 0.9 };
            let nz = (cc.zoom * factor).clamp(0.4, 5.0);
            if (nz - cc.zoom).abs() > 0.001 {
                let cr = cursor - resp.rect.min.to_vec2();
                let wb = Vec2::new((cr.x - cc.pan_offset.0) / cc.zoom, (cr.y - cc.pan_offset.1) / cc.zoom);
                cc.zoom = nz; cc.pan_offset.0 = cr.x - wb.x * nz; cc.pan_offset.1 = cr.y - wb.y * nz;
                ui.ctx().request_repaint();
            }
        } else { cc.zoom = (cc.zoom * if scroll > 0.0 { 1.1 } else { 0.9 }).clamp(0.4, 5.0); ui.ctx().request_repaint(); }
    }

    // drag: move or pan
    if resp.dragged_by(egui::PointerButton::Primary) {
        let on_handle = cc.selected_index.and_then(|si| {
            (si < cc.placed_components.len()).then(|| {
                let c = &cc.placed_components[si]; let (w, h) = component_size(c);
                resize_handle_rect(block_rect(c.pos, origin, zoom, w, h))
                    .contains(resp.interact_pointer_pos().unwrap_or(Pos2::ZERO))
            })
        }).unwrap_or(false);
        if let Some(si) = cc.selected_index {
            if !on_handle {
                let d = resp.drag_delta(); let c = &mut cc.placed_components[si];
                c.pos.x += d.x / zoom; c.pos.y += d.y / zoom;
                state.status_message = format!("Moving {}", c.component_type.name());
                ui.ctx().request_repaint();
            }
        } else { let d = resp.drag_delta(); cc.pan_offset.0 += d.x; cc.pan_offset.1 += d.y; ui.ctx().request_repaint(); }
    }

    // hit testing
    let mut resize_hit: Option<usize> = None;
    let mut click_hit: Option<usize> = None;
    if resp.clicked_by(egui::PointerButton::Primary) {
        if let Some(cursor) = resp.interact_pointer_pos() {
            let cp = screen_to_element(cursor, origin, zoom);
            if let Some(si) = cc.selected_index {
                if si < cc.placed_components.len() {
                    let c = &cc.placed_components[si]; let (w, h) = component_size(c);
                    let hr = resize_handle_rect(block_rect(c.pos, origin, zoom, w, h));
                    if hr.contains(cursor) { resize_hit = Some(si); }
                }
            }
            if resize_hit.is_none() {
                for idx in (0..cc.placed_components.len()).rev() {
                    let c = &cc.placed_components[idx]; let (w, h) = component_size(c);
                    let hw = w / 2.0; let hh = h / 2.0;
                    if cp.x >= c.pos.x - hw && cp.x <= c.pos.x + hw && cp.y >= c.pos.y - hh && cp.y <= c.pos.y + hh {
                        click_hit = Some(idx); break;
                    }
                }
            }
        }
    }

    if resize_hit.is_none() && resp.clicked_by(egui::PointerButton::Primary) {
        if let Some(cursor) = resp.interact_pointer_pos() {
            let cp = screen_to_element(cursor, origin, zoom);
            if let Some(idx) = click_hit {
                cc.palette_selection = None; cc.selected_index = Some(idx);
                state.status_message = format!("Selected {}", cc.placed_components[idx].component_type.name());
            } else if let Some(ct) = cc.palette_selection {
                cc.place_component(ct, snap(cp, GRID_SPACING)); cc.palette_selection = None;
                state.status_message = format!("Placed {}", ct.name());
            } else { cc.selected_index = None; }
            ui.ctx().request_repaint();
        }
    }

    // resize drag on selected component
    if let Some(si) = cc.selected_index {
        if si < cc.placed_components.len() {
            let c = &cc.placed_components[si]; let (cw, ch) = component_size(c);
            let r = block_rect(c.pos, origin, zoom, cw, ch);
            let hr = resize_handle_rect(r);
            if resp.dragged_by(egui::PointerButton::Primary) && hr.contains(resp.interact_pointer_pos().unwrap_or(r.min)) {
                let d = resp.drag_delta(); let c = &mut cc.placed_components[si];
                let (cw, ch) = component_size(c); let (mw, mh) = min_size_for(c);
                c.size_override = Some(((cw + d.x / zoom).max(mw), (ch + d.y / zoom).max(mh)));
                state.status_message = format!("Resizing {}", c.component_type.name());
                ui.ctx().request_repaint();
            }
            if hr.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO))) { ui.ctx().set_cursor_icon(CursorIcon::ResizeSouthEast); }
        }
    }

    // right-click delete
    if resp.clicked_by(egui::PointerButton::Secondary) {
        if let Some(cursor) = resp.interact_pointer_pos() {
            let cp = screen_to_element(cursor, origin, zoom);
            if let Some(idx) = find_component_at(cc, cp, origin, zoom) {
                cc.selected_index = Some(idx); cc.delete_selected();
                state.status_message = "Component deleted".to_owned(); ui.ctx().request_repaint();
            }
        }
    }

    // draw
    draw_grid(&painter, origin, resp.rect, zoom, tk.grid_color);

    for (idx, comp) in cc.placed_components.iter().enumerate() {
        if comp.component_type.is_plot() { continue; }
        let sel = cc.selected_index == Some(idx); let (w, h) = component_size(comp);
        let r = block_rect(comp.pos, origin, zoom, w, h);
        draw_component_block(&painter, &resp.rect, comp, r, sel, cc.get_value(comp.component_type), tk);
    }

    let plot_idx: Vec<_> = cc.placed_components.iter().enumerate().filter(|(_,c)|c.component_type.is_plot()).map(|(i,c)|(i,c.pos,c.size_override)).collect();
    for (idx, pos, sz) in &plot_idx {
        let sel = cc.selected_index == Some(*idx);
        let (w, h) = sz.unwrap_or((PLOT_BLOCK_W, PLOT_BLOCK_H));
        let r = block_rect(*pos, origin, zoom, w, h);
        draw_plot_block(ui, &painter, &resp.rect, r, sel, cc, state.theme, tk);
    }

    // resize handle on selected
    if let Some(si) = cc.selected_index {
        if si < cc.placed_components.len() {
            let c = &cc.placed_components[si]; let (w, h) = component_size(c);
            let hr = resize_handle_rect(block_rect(c.pos, origin, zoom, w, h));
            painter.rect_filled(hr, CornerRadius::same(2), tk.accent_color);
            let cx = hr.center(); let s = RESIZE_HANDLE_SIZE / 4.0;
            painter.line_segment([Pos2::new(cx.x - s, cx.y - s), Pos2::new(cx.x + s, cx.y + s)], Stroke::new(2.0, tk.canvas_bg_color));
        }
    }

    // editable overlay
    if let Some(si) = cc.selected_index {
        if si < cc.placed_components.len() {
            let ct = cc.placed_components[si].component_type;
            if ct.is_editable() && !ct.is_plot() { draw_editable_ui(ui, si, cc, origin, zoom, &resp.rect); }
        }
    }

    // preview
    if let Some(ct) = cc.palette_selection {
        if let Some(cursor) = resp.hover_pos() {
            let sn = snap(screen_to_element(cursor, origin, zoom), GRID_SPACING);
            let (dw, dh) = if ct.is_plot() { (PLOT_BLOCK_W, PLOT_BLOCK_H) } else { (BLOCK_W, BLOCK_H) };
            let r = block_rect(sn, origin, zoom, dw, dh);
            painter.rect_stroke(r, CornerRadius::same(4), Stroke::new(1.5, tk.accent_light_color), egui::StrokeKind::Outside);
            painter.rect_filled(r, CornerRadius::same(4), Color32::from_rgba_premultiplied(tk.accent_color.r(), tk.accent_color.g(), tk.accent_color.b(), 20));
            let lbl = if ct.is_plot() { "📈 Plot".into() } else { format!("{}: {}", ct.name(), format_eng(cc.get_value(ct), ct.unit())) };
            painter.text(r.center(), Align2::CENTER_CENTER, &lbl, egui::TextStyle::Monospace.resolve(ui.style()), tk.accent_light_color);
        }
    }

    painter.text(Pos2::new(resp.rect.min.x + 12.0, resp.rect.max.y - 12.0), Align2::LEFT_BOTTOM,
        format!("{} components  ·  {:.0}% zoom  ·  Click to place  ·  Right-click to delete  ·  Scroll to zoom", cc.placed_components.len(), cc.zoom * 100.0),
        egui::TextStyle::Monospace.resolve(&egui::Style::default()), tk.status_color);
}

fn draw_plot_block(ui: &mut Ui, painter: &egui::Painter, canvas: &Rect, rect: Rect, sel: bool, cc: &mut crate::app::state::ComponentCanvasState, theme: Theme, tk: &crate::tupan_ui::DesignTokens) {
    if !rect.intersect(*canvas).is_positive() { return; }
    let bc = if sel { tk.selected_color } else { tk.input_border_color };
    painter.rect_filled(rect, CornerRadius::same(8), tk.plot_bg_color);
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(if sel { 2.0 } else { 1.0 }, bc), egui::StrokeKind::Outside);
    let pa = rect.shrink(6.0); let pr = pa.intersect(*canvas);
    if pr.is_positive() { let mut cu = ui.new_child(egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(pr.min, pr.size())).layout(*ui.layout())); draw_plot(&mut cu, &cc.shared_params, theme); }
}

fn draw_component_block(painter: &egui::Painter, canvas: &Rect, comp: &PlacedComponent, rect: Rect, selected: bool, value: f64, tk: &crate::tupan_ui::DesignTokens) {
    if !rect.intersect(*canvas).is_positive() { return; }
    let edit = comp.component_type.is_editable();
    let bg = if edit { tk.input_bg_color } else { tk.computed_bg_color };
    let bc = if selected { tk.selected_color } else if edit { tk.input_border_color } else { tk.computed_border_color };
    let accent = if edit { tk.accent_color } else { tk.accent_dim_color };
    painter.rect_filled(rect, CornerRadius::same(8), bg);
    painter.rect_filled(Rect::from_min_size(Pos2::new(rect.min.x, rect.min.y), Vec2::new(rect.width(), 3.0)), CornerRadius::same(2), accent);
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(if selected { 2.0 } else { 1.0 }, bc), egui::StrokeKind::Outside);
    painter.text(Pos2::new(rect.min.x + 8.0, rect.min.y + 10.0), Align2::LEFT_TOP, comp.component_type.name(), egui::TextStyle::Monospace.resolve(&egui::Style::default()), tk.text_primary);
    painter.text(Pos2::new(rect.center().x, rect.max.y - 10.0), Align2::CENTER_BOTTOM, &format_eng(value, comp.component_type.unit()), egui::TextStyle::Monospace.resolve(&egui::Style::default()), if selected { tk.selected_color } else { tk.text_value });
    if edit && !selected { painter.text(Pos2::new(rect.right() - 4.0, rect.min.y + 10.0), Align2::RIGHT_TOP, "click", egui::TextStyle::Monospace.resolve(&egui::Style::default()), Color32::from_rgba_premultiplied(tk.text_secondary.r(), tk.text_secondary.g(), tk.text_secondary.b(), 80)); }
}

fn draw_editable_ui(ui: &mut Ui, idx: usize, cc: &mut crate::app::state::ComponentCanvasState, origin: Pos2, zoom: f32, canvas: &Rect) {
    let tk = ui.tokens(); let comp = &cc.placed_components[idx];
    let (w, h) = component_size(comp); let r = block_rect(comp.pos, origin, zoom, w, h).intersect(*canvas);
    if !r.is_positive() { return; }
    let ct = comp.component_type;
    let mut cu = ui.new_child(egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(r.min + Vec2::new(4.0, 4.0), Vec2::new(r.size().x - 8.0, r.size().y - 8.0))).layout(egui::Layout::top_down_justified(egui::Align::Center)));
    cu.add_space(4.0);
    cu.label(egui::RichText::new(ct.name()).color(tk.text_primary).monospace().size(11.0).strong());
    let (lo, hi, spd, sfx) = match ct {
        CanvasComponentType::Vin => (1.0, 500.0, 1.0, "V"), CanvasComponentType::Vout => (0.5, 500.0, 1.0, "V"),
        CanvasComponentType::DutyCycle => (1.0, 99.0, 0.5, "%"), CanvasComponentType::Frequency => (100.0, 1_000_000.0, 1000.0, "Hz"),
        CanvasComponentType::DeltaIl => (0.1, 100.0, 0.5, "%"), CanvasComponentType::IoutMax => (0.1, 100.0, 0.2, "A"),
        CanvasComponentType::DeltaVo => (0.01, 50.0, 0.1, "%"), _ => return,
    };
    let mut v = cc.get_value(ct);
    if cu.add(egui::Slider::new(&mut v, lo..=hi).suffix(sfx).show_value(false).clamping(egui::SliderClamping::Never)).changed() { cc.set_value(ct, v); cu.ctx().request_repaint(); }
    cu.horizontal(|ui| { ui.add_space(8.0); ui.label(egui::RichText::new("=").color(tk.text_secondary).monospace()); if ui.add(egui::DragValue::new(&mut v).speed(spd).suffix(sfx)).changed() { cc.set_value(ct, v); ui.ctx().request_repaint(); } });
    cu.add_space(2.0);
    cu.label(egui::RichText::new(format!("L {}  C {}", format_eng(cc.shared_params.calc_inductance(), "H"), format_eng(cc.shared_params.calc_capacitance(), "F"))).color(tk.text_value).size(9.0).monospace());
    if cu.ctx().is_pointer_over_egui() { cu.ctx().request_repaint(); }
}

fn draw_plot(ui: &mut Ui, params: &crate::app::state::SharedParams, _theme: Theme) {
    let (l, c_val, f, d, vin, vout, iout) = (params.calc_inductance(), params.calc_capacitance(), params.frequency, params.duty_cycle, params.vin, params.vout, params.iout_max);
    let period = if f > 0.0 { 1.0 / f } else { 1e-5 }; let t_end = 4.0 * period; let n = 400; let dt = t_end / n as f64;
    let mut il = Vec::with_capacity(n + 1); let mut vc = Vec::with_capacity(n + 1); let mut sw = Vec::with_capacity(n + 1);
    let rip = if l > 0.0 && f > 0.0 { ((vin - vout) * d) / (l * f) } else { 0.0 };
    let (mut il_cur, mut vc_cur) = (iout - rip / 2.0, vout);
    for i in 0..=n {
        let t = i as f64 * dt; let s = if (t % period) / period < d { 1.0 } else { 0.0 };
        if l > 0.0 { il_cur += (s * vin - vc_cur) / l * dt; }
        if c_val > 0.0 && l > 0.0 { vc_cur += (il_cur - vc_cur / (vout / iout.max(0.01))) / c_val * dt; }
        let td = if t_end >= 1e-3 { t * 1e3 } else { t * 1e6 };
        il.push([td, il_cur]); vc.push([td, vc_cur]); sw.push([td, s * iout * 1.2]);
    }
    let tu = if t_end >= 1e-3 { "ms" } else { "μs" };
    Plot::new("canvas_plot").legend(Legend::default()).height(ui.available_height().max(100.0)).width(ui.available_width().max(150.0))
        .x_axis_label(&format!("Time [{}]", tu)).y_axis_label("Value").allow_drag(false).allow_zoom(false).allow_scroll(false).allow_boxed_zoom(false)
        .show(ui, |pu| { pu.line(Line::new("iL [A]", PlotPoints::from(il)).color(Color32::from_rgb(255,165,0)).width(1.5)); pu.line(Line::new("Vout [V]", PlotPoints::from(vc)).color(Color32::from_rgb(100,200,255)).width(1.5)); pu.line(Line::new("Switch", PlotPoints::from(sw)).color(Color32::from_rgb(160,160,180)).width(1.0).style(LineStyle::Dashed{length:6.0})); });
}

fn handle_keyboard(ui: &mut Ui, state: &mut AppState) {
    let cc = &mut state.component_canvas;
    if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) && cc.selected_index.is_some() {
        cc.delete_selected(); state.status_message = "Component deleted".to_owned(); ui.ctx().request_repaint();
    }
}

fn draw_grid(painter: &egui::Painter, origin: Pos2, rect: Rect, zoom: f32, color: Color32) {
    let gs = GRID_SPACING * zoom; let mut x = (origin.x % gs) - gs;
    while x < rect.max.x { painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], Stroke::new(0.5, color)); x += gs; }
    let mut y = (origin.y % gs) - gs;
    while y < rect.max.y { painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], Stroke::new(0.5, color)); y += gs; }
}

fn screen_to_element(screen: Pos2, origin: Pos2, zoom: f32) -> Pos { Pos::new((screen.x - origin.x) / zoom, (screen.y - origin.y) / zoom) }
fn find_component_at(cc: &crate::app::state::ComponentCanvasState, point: Pos, _origin: Pos2, _zoom: f32) -> Option<usize> {
    for (idx, comp) in cc.placed_components.iter().enumerate().rev() {
        let (w, h) = component_size(comp); let hw = w / 2.0; let hh = h / 2.0;
        if point.x >= comp.pos.x - hw && point.x <= comp.pos.x + hw && point.y >= comp.pos.y - hh && point.y <= comp.pos.y + hh { return Some(idx); }
    }
    None
}
fn snap(pos: Pos, grid: f32) -> Pos { Pos::new((pos.x / grid).round() * grid, (pos.y / grid).round() * grid) }
fn format_value(value: f64, unit: &str) -> String {
    let av = value.abs(); if av == 0.0 { return format!("0 {}", unit); }
    let (s, p) = if av >= 1_000_000.0 { (value / 1_000_000.0, "M") } else if av >= 1_000.0 { (value / 1_000.0, "k") } else if av >= 1.0 { (value, "") } else if av >= 0.001 { (value * 1_000.0, "m") } else if av >= 0.000_001 { (value * 1_000_000.0, "μ") } else if av >= 1e-9 { (value * 1e9, "n") } else { (value * 1e12, "p") };
    let d = if s.abs() >= 100.0 { 1 } else if s.abs() >= 10.0 { 2 } else { 3 };
    format!("{:.prec$} {}{}", s, p, unit, prec = d)
}
fn format_eng(value: f64, unit: &str) -> String { format_value(value, unit) }
