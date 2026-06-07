"""Design tokens and theme management for Tupan.

Light-only theme with #f1efe7 as the primary background color.
Provides a cohesive Palette dict plus QPalette helpers.
"""

from PySide6.QtGui import QPalette, QColor


# ── Light theme color table ──

COLORS = {
    "bg_primary": "#f1efe7",
    "bg_secondary": "#ffffff",
    "bg_surface": "#e9e7df",
    "text_primary": "#1c1c1e",
    "text_secondary": "#5c5c62",
    "text_dim": "#9c9ca2",
    "accent": "#2563eb",
    "accent_dim": "#60a5fa",
    "success": "#16a34a",
    "warning": "#d97706",
    "error": "#dc2626",
    "border": "#dddcd4",
    "card_bg": "#ffffff",
    "card_hover": "#f6f4ee",
    "card_selected": "#eeebe2",
    "canvas_bg": "#f1efe7",
    "sidebar_bg": "#ffffff",
    "top_bar_bg": "#f1efe7",
    "wire": "#1c1c1e",
    "component": "#2563eb",
    "label": "#5c5c62",
    "ground": "#9c9ca2",
}


def get_colors() -> dict:
    """Get the light theme color table."""
    return COLORS


def make_palette() -> QPalette:
    """Create a light QPalette based on the theme colors."""
    p = QPalette()
    p.setColor(QPalette.ColorRole.Window, QColor("#f1efe7"))
    p.setColor(QPalette.ColorRole.WindowText, QColor("#1c1c1e"))
    p.setColor(QPalette.ColorRole.Base, QColor("#ffffff"))
    p.setColor(QPalette.ColorRole.AlternateBase, QColor("#e9e7df"))
    p.setColor(QPalette.ColorRole.ToolTipBase, QColor("#ffffff"))
    p.setColor(QPalette.ColorRole.ToolTipText, QColor("#1c1c1e"))
    p.setColor(QPalette.ColorRole.Text, QColor("#1c1c1e"))
    p.setColor(QPalette.ColorRole.Button, QColor("#ffffff"))
    p.setColor(QPalette.ColorRole.ButtonText, QColor("#1c1c1e"))
    p.setColor(QPalette.ColorRole.BrightText, QColor("#dc2626"))
    p.setColor(QPalette.ColorRole.Link, QColor("#2563eb"))
    p.setColor(QPalette.ColorRole.Highlight, QColor("#2563eb"))
    p.setColor(QPalette.ColorRole.HighlightedText, QColor("#ffffff"))
    return p


def apply_theme(app):
    """Apply the light theme palette to the QApplication."""
    app.setPalette(make_palette())
