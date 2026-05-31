//! Design tokens — a Rerum-inspired palette for Tupan.
//!
//! This mirrors the structure of Rerun's `re_ui::DesignTokens` but is standalone,
//! using a minimal color-table + RON approach.

use std::collections::HashMap;

use egui::{Color32, CornerRadius, Margin, Stroke, Vec2};

// ── Color table ───────────────────────────────────────────────────────

/// A flat mapping of `"{Hue}.{Scale}"` strings to hex colors.
type ColorTable = HashMap<String, Color32>;

fn load_color_table(ron_str: &str) -> ColorTable {
    let value: ron::Value = ron::from_str(ron_str).expect("Failed to parse color_table.ron");
    let mut table = HashMap::new();

    // Navigate to Global.Color
    let global = match &value {
        ron::Value::Map(root) => root
            .iter()
            .find(|(k, _)| matches!(k, ron::Value::String(s) if s == "Global"))
            .and_then(|(_, v)| match v {
                ron::Value::Map(m) => m.iter().find(|(k, _)| matches!(k, ron::Value::String(s) if s == "Color")),
                _ => None,
            })
            .map(|(_, v)| v),
        _ => None,
    }
    .expect("Missing Global.Color in color_table.ron");

    fn collect_hue(
        table: &mut ColorTable,
        hue_name: &str,
        hue: &ron::Value,
    ) {
        if let ron::Value::Map(map) = hue {
            for (k, v) in map.iter() {
                if let ron::Value::String(scale) = k {
                    if let ron::Value::String(hex) = v {
                        let key = format!("{}.{}", hue_name, scale);
                        let color = Color32::from_hex(hex.as_str()).unwrap_or_else(|_| {
                            panic!("Invalid hex color for {key}: {hex}")
                        });
                        table.insert(key, color);
                    }
                }
            }
        }
    }

    if let ron::Value::Map(hues) = global {
        for (hue_name, hue_value) in hues.iter() {
            if let ron::Value::String(name) = hue_name {
                collect_hue(&mut table, name, hue_value);
            }
        }
    }

    table
}

fn resolve_value<'v>(value: &'v ron::Value, path: &str) -> &'v ron::Value {
    let mut current = value;
    for component in path.split('.') {
        match current {
            ron::Value::Map(map) => {
                let found = map
                    .iter()
                    .find(|(k, _)| matches!(k, ron::Value::String(s) if s == component))
                    .map(|(_, v)| v);
                current = found.unwrap_or_else(|| panic!("Missing path component '{component}' in '{path}'"));
            }
            _ => panic!("Expected Map at '{component}' in '{path}'"),
        }
    }
    current
}

fn resolve_color(
    color_table: &ColorTable,
    value: &ron::Value,
) -> Color32 {
    match value {
        ron::Value::String(s) => {
            if let Some(hex) = s.strip_prefix('#') {
                Color32::from_hex(hex).expect("Invalid hex color")
            } else if let Some(path) = s
                .strip_prefix('{')
                .and_then(|s| s.strip_suffix('}'))
            {
                color_table
                    .get(path)
                    .copied()
                    .unwrap_or_else(|| panic!("Missing color in table: {path}"))
            } else {
                panic!("Unexpected color string format: {s}")
            }
        }
        _ => panic!("Expected string for color value, got {value:?}"),
    }
}

// ── DesignTokens ──────────────────────────────────────────────────────

/// Tupan design tokens — the complete palette and style parameters.
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub theme: egui::Theme,

    // Sizing
    pub small_icon_size: Vec2,
    pub card_corner_radius: CornerRadius,
    pub window_corner_radius: CornerRadius,
    pub view_padding: i8,

    // Core colors
    pub top_bar_color: Color32,
    pub panel_bg_color: Color32,
    pub canvas_bg_color: Color32,
    pub sidebar_bg_color: Color32,
    pub section_header_color: Color32,

    // Card / surface colors
    pub card_bg_color: Color32,
    pub card_hover_color: Color32,
    pub card_selected_color: Color32,

    // Input / editable colors
    pub input_bg_color: Color32,
    pub input_border_color: Color32,

    // Computed / read-only colors
    pub computed_bg_color: Color32,
    pub computed_border_color: Color32,

    // Accent / selection
    pub accent_color: Color32,
    pub accent_dim_color: Color32,
    pub accent_light_color: Color32,
    pub selected_color: Color32,
    pub selection_bg_fill: Color32,
    pub selection_stroke_color: Color32,

    // Text
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_value: Color32,
    pub text_strong: Color32,
    pub text_subdued: Color32,

    // Canvas
    pub grid_color: Color32,
    pub plot_bg_color: Color32,
    pub status_color: Color32,

    // Semantic colors
    pub error_fg_color: Color32,
    pub warn_fg_color: Color32,
    pub success_fg_color: Color32,
    pub highlight_color: Color32,

    // Widget states
    pub widget_inactive_bg: Color32,
    pub widget_hovered_color: Color32,
    pub widget_active_color: Color32,
    pub text_edit_bg_color: Color32,

    // Overlays
    pub popup_shadow_color: Color32,
    pub floating_color: Color32,
    pub faint_bg_color: Color32,
}

impl DesignTokens {
    /// Load design tokens from a RON string.
    pub fn load(theme: egui::Theme, tokens_ron: &str) -> Result<Self, String> {
        let color_table = load_color_table(include_str!("data/color_table.ron"));
        let theme_json: ron::Value =
            ron::from_str(tokens_ron).map_err(|e| format!("Failed to parse theme RON: {e}"))?;

        let get_color = |name: &str| -> Color32 {
            let val = resolve_value(&theme_json, name);
            resolve_color(&color_table, val)
        };

        Ok(Self {
            theme,
            small_icon_size: Vec2::splat(14.0),
            card_corner_radius: CornerRadius::same(6),
            window_corner_radius: CornerRadius::same(6),
            view_padding: 12,

            top_bar_color: get_color("top_bar_color"),
            panel_bg_color: get_color("panel_bg_color"),
            canvas_bg_color: get_color("canvas_bg_color"),
            sidebar_bg_color: get_color("sidebar_bg_color"),
            section_header_color: get_color("section_header_color"),

            card_bg_color: get_color("card_bg_color"),
            card_hover_color: get_color("card_hover_color"),
            card_selected_color: get_color("card_selected_color"),

            input_bg_color: get_color("input_bg_color"),
            input_border_color: get_color("input_border_color"),

            computed_bg_color: get_color("computed_bg_color"),
            computed_border_color: get_color("computed_border_color"),

            accent_color: get_color("accent_color"),
            accent_dim_color: get_color("accent_dim_color"),
            accent_light_color: get_color("accent_light_color"),
            selected_color: get_color("selected_color"),
            selection_bg_fill: get_color("selection_bg_fill"),
            selection_stroke_color: get_color("selection_stroke_color"),

            text_primary: get_color("text_primary_color"),
            text_secondary: get_color("text_secondary_color"),
            text_value: get_color("text_value_color"),
            text_strong: get_color("text_strong_color"),
            text_subdued: get_color("text_subdued_color"),

            grid_color: get_color("grid_color"),
            plot_bg_color: get_color("plot_bg_color"),
            status_color: get_color("status_color"),

            error_fg_color: get_color("error_fg_color"),
            warn_fg_color: get_color("warn_fg_color"),
            success_fg_color: get_color("success_fg_color"),
            highlight_color: get_color("highlight_color"),

            widget_inactive_bg: get_color("widget_inactive_bg"),
            widget_hovered_color: get_color("widget_hovered_color"),
            widget_active_color: get_color("widget_active_color"),
            text_edit_bg_color: get_color("text_edit_bg_color"),

            popup_shadow_color: get_color("popup_shadow_color"),
            floating_color: get_color("floating_color"),
            faint_bg_color: get_color("faint_bg_color"),
        })
    }

    // ── Style application ──────────────────────────────────────────

    /// Apply these tokens to an egui style.
    pub fn apply(&self, style: &mut egui::Style) {
        self.set_spacing(style);
        self.set_colors(style);
    }

    fn set_spacing(&self, style: &mut egui::Style) {
        style.visuals.button_frame = true;

        // Turn off strokes around buttons (like re_ui)
        style.visuals.widgets.inactive.bg_stroke = Default::default();
        style.visuals.widgets.hovered.bg_stroke = Default::default();
        style.visuals.widgets.active.bg_stroke = Default::default();
        style.visuals.widgets.open.bg_stroke = Default::default();

        // Expansion on hover/active
        style.visuals.widgets.hovered.expansion = 2.0;
        style.visuals.widgets.active.expansion = 2.0;
        style.visuals.widgets.open.expansion = 2.0;

        style.visuals.window_corner_radius = 6.0.into();
        style.visuals.menu_corner_radius = 6.0.into();

        let small_cr: CornerRadius = 4.0.into();
        style.visuals.widgets.noninteractive.corner_radius = small_cr;
        style.visuals.widgets.inactive.corner_radius = small_cr;
        style.visuals.widgets.hovered.corner_radius = small_cr;
        style.visuals.widgets.active.corner_radius = small_cr;
        style.visuals.widgets.open.corner_radius = small_cr;

        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = Vec2::new(1.0, 0.0);
        style.spacing.indent = 14.0;

        style.spacing.scroll.bar_inner_margin = 2.0;
        style.spacing.scroll.bar_width = 6.0;
        style.spacing.scroll.bar_outer_margin = 2.0;

        style.visuals.clip_rect_margin = 0.0;
        style.visuals.striped = false;
        style.visuals.indent_has_left_vline = false;
        style.visuals.image_loading_spinners = false;
    }

    fn set_colors(&self, style: &mut egui::Style) {
        style.visuals.faint_bg_color = self.faint_bg_color;
        style.visuals.extreme_bg_color = self.widget_active_color;
        style.visuals.text_edit_bg_color = Some(self.text_edit_bg_color);

        style.visuals.widgets.noninteractive.weak_bg_fill = self.panel_bg_color;
        style.visuals.widgets.noninteractive.bg_fill = self.panel_bg_color;

        style.visuals.widgets.inactive.weak_bg_fill = Default::default();
        style.visuals.widgets.inactive.bg_fill = self.widget_inactive_bg;

        // Hovered / active / open
        let hovered = self.widget_hovered_color;
        style.visuals.widgets.hovered.weak_bg_fill = hovered;
        style.visuals.widgets.hovered.bg_fill = hovered;
        style.visuals.widgets.active.weak_bg_fill = hovered;
        style.visuals.widgets.active.bg_fill = hovered;
        style.visuals.widgets.open.weak_bg_fill = hovered;
        style.visuals.widgets.open.bg_fill = hovered;

        // Selection
        style.visuals.selection.bg_fill = self.selection_bg_fill;
        style.visuals.selection.stroke.color = self.selection_stroke_color;

        // Stroke colors
        style.visuals.widgets.noninteractive.bg_stroke.color =
            self.input_border_color.gamma_multiply(0.4);

        // Text colors per widget state
        style.visuals.widgets.noninteractive.fg_stroke.color = self.text_subdued;
        style.visuals.widgets.inactive.fg_stroke.color = self.text_primary;
        style.visuals.widgets.active.fg_stroke.color = self.text_strong;
        style.visuals.widgets.active.fg_stroke.width = 2.0;
        style.visuals.selection.stroke.width = 2.0;

        // Shadows
        let shadow = egui::epaint::Shadow {
            offset: [0, 15],
            blur: 50,
            spread: 0,
            color: self.popup_shadow_color,
        };
        style.visuals.popup_shadow = shadow;
        style.visuals.window_shadow = shadow;

        // Backgrounds
        style.visuals.window_fill = self.floating_color;
        style.visuals.window_stroke = Stroke::NONE;
        style.visuals.panel_fill = self.panel_bg_color;
        style.visuals.hyperlink_color = self.text_primary;

        // Semantic colors
        style.visuals.error_fg_color = self.error_fg_color;
        style.visuals.warn_fg_color = self.warn_fg_color;
    }

    // ── Layout helpers ─────────────────────────────────────────────

    pub fn top_bar_height(&self) -> f32 {
        28.0
    }

    pub fn top_bar_margin(&self) -> Margin {
        Margin::symmetric(8, 0)
    }

    pub fn panel_margin(&self) -> Margin {
        Margin::symmetric(self.view_padding, 0)
    }
}
