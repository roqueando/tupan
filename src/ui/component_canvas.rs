use crate::app::state::{AppState, CanvasComponentType, Theme};
use crate::schematic::primitives::Pos;
use egui::{Align2, Color32, CornerRadius, Pos2, Rect, Stroke, Ui, Vec2};
use egui_plot::{Legend, Line, LineStyle, Plot, PlotPoints};

// ── Sizing ──
const BLOCK_W: f32 = 180.0;
const BLOCK_H: f32 = 90.0;
const PLOT_BLOCK_W: f32 = 280.0;
const PLOT_BLOCK_H: f32 = 200.0;
const GRID_SPACING: f32 = 40.0;

// ── Design Palette ────────────────────────────────────────────────────

mod palette {
    use egui::Color32;
    pub const ACCENT: Color32 = Color32::from_rgb(99, 130, 255);
    pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(130, 160, 255);
    pub const ACCENT_DIM: Color32 = Color32::from_rgb(60, 90, 200);
    pub const CANVAS_BG_DARK: Color32 = Color32::from_rgb(18, 18, 26);
    pub const CANVAS_BG_LIGHT: Color32 = Color32::from_rgb(241, 239, 231);
    pub const GRID_DARK: Color32 = Color32::from_rgba_premultiplied(60, 60, 80, 40);
    pub const GRID_LIGHT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 10);
    pub const SIDEBAR_BG_DARK: Color32 = Color32::from_rgb(22, 22, 32);
    pub const SIDEBAR_BG_LIGHT: Color32 = Color32::from_rgb(255, 255, 255);
    pub const SECTION_TITLE_DARK: Color32 = Color32::from_rgb(160, 170, 200);
    pub const SECTION_TITLE_LIGHT: Color32 = Color32::from_rgb(80, 85, 110);
    pub const INPUT_BG_DARK: Color32 = Color32::from_rgba_premultiplied(25, 35, 60, 230);
    pub const INPUT_BG_LIGHT: Color32 = Color32::from_rgba_premultiplied(230, 240, 255, 230);
    pub const INPUT_BORDER: Color32 = Color32::from_rgba_premultiplied(99, 130, 255, 100);
    pub const COMPUTED_BG_DARK: Color32 = Color32::from_rgba_premultiplied(40, 25, 15, 230);
    pub const COMPUTED_BG_LIGHT: Color32 = Color32::from_rgba_premultiplied(255, 240, 225, 230);
    pub const COMPUTED_BORDER: Color32 = Color32::from_rgba_premultiplied(200, 140, 80, 100);
    pub const SELECTED: Color32 = Color32::from_rgb(255, 210, 60);
    pub const TEXT_PRIMARY_DARK: Color32 = Color32::from_rgb(220, 225, 240);
    pub const TEXT_PRIMARY_LIGHT: Color32 = Color32::from_rgb(30, 35, 50);
    pub const TEXT_SECONDARY_DARK: Color32 = Color32::from_rgb(140, 150, 175);
    pub const TEXT_SECONDARY_LIGHT: Color32 = Color32::from_rgb(110, 115, 135);
    pub const TEXT_VALUE_DARK: Color32 = Color32::from_rgb(130, 190, 255);
    pub const TEXT_VALUE_LIGHT: Color32 = Color32::from_rgb(30, 100, 200);
    pub const CARD_BG_DARK: Color32 = Color32::from_rgba_premultiplied(30, 35, 50, 180);
    pub const CARD_BG_LIGHT: Color32 = Color32::from_rgba_premultiplied(240, 242, 248, 180);
    pub const CARD_HOVER_DARK: Color32 = Color32::from_rgba_premultiplied(40, 48, 70, 200);
    pub const CARD_HOVER_LIGHT: Color32 = Color32::from_rgba_premultiplied(225, 230, 245, 200);
    pub const CARD_SELECTED_DARK: Color32 = Color32::from_rgba_premultiplied(50, 65, 100, 230);
    pub const CARD_SELECTED_LIGHT: Color32 = Color32::from_rgba_premultiplied(200, 215, 240, 230);
    pub const STATUS_DARK: Color32 = Color32::from_rgba_premultiplied(140, 150, 175, 160);
    pub const STATUS_LIGHT: Color32 = Color32::from_rgba_premultiplied(110, 115, 135, 160);
    pub const PLOT_BG_DARK: Color32 = Color32::from_rgba_premultiplied(20, 22, 32, 230);
    pub const PLOT_BG_LIGHT: Color32 = Color32::from_rgba_premultiplied(245, 245, 250, 230);
}

// ── ThemeColors resolver ──────────────────────────────────────────────

struct ThemeColors {
    canvas_bg: Color32, grid: Color32, sidebar_bg: Color32, section_title: Color32,
    input_bg: Color32, input_border: Color32, computed_bg: Color32, computed_border: Color32,
    text_primary: Color32, text_secondary: Color32, text_value: Color32,
    card_bg: Color32, card_hover: Color32, card_selected: Color32,
    status: Color32, plot_bg: Color32,
}

impl ThemeColors {
    fn resolve(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Self {
                canvas_bg: palette::CANVAS_BG_DARK, grid: palette::GRID_DARK,
                sidebar_bg: palette::SIDEBAR_BG_DARK, section_title: palette::SECTION_TITLE_DARK,
                input_bg: palette::INPUT_BG_DARK, input_border: palette::INPUT_BORDER,
                computed_bg: palette::COMPUTED_BG_DARK, computed_border: palette::COMPUTED_BORDER,
                text_primary: palette::TEXT_PRIMARY_DARK, text_secondary: palette::TEXT_SECONDARY_DARK,
                text_value: palette::TEXT_VALUE_DARK,
                card_bg: palette::CARD_BG_DARK, card_hover: palette::CARD_HOVER_DARK,
                card_selected: palette::CARD_SELECTED_DARK, status: palette::STATUS_DARK,
                plot_bg: palette::PLOT_BG_DARK,
            },
            Theme::Light => Self {
                canvas_bg: palette::CANVAS_BG_LIGHT, grid: palette::GRID_LIGHT,
                sidebar_bg: palette::SIDEBAR_BG_LIGHT, section_title: palette::SECTION_TITLE_LIGHT,
                input_bg: palette::INPUT_BG_LIGHT, input_border: palette::INPUT_BORDER,
                computed_bg: palette::COMPUTED_BG_LIGHT, computed_border: palette::COMPUTED_BORDER,
                text_primary: palette::TEXT_PRIMARY_LIGHT, text_secondary: palette::TEXT_SECONDARY_LIGHT,
                text_value: palette::TEXT_VALUE_LIGHT,
                card_bg: palette::CARD_BG_LIGHT, card_hover: palette::CARD_HOVER_LIGHT,
                card_selected: palette::CARD_SELECTED_LIGHT, status: palette::STATUS_LIGHT,
                plot_bg: palette::PLOT_BG_LIGHT,
            },
        }
    }
}

// ── Main entry ────────────────────────────────────────────────────────

pub fn show_component_canvas(ui: &mut Ui, state: &mut AppState) {
    let colors = ThemeColors::resolve(state.theme);

    egui::Panel::left("canvas_sidebar")
        .resizable(true).default_size(220.0).min_size(180.0)
        .frame(egui::Frame { fill: colors.sidebar_bg, inner_margin: egui::Margin::symmetric(12, 8), ..Default::default() })
        .show_inside(ui, |ui| { draw_sidebar(ui, state, &colors); });

    egui::CentralPanel::default()
        .frame(egui::Frame { fill: colors.canvas_bg, ..Default::default() })
        .show_inside(ui, |ui| { handle_canvas(ui, state, &colors); });

    handle_keyboard(ui, state);
}

// ── Sidebar ───────────────────────────────────────────────────────────

fn draw_sidebar(ui: &mut Ui, state: &mut AppState, colors: &ThemeColors) {
    let cc = &mut state.component_canvas;

    section_header(ui, "INPUTS", colors);
    ui.add_space(6.0);
    for &(ct, name, icon) in &[
        (CanvasComponentType::Vin, "Vin", "⚡"),
        (CanvasComponentType::Vout, "Vout", "🔌"),
        (CanvasComponentType::DutyCycle, "Duty Cycle", "〰"),
        (CanvasComponentType::Frequency, "Frequency", "📡"),
        (CanvasComponentType::DeltaIl, "ΔiL", "📉"),
        (CanvasComponentType::IoutMax, "Iout,max", "💧"),
        (CanvasComponentType::DeltaVo, "ΔVo", "📊"),
    ] {
        draw_palette_card(ui, cc, ct, name, icon, colors);
    }

    ui.add_space(12.0);
    section_header(ui, "COMPUTED", colors);
    ui.add_space(6.0);
    for &(ct, name, icon) in &[
        (CanvasComponentType::Inductor, "Inductor (L)", "〰"),
        (CanvasComponentType::Capacitor, "Capacitor (C)", "‖‖"),
    ] {
        draw_palette_card(ui, cc, ct, name, icon, colors);
    }

    ui.add_space(12.0);
    section_header(ui, "VIZ", colors);
    ui.add_space(6.0);
    draw_palette_card(ui, cc, CanvasComponentType::Plot, "Curve Plot", "📈", colors);

    ui.add_space(16.0);
    section_header(ui, "PARAMETERS", colors);
    ui.add_space(6.0);

    let mut changed = false;
    changed |= param_row(ui, "Vin", &mut cc.shared_params.vin, 1.0, 500.0, 1.0, "V", colors);
    changed |= param_row(ui, "Vout", &mut cc.shared_params.vout, 0.5, 500.0, 1.0, "V", colors);
    if changed && cc.shared_params.vin > 0.0 {
        cc.shared_params.duty_cycle = (cc.shared_params.vout / cc.shared_params.vin).clamp(0.0, 1.0);
    }
    let mut dc = cc.shared_params.duty_cycle * 100.0;
    if param_row(ui, "D", &mut dc, 1.0, 99.0, 0.5, "%", colors) {
        cc.shared_params.duty_cycle = (dc / 100.0).clamp(0.0, 1.0);
        cc.shared_params.vout = cc.shared_params.vin * cc.shared_params.duty_cycle;
        changed = true;
    }
    changed |= param_row(ui, "Freq", &mut cc.shared_params.frequency, 100.0, 1_000_000.0, 1000.0, "Hz", colors);
    changed |= param_pct(ui, "ΔiL", &mut cc.shared_params.delta_il, 0.001, 1.0, colors);
    changed |= param_row(ui, "Iout,max", &mut cc.shared_params.iout_max, 0.1, 100.0, 0.2, "A", colors);
    changed |= param_pct(ui, "ΔVo", &mut cc.shared_params.delta_vo, 0.0001, 0.5, colors);

    ui.add_space(16.0);
    section_header(ui, "RESULTS", colors);
    ui.add_space(6.0);
    result_row(ui, "L", cc.shared_params.calc_inductance(), "H", colors);
    result_row(ui, "C", cc.shared_params.calc_capacitance(), "F", colors);
    result_row(ui, "ΔiL(A)", cc.shared_params.calc_delta_il_amps(), "A", colors);
    if changed { state.status_message = "Parameters updated".to_owned(); }
}

// ── Sidebar helpers ───────────────────────────────────────────────────

fn section_header(ui: &mut Ui, text: &str, colors: &ThemeColors) {
    ui.add_space(2.0);
    ui.label(egui::RichText::new(text).color(colors.section_title).size(10.0).strong().monospace());
    ui.separator();
}

fn draw_palette_card(ui: &mut Ui, cc: &mut crate::app::state::ComponentCanvasState, ctype: CanvasComponentType, name: &str, icon: &str, colors: &ThemeColors) {
    let selected = cc.palette_selection == Some(ctype);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), egui::Sense::click());
    let bg = if selected { colors.card_selected } else if response.hovered() { colors.card_hover } else { colors.card_bg };
    ui.painter().rect_filled(rect, CornerRadius::same(4), bg);
    if selected {
        ui.painter().rect_filled(Rect::from_min_size(Pos2::new(rect.min.x, rect.min.y), Vec2::new(3.0, rect.height())), CornerRadius::same(2), palette::ACCENT);
    }
    ui.painter().text(Pos2::new(rect.min.x + 12.0, rect.center().y), Align2::LEFT_CENTER, &format!("{}  {}", icon, name), egui::TextStyle::Body.resolve(ui.style()), colors.text_primary);
    if response.clicked() { cc.palette_selection = Some(ctype); }
}

fn param_row(ui: &mut Ui, label: &str, value: &mut f64, min: f64, max: f64, speed: f64, suffix: &str, colors: &ThemeColors) -> bool {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(colors.text_secondary).size(11.0).monospace());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(egui::DragValue::new(value).speed(speed).range(min..=max).suffix(suffix)).changed()
        }).inner
    }).inner
}

fn param_pct(ui: &mut Ui, label: &str, value: &mut f64, min: f64, max: f64, colors: &ThemeColors) -> bool {
    let mut display = *value * 100.0;
    let changed = ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(colors.text_secondary).size(11.0).monospace());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(egui::DragValue::new(&mut display).speed(0.5).range(min * 100.0..=max * 100.0).suffix(" %")).changed()
        }).inner
    }).inner;
    if changed { *value = (display / 100.0).clamp(min, max); }
    changed
}

fn result_row(ui: &mut Ui, label: &str, value: f64, unit: &str, colors: &ThemeColors) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(colors.text_secondary).size(11.0).monospace());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(format_eng_small(value, unit)).color(colors.text_value).size(11.0).monospace());
        });
    });
}

// ── Canvas ────────────────────────────────────────────────────────────

fn handle_canvas(ui: &mut Ui, state: &mut AppState, colors: &ThemeColors) {
    let available = ui.available_size();
    let (response, painter) = ui.allocate_painter(available, egui::Sense::click_and_drag());
    let cc = &mut state.component_canvas;
    let origin = Pos2::new(response.rect.min.x + cc.pan_offset.0, response.rect.min.y + cc.pan_offset.1);
    let zoom = cc.zoom;

    let scroll = ui.input(|i| i.smooth_scroll_delta().y);
    if scroll != 0.0 {
        if let Some(cursor) = response.hover_pos() {
            let factor = if scroll > 0.0 { 1.1 } else { 0.9 };
            let nz = (cc.zoom * factor).clamp(0.2, 5.0);
            if (nz - cc.zoom).abs() > 0.001 {
                let cr = cursor - response.rect.min.to_vec2();
                let wb = Vec2::new((cr.x - cc.pan_offset.0) / cc.zoom, (cr.y - cc.pan_offset.1) / cc.zoom);
                cc.zoom = nz; cc.pan_offset.0 = cr.x - wb.x * nz; cc.pan_offset.1 = cr.y - wb.y * nz;
                ui.ctx().request_repaint();
            }
        } else { cc.zoom = (cc.zoom * if scroll > 0.0 { 1.1 } else { 0.9 }).clamp(0.2, 5.0); ui.ctx().request_repaint(); }
    }

    if response.dragged_by(egui::PointerButton::Primary) && cc.selected_index.is_none() {
        let d = response.drag_delta();
        cc.pan_offset.0 += d.x; cc.pan_offset.1 += d.y; ui.ctx().request_repaint();
    }

    if response.clicked_by(egui::PointerButton::Primary) {
        if let Some(cursor) = response.interact_pointer_pos() {
            let cp = screen_to_element(cursor, origin, zoom);
            let hit = find_component_at(&cc.placed_components, cp);
            if let Some(idx) = hit {
                cc.palette_selection = None; cc.selected_index = Some(idx);
                state.status_message = format!("Selected {}", cc.placed_components[idx].component_type.name());
            } else if let Some(ct) = cc.palette_selection {
                cc.place_component(ct, snap(cp, GRID_SPACING)); cc.palette_selection = None;
                state.status_message = format!("Placed {}", ct.name());
            } else { cc.selected_index = None; }
            ui.ctx().request_repaint();
        }
    }

    if response.clicked_by(egui::PointerButton::Secondary) {
        if let Some(cursor) = response.interact_pointer_pos() {
            let cp = screen_to_element(cursor, origin, zoom);
            if let Some(idx) = find_component_at(&cc.placed_components, cp) {
                cc.selected_index = Some(idx); cc.delete_selected();
                state.status_message = "Component deleted".to_owned(); ui.ctx().request_repaint();
            }
        }
    }

    draw_grid(&painter, origin, response.rect, zoom, colors);

    if let Some(si) = cc.selected_index {
        if si < cc.placed_components.len() {
            let ct = cc.placed_components[si].component_type;
            if ct.is_editable() && !ct.is_plot() { draw_editable_ui(ui, si, cc, origin, zoom, &response.rect, colors); }
        }
    }

    for (idx, comp) in cc.placed_components.iter().enumerate() {
        if comp.component_type.is_plot() { continue; }
        let sel = cc.selected_index == Some(idx);
        if sel && comp.component_type.is_editable() { continue; }
        draw_component_block(&painter, comp, origin, zoom, sel, cc.get_value(comp.component_type), colors);
    }

    for (idx, comp) in cc.placed_components.iter().enumerate() {
        if !comp.component_type.is_plot() { continue; }
        let sel = cc.selected_index == Some(idx);
        let rect = plot_block_rect(comp.pos, origin, zoom);
        let clip = rect.intersect(response.rect);
        if clip.is_positive() {
            let pr = clip.shrink(4.0);
            let (_pr, pp) = ui.allocate_painter(pr.size(), egui::Sense::click());
            pp.rect_filled(Rect::from_min_size(Pos2::ZERO, pr.size()), CornerRadius::same(8), colors.plot_bg);
            pp.rect_stroke(Rect::from_min_size(Pos2::ZERO, pr.size()), CornerRadius::same(8),
                Stroke::new(if sel { 2.0 } else { 1.0 }, if sel { palette::SELECTED } else { colors.input_border }), egui::StrokeKind::Outside);
            let cr = egui::Rect::from_min_size(Pos2::ZERO, pr.size() - Vec2::new(8.0, 8.0));
            let mut cu = ui.new_child(egui::UiBuilder::new()
                .max_rect(egui::Rect::from_min_size(pr.min + Vec2::new(4.0, 4.0), cr.size())).layout(*ui.layout()));
            draw_plot(&mut cu, &cc.shared_params, state.theme);
        }
    }

    if let Some(ct) = cc.palette_selection {
        if let Some(cursor) = response.hover_pos() {
            let sn = snap(screen_to_element(cursor, origin, zoom), GRID_SPACING);
            let (r, l) = if ct.is_plot() { (plot_block_rect(sn, origin, zoom), "📈 Plot".to_owned()) }
                else { (block_rect(sn, origin, zoom), format!("{}: {}", ct.name(), format_eng_small(cc.get_value(ct), ct.unit()))) };
            painter.rect_stroke(r, CornerRadius::same(4), Stroke::new(1.5, palette::ACCENT_LIGHT), egui::StrokeKind::Outside);
            painter.rect_filled(r, CornerRadius::same(4), Color32::from_rgba_premultiplied(99, 130, 255, 20));
            painter.text(r.center(), Align2::CENTER_CENTER, &l, egui::TextStyle::Monospace.resolve(ui.style()), palette::ACCENT_LIGHT);
        }
    }

    painter.text(Pos2::new(response.rect.min.x + 12.0, response.rect.max.y - 12.0), Align2::LEFT_BOTTOM,
        format!("{} components  ·  {:.0}% zoom  ·  Click to place  ·  Right-click to delete  ·  Scroll to zoom", cc.placed_components.len(), cc.zoom * 100.0),
        egui::TextStyle::Monospace.resolve(&egui::Style::default()), colors.status);
}

// ── Inline editor ─────────────────────────────────────────────────────

fn draw_editable_ui(ui: &mut Ui, idx: usize, cc: &mut crate::app::state::ComponentCanvasState, origin: Pos2, zoom: f32, canvas_rect: &Rect, colors: &ThemeColors) {
    let comp = &cc.placed_components[idx];
    let rect = block_rect(comp.pos, origin, zoom);
    let clip = rect.intersect(*canvas_rect);
    if !clip.is_positive() { return; }
    let ct = comp.component_type;

    let mut cu = ui.new_child(egui::UiBuilder::new()
        .max_rect(egui::Rect::from_min_size(clip.min, Vec2::new(clip.size().x, clip.size().y)))
        .layout(egui::Layout::top_down_justified(egui::Align::Center)));
    let p = cu.painter();

    p.rect_filled(Rect::from_min_size(Pos2::ZERO, clip.size()), CornerRadius::same(8), colors.input_bg);
    p.rect_filled(Rect::from_min_size(Pos2::ZERO, Vec2::new(clip.size().x, 3.0)), CornerRadius::same(2), palette::ACCENT);
    p.rect_stroke(Rect::from_min_size(Pos2::ZERO, clip.size()), CornerRadius::same(8), Stroke::new(2.0, palette::SELECTED), egui::StrokeKind::Outside);

    cu.add_space(4.0);
    cu.label(egui::RichText::new(ct.name()).color(colors.text_primary).monospace().size(11.0).strong());

    let (lo, hi, spd, sfx) = match ct {
        CanvasComponentType::Vin => (1.0, 500.0, 1.0, "V"),
        CanvasComponentType::Vout => (0.5, 500.0, 1.0, "V"),
        CanvasComponentType::DutyCycle => (1.0, 99.0, 0.5, "%"),
        CanvasComponentType::Frequency => (100.0, 1_000_000.0, 1000.0, "Hz"),
        CanvasComponentType::DeltaIl => (0.1, 100.0, 0.5, "%"),
        CanvasComponentType::IoutMax => (0.1, 100.0, 0.2, "A"),
        CanvasComponentType::DeltaVo => (0.01, 50.0, 0.1, "%"),
        _ => return,
    };

    // Always start from the current shared value so slider + input stay in sync
    let mut v = cc.get_value(ct);

    if cu.add(egui::Slider::new(&mut v, lo..=hi).suffix(sfx).show_value(false).clamping(egui::SliderClamping::Never)).changed() {
        cc.set_value(ct, v);
        cu.ctx().request_repaint();
    }

    cu.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(egui::RichText::new("=").color(colors.text_secondary).monospace());
        if ui.add(egui::DragValue::new(&mut v).speed(spd).suffix(sfx)).changed() {
            cc.set_value(ct, v);
            ui.ctx().request_repaint();
        }
    });

    cu.add_space(2.0);
    cu.label(egui::RichText::new(format!("L {}  C {}", format_eng_small(cc.shared_params.calc_inductance(), "H"), format_eng_small(cc.shared_params.calc_capacitance(), "F")))
        .color(colors.text_value).size(9.0).monospace());
    if cu.ctx().is_pointer_over_egui() { cu.ctx().request_repaint(); }
}

// ── Static block ──────────────────────────────────────────────────────

fn draw_component_block(painter: &egui::Painter, component: &crate::app::state::PlacedComponent, origin: Pos2, zoom: f32, selected: bool, value: f64, colors: &ThemeColors) {
    let rect = block_rect(component.pos, origin, zoom);
    let edit = component.component_type.is_editable();
    let bg = if edit { colors.input_bg } else { colors.computed_bg };
    let bc = if selected { palette::SELECTED } else if edit { colors.input_border } else { colors.computed_border };
    let accent = if edit { palette::ACCENT } else { palette::ACCENT_DIM };

    painter.rect_filled(rect, CornerRadius::same(8), bg);
    painter.rect_filled(Rect::from_min_size(Pos2::new(rect.min.x, rect.min.y), Vec2::new(rect.width(), 3.0)), CornerRadius::same(2), accent);
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(if selected { 2.0 } else { 1.0 }, bc), egui::StrokeKind::Outside);

    painter.text(Pos2::new(rect.min.x + 8.0, rect.min.y + 10.0), Align2::LEFT_TOP, component.component_type.name(),
        egui::TextStyle::Monospace.resolve(&egui::Style::default()), colors.text_primary);
    painter.text(Pos2::new(rect.center().x, rect.max.y - 10.0), Align2::CENTER_BOTTOM, &format_eng_small(value, component.component_type.unit()),
        egui::TextStyle::Monospace.resolve(&egui::Style::default()), if selected { palette::SELECTED } else { colors.text_value });

    if edit && !selected {
        painter.text(Pos2::new(rect.right() - 4.0, rect.min.y + 10.0), Align2::RIGHT_TOP, "click",
            egui::TextStyle::Monospace.resolve(&egui::Style::default()),
            Color32::from_rgba_premultiplied(colors.text_secondary.r(), colors.text_secondary.g(), colors.text_secondary.b(), 80));
    }
}

// ── Plot ──────────────────────────────────────────────────────────────

fn draw_plot(ui: &mut Ui, params: &crate::app::state::SharedParams, _theme: Theme) {
    let l = params.calc_inductance();
    let c_val = params.calc_capacitance();
    let f = params.frequency;
    let d = params.duty_cycle;
    let vin = params.vin;
    let vout = params.vout;
    let iout_max = params.iout_max;
    let _dil_pct = params.delta_il;

    // Show a few switching cycles in time domain
    let period = if f > 0.0 { 1.0 / f } else { 1e-5 };
    let t_end = 4.0 * period; // show 4 switching cycles
    let n_pts = 400;
    let dt = t_end / n_pts as f64;

    let mut il_wave: Vec<[f64; 2]> = Vec::with_capacity(n_pts + 1);
    let mut vc_wave: Vec<[f64; 2]> = Vec::with_capacity(n_pts + 1);
    let mut switch_wave: Vec<[f64; 2]> = Vec::with_capacity(n_pts + 1);

    // Steady-state approximate initial conditions for a buck converter
    // iL_avg = Iout ≈ Vout / Rload (use iout_max as proxy for nominal)
    // ΔiL_pp = (Vin - Vout) * D / (L * f)  — ripple peak-to-peak
    let il_avg = iout_max;
    let rip_amps = if l > 0.0 && f > 0.0 {
        ((vin - vout) * d) / (l * f)
    } else {
        0.0
    };
    let il_initial = il_avg - rip_amps / 2.0;

    // Output voltage steady-state ≈ Vout, with small ripple
    let vc_initial = vout;

    let mut il = il_initial;
    let mut vc = vc_initial;

    for i in 0..=n_pts {
        let t = i as f64 * dt;
        let phase = (t % period) / period;
        let s = if phase < d { 1.0 } else { 0.0 };

        // Simple piecewise-linear integration for buck converter:
        // ON:  diL/dt = (Vin - Vc) / L , dvC/dt = (iL - Vc/R) / C
        // OFF: diL/dt = -Vc / L        , dvC/dt = (iL - Vc/R) / C
        if l > 0.0 {
            let dil_dt = (s * vin - vc) / l;
            il += dil_dt * dt;
        }
        if c_val > 0.0 && l > 0.0 {
            let dvc_dt = (il - vc / (vout / iout_max.max(0.01))) / c_val;
            vc += dvc_dt * dt;
        }

        // Convert time to μs or ms for display
        let t_display = if t_end >= 1e-3 { t * 1e3 } else { t * 1e6 };

        il_wave.push([t_display, il]);
        vc_wave.push([t_display, vc]);
        switch_wave.push([t_display, s * il_avg * 1.2]); // scaled switch state for visibility
    }

    let t_unit = if t_end >= 1e-3 { "ms" } else { "μs" };

    Plot::new("canvas_plot").legend(Legend::default())
        .height(ui.available_height().max(100.0)).width(ui.available_width().max(150.0))
        .x_axis_label(&format!("Time [{}]", t_unit)).y_axis_label("Value")
        .allow_drag(false).allow_zoom(false).allow_scroll(false).allow_boxed_zoom(false)
        .show(ui, |pu| {
            pu.line(Line::new("iL [A]", PlotPoints::from(il_wave)).color(Color32::from_rgb(255, 165, 0)).width(1.5));
            pu.line(Line::new("Vout [V]", PlotPoints::from(vc_wave)).color(Color32::from_rgb(100, 200, 255)).width(1.5));
            pu.line(Line::new("Switch", PlotPoints::from(switch_wave)).color(Color32::from_rgb(160, 160, 180)).width(1.0).style(LineStyle::Dashed { length: 6.0 }));
        });
}

// ── Keyboard ──────────────────────────────────────────────────────────

fn handle_keyboard(ui: &mut Ui, state: &mut AppState) {
    let cc = &mut state.component_canvas;
    if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) && cc.selected_index.is_some() {
        cc.delete_selected();
        state.status_message = "Component deleted".to_owned();
        ui.ctx().request_repaint();
    }
}

// ── Geometry & Helpers ────────────────────────────────────────────────

fn block_rect(pos: Pos, origin: Pos2, zoom: f32) -> Rect {
    Rect::from_min_size(Pos2::new(origin.x + pos.x * zoom, origin.y + pos.y * zoom), Vec2::new(BLOCK_W * zoom, BLOCK_H * zoom))
}
fn plot_block_rect(pos: Pos, origin: Pos2, zoom: f32) -> Rect {
    Rect::from_min_size(Pos2::new(origin.x + pos.x * zoom, origin.y + pos.y * zoom), Vec2::new(PLOT_BLOCK_W * zoom, PLOT_BLOCK_H * zoom))
}
fn draw_grid(painter: &egui::Painter, origin: Pos2, rect: Rect, zoom: f32, colors: &ThemeColors) {
    let gs = GRID_SPACING * zoom; let mut x = (origin.x % gs) - gs;
    while x < rect.max.x { painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], Stroke::new(0.5, colors.grid)); x += gs; }
    let mut y = (origin.y % gs) - gs;
    while y < rect.max.y { painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], Stroke::new(0.5, colors.grid)); y += gs; }
}
fn screen_to_element(screen: Pos2, origin: Pos2, zoom: f32) -> Pos {
    Pos::new((screen.x - origin.x) / zoom, (screen.y - origin.y) / zoom)
}
fn find_component_at(components: &[crate::app::state::PlacedComponent], point: Pos) -> Option<usize> {
    for (idx, comp) in components.iter().enumerate().rev() {
        let (hw, hh) = if comp.component_type.is_plot() { (PLOT_BLOCK_W / 2.0, PLOT_BLOCK_H / 2.0) } else { (BLOCK_W / 2.0, BLOCK_H / 2.0) };
        if point.x >= comp.pos.x - hw && point.x <= comp.pos.x + hw && point.y >= comp.pos.y - hh && point.y <= comp.pos.y + hh { return Some(idx); }
    }
    None
}
fn snap(pos: Pos, grid: f32) -> Pos { Pos::new((pos.x /
grid).round() * grid, (pos.y / grid).round() * grid) }
fn format_value(value: f64, unit: &str) -> String {
    let av = value.abs();
    if av == 0.0 { return format!("0 {}", unit); }
    let (scaled, prefix) = if av >= 1_000_000.0 { (value / 1_000_000.0, "M") } else if av >= 1_000.0 { (value / 1_000.0, "k") }
    else if av >= 1.0 { (value, "") } else if av >= 0.001 { (value * 1_000.0, "m") } else if av >= 0.000_001 { (value * 1_000_000.0, "μ") }
    else if av >= 1e-9 { (value * 1e9, "n") } else { (value * 1e12, "p") };
    let dec = if scaled.abs() >= 100.0 { 1 } else if scaled.abs() >= 10.0 { 2 } else { 3 };
    format!("{:.prec$} {}{}", scaled, prefix, unit, prec = dec)
}
fn format_eng_small(value: f64, unit: &str) -> String { format_value(value, unit) }
