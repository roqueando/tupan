"""ComputedValuesWidget — shows computed component values.

Uses #000 (black) labels in light mode for readability.
"""

from PySide6.QtWidgets import QFormLayout, QLabel, QGroupBox

from tupan.ui.schematic_view import format_eng
from tupan.domain.design_params import DesignResults
from tupan.ui.theme import get_colors


class ComputedValuesWidget(QGroupBox):
    """Group box showing computed component values as read-only labels."""

    def __init__(self, parent=None):
        super().__init__("Computed Components", parent)
        self._dark = True
        layout = QFormLayout(self)
        layout.setSpacing(2)
        layout.setContentsMargins(8, 8, 8, 8)

        self.dil_label = QLabel("--")
        self.dil_label.setStyleSheet("font-weight: bold;")
        layout.addRow("\u0394iL (A):", self.dil_label)

        self.dvo_label = QLabel("--")
        self.dvo_label.setStyleSheet("font-weight: bold;")
        layout.addRow("\u0394Vo (V):", self.dvo_label)

        self.l_label = QLabel("--")
        layout.addRow("L:", self.l_label)

        self.c_label = QLabel("--")
        layout.addRow("C:", self.c_label)

        self.r_label = QLabel("--")
        layout.addRow("R:", self.r_label)

    def _apply_theme(self):
        """Update colors based on current theme."""
        if self._dark:
            colors = get_colors(True)
            self.l_label.setStyleSheet(
                f"font-weight: bold; color: {colors['accent']};"
            )
            self.c_label.setStyleSheet(
                f"font-weight: bold; color: {colors['success']};"
            )
            self.r_label.setStyleSheet(
                f"font-weight: bold; color: {colors['warning']};"
            )
        else:
            # Light mode: pure black for readability
            self.l_label.setStyleSheet("font-weight: bold; color: #000;")
            self.c_label.setStyleSheet("font-weight: bold; color: #000;")
            self.r_label.setStyleSheet("font-weight: bold; color: #000;")

    def set_dark(self, dark: bool):
        self._dark = dark
        self._apply_theme()

    def update_values(self, computed: DesignResults):
        self.dil_label.setText(format_eng(computed.delta_il_amps, "A"))
        self.dvo_label.setText(format_eng(computed.delta_vo_volts, "V"))
        self.l_label.setText(format_eng(computed.inductance, "H"))
        self.c_label.setText(format_eng(computed.capacitance, "F"))
        self.r_label.setText(format_eng(computed.load_resistance, "\u03A9"))
        self._apply_theme()
