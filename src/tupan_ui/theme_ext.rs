//! Extension traits for [`egui::Context`] and [`egui::Style`] / [`egui::Visuals`]
//! to provide easy access to [`DesignTokens`].

use crate::tupan_ui::{DesignTokens, design_tokens_for};

/// Trait that provides access to [`DesignTokens`] from egui context / style / visuals.
pub trait ThemeExt {
    /// Get the design tokens for the current theme.
    fn tokens(&self) -> &'static DesignTokens;
}

impl ThemeExt for egui::Context {
    fn tokens(&self) -> &'static DesignTokens {
        design_tokens_for(self.theme())
    }
}

impl ThemeExt for egui::Style {
    fn tokens(&self) -> &'static DesignTokens {
        design_tokens_for(if self.visuals.dark_mode {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        })
    }
}

impl ThemeExt for egui::Visuals {
    fn tokens(&self) -> &'static DesignTokens {
        design_tokens_for(if self.dark_mode {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        })
    }
}
