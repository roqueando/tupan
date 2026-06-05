"""Auto-generated converter schematic widget using schemdraw.

Renders the converter circuit diagram via schemdraw and displays it
as a QPixmap in a QLabel.
"""

from PySide6.QtCore import Qt
from PySide6.QtGui import QPixmap
from PySide6.QtWidgets import QLabel, QWidget, QVBoxLayout

from tupan.app.state import AppState
from tupan.schematic.schemdraw_layout import draw_converter, ComponentLabels
from tupan.ui.theme import get_colors


def format_eng(value: float, unit: str) -> str:
    """Format a value with appropriate SI prefix."""
    abs_val = abs(value)
    if abs_val == 0.0:
        return f"0 {unit}"
    if abs_val >= 1_000_000.0:
        return f"{value / 1_000_000.0:.2f} M{unit}"
    elif abs_val >= 1_000.0:
        return f"{value / 1_000.0:.2f} k{unit}"
    elif abs_val >= 1.0:
        return f"{value:.3f} {unit}"
    elif abs_val >= 0.001:
        return f"{value * 1_000.0:.3f} m{unit}"
    elif abs_val >= 0.000_001:
        return f"{value * 1_000_000.0:.3f} μ{unit}"
    elif abs_val >= 1e-9:
        return f"{value * 1e9:.3f} n{unit}"
    else:
        return f"{value * 1e12:.3f} p{unit}"


class SchematicView(QWidget):
    """Widget that renders the converter schematic using schemdraw."""

    def __init__(self, state: AppState, parent=None):
        super().__init__(parent)
        self.state = state
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)

        self.image_label = QLabel()
        self.image_label.setAlignment(Qt.AlignmentFlag.AlignCenter)
        layout.addWidget(self.image_label)

        self._render()

    def _render(self):
        """Generate and display the schematic image."""
        labels = ComponentLabels(
            vin=format_eng(self.state.design.vin, "V"),
            vout=format_eng(self.state.results.vout, "V"),
            inductance=format_eng(self.state.computed.inductance, "H"),
            capacitance=format_eng(self.state.computed.capacitance, "F"),
            load=format_eng(self.state.computed.load_resistance, "Ω"),
            frequency=format_eng(self.state.design.frequency, "Hz"),
            duty_cycle=f"{self.state.design.duty_cycle * 100:.1f}%",
        )

        png_data = draw_converter(self.state.active_converter, labels)
        pixmap = QPixmap()
        if pixmap.loadFromData(png_data, "PNG"):
            # Scale to fit width while maintaining aspect ratio
            scaled = pixmap.scaledToWidth(
                max(200, self.width() - 20),
                Qt.TransformationMode.SmoothTransformation
            )
            self.image_label.setPixmap(scaled)
        else:
            self.image_label.setText("Schematic unavailable")

    def resizeEvent(self, event):
        """Re-render at the new size."""
        super().resizeEvent(event)
        self._render()

    def update_state(self, state: AppState):
        """Refresh the view with a new state."""
        self.state = state
        self._render()
