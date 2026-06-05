"""Design tokens and theme management for Tupan.

Ports the design_tokens.rs RON-based color table to a Python dict,
plus PySide6 palette helpers for dark/light switching.
"""

from PySide6.QtGui import QPalette, QColor
from dataclasses import dataclass


# ── Color tables ──

DARK_COLORS = {
    "bg_primary": "#1e1e2e",
    "bg_secondary": "#181825",
    "bg_surface": "#11111b",
    "text_primary": "#cdd6f4",
    "text_secondary": "#a6adc8",
    "text_dim": "#6c7086",
    "accent": "#89b4fa",
    "accent_dim": "#74c7ec",
    "success": "#a6e3a1",
    "warning": "#f9e2af",
    "error": "#f38ba8",
    "border": "#313244",
    "card_bg": "#1e1e2e",
    "card_hover": "#2a2a3c",
    "card_selected": "#313244",
    "canvas_bg": "#11111b",
    "sidebar_bg": "#181825",
    "top_bar_bg": "#1e1e2e",
    "wire": "#cdd6f4",
    "component": "#89b4fa",
    "label": "#a6adc8",
    "ground": "#6c7086",
}

LIGHT_COLORS = {
    "bg_primary": "#eff1f5",
    "bg_secondary": "#e6e9ef",
    "bg_surface": "#dce0e8",
    "text_primary": "#4c4f69",
    "text_secondary": "#5c5f77",
    "text_dim": "#9ca0b0",
    "accent": "#1e66f5",
    "accent_dim": "#04a5e5",
    "success": "#40a02b",
    "warning": "#df8e1d",
    "error": "#d20f39",
    "border": "#ccd0da",
    "card_bg": "#eff1f5",
    "card_hover": "#e6e9ef",
    "card_selected": "#dce0e8",
    "canvas_bg": "#dce0e8",
    "sidebar_bg": "#e6e9ef",
    "top_bar_bg": "#eff1f5",
    "wire": "#4c4f69",
    "component": "#1e66f5",
    "label": "#5c5f77",
    "ground": "#9ca0b0",
}


def get_colors(dark: bool = True) -> dict:
    """Get the color table for the current theme."""
    return DARK_COLORS if dark else LIGHT_COLORS


def make_dark_palette() -> QPalette:
    """Create a dark QPalette."""
    p = QPalette()
    p.setColor(QPalette.ColorRole.Window, QColor("#1e1e2e"))
    p.setColor(QPalette.ColorRole.WindowText, QColor("#cdd6f4"))
    p.setColor(QPalette.ColorRole.Base, QColor("#181825"))
    p.setColor(QPalette.ColorRole.AlternateBase, QColor("#11111b"))
    p.setColor(QPalette.ColorRole.ToolTipBase, QColor("#313244"))
    p.setColor(QPalette.ColorRole.ToolTipText, QColor("#cdd6f4"))
    p.setColor(QPalette.ColorRole.Text, QColor("#cdd6f4"))
    p.setColor(QPalette.ColorRole.Button, QColor("#313244"))
    p.setColor(QPalette.ColorRole.ButtonText, QColor("#cdd6f4"))
    p.setColor(QPalette.ColorRole.BrightText, QColor("#f38ba8"))
    p.setColor(QPalette.ColorRole.Link, QColor("#89b4fa"))
    p.setColor(QPalette.ColorRole.Highlight, QColor("#89b4fa"))
    p.setColor(QPalette.ColorRole.HighlightedText, QColor("#11111b"))
    return p


def make_light_palette() -> QPalette:
    """Create a light QPalette."""
    p = QPalette()
    p.setColor(QPalette.ColorRole.Window, QColor("#eff1f5"))
    p.setColor(QPalette.ColorRole.WindowText, QColor("#4c4f69"))
    p.setColor(QPalette.ColorRole.Base, QColor("#e6e9ef"))
    p.setColor(QPalette.ColorRole.AlternateBase, QColor("#dce0e8"))
    p.setColor(QPalette.ColorRole.ToolTipBase, QColor("#ccd0da"))
    p.setColor(QPalette.ColorRole.ToolTipText, QColor("#4c4f69"))
    p.setColor(QPalette.ColorRole.Text, QColor("#4c4f69"))
    p.setColor(QPalette.ColorRole.Button, QColor("#e6e9ef"))
    p.setColor(QPalette.ColorRole.ButtonText, QColor("#4c4f69"))
    p.setColor(QPalette.ColorRole.BrightText, QColor("#d20f39"))
    p.setColor(QPalette.ColorRole.Link, QColor("#1e66f5"))
    p.setColor(QPalette.ColorRole.Highlight, QColor("#1e66f5"))
    p.setColor(QPalette.ColorRole.HighlightedText, QColor("#ffffff"))
    return p


def apply_theme(app, dark: bool = True):
    """Apply the theme palette to the QApplication."""
    if dark:
        app.setPalette(make_dark_palette())
    else:
        app.setPalette(make_light_palette())
