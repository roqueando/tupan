"""ComputedValuesWidget — shows computed component values."""

from PySide6.QtWidgets import QFormLayout, QLabel, QGroupBox

from tupan.ui.schematic_view import format_eng
from tupan.domain.design_params import DesignResults
from tupan.ui.theme import COLORS


class ComputedValuesWidget(QGroupBox):
    """Group box showing computed component values as read-only labels."""

    def __init__(self, parent=None):
        super().__init__("Computed Components", parent)
        layout = QFormLayout(self)
        layout.setSpacing(2)
        layout.setContentsMargins(8, 8, 8, 8)

        self.dil_label = QLabel("--")
        self.dil_label.setWordWrap(True)
        self.dil_label.setStyleSheet("font-weight: bold;")
        layout.addRow("\u0394iL (A):", self.dil_label)

        self.dvo_label = QLabel("--")
        self.dvo_label.setWordWrap(True)
        self.dvo_label.setStyleSheet("font-weight: bold;")
        layout.addRow("\u0394Vo (V):", self.dvo_label)

        self.l_label = QLabel("--")
        self.l_label.setWordWrap(True)
        layout.addRow("L:", self.l_label)

        self.c_label = QLabel("--")
        self.c_label.setWordWrap(True)
        layout.addRow("C:", self.c_label)

        self.r_label = QLabel("--")
        self.r_label.setWordWrap(True)
        layout.addRow("R:", self.r_label)

        self._apply_theme()

    def _apply_theme(self):
        """Update colors based on the light theme."""
        self.l_label.setStyleSheet(
            f"font-weight: bold; color: {COLORS['accent']};"
        )
        self.c_label.setStyleSheet(
            f"font-weight: bold; color: {COLORS['success']};"
        )
        self.r_label.setStyleSheet(
            f"font-weight: bold; color: {COLORS['warning']};"
        )

    def update_values(self, computed: DesignResults):
        self.dil_label.setText(format_eng(computed.delta_il_amps, "A"))
        self.dvo_label.setText(format_eng(computed.delta_vo_volts, "V"))
        self.l_label.setText(format_eng(computed.inductance, "H"))
        self.c_label.setText(format_eng(computed.capacitance, "F"))
        self.r_label.setText(format_eng(computed.load_resistance, "\u03A9"))
