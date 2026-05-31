//! Tupan UI design system — inspired by Rerun's [re_ui] crate.
//!
//! Provides a design token system with a Rerum-inspired color palette,
//! theme-aware `DesignTokens`, and extension traits for [`egui::Context`]
//! and [`egui::Ui`].

mod design_tokens;
mod theme_ext;
mod ui_ext;

pub use design_tokens::DesignTokens;
pub use theme_ext::ThemeExt;
pub use ui_ext::UiExt;

use std::sync::OnceLock;

static DARK_TOKENS: OnceLock<DesignTokens> = OnceLock::new();
static LIGHT_TOKENS: OnceLock<DesignTokens> = OnceLock::new();

/// Get the design tokens for a given egui theme.
pub fn design_tokens_for(theme: egui::Theme) -> &'static DesignTokens {
    match theme {
        egui::Theme::Dark => DARK_TOKENS.get_or_init(|| {
            DesignTokens::load(egui::Theme::Dark, include_str!("data/theme_dark.ron"))
                .expect("Failed to load dark theme design tokens")
        }),
        egui::Theme::Light => LIGHT_TOKENS.get_or_init(|| {
            DesignTokens::load(egui::Theme::Light, include_str!("data/theme_light.ron"))
                .expect("Failed to load light theme design tokens")
        }),
    }
}

/// Apply the design system styling to the egui context.
///
/// Call this once on startup after creating the egui context.
pub fn apply_style(ctx: &egui::Context) {
    for theme in [egui::Theme::Dark, egui::Theme::Light] {
        let tokens = design_tokens_for(theme);
        let mut style = std::sync::Arc::unwrap_or_clone(ctx.style_of(theme));
        tokens.apply(&mut style);
        ctx.set_style_of(theme, style);
    }
}
