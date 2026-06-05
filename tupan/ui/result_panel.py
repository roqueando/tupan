"""Results panel — operating point and performance metrics.

Shows:
  - Duty cycle
  - Voltage / Current (Vout, Iout, Iin)
  - Ripple (V ripple, IL ripple)
  - Losses & Efficiency (conduction, switching, total, color-coded efficiency)
"""

from PySide6.QtCore import Qt
from PySide6.QtWidgets import QWidget, QVBoxLayout, QFormLayout, QLabel, QGroupBox

from tupan.app.state import AppState
from tupan.domain import ConverterType
from tupan.ui.schematic_view import format_eng


class ResultPanel(QWidget):

    def __init__(self, state, parent=None):
        super().__init__(parent)
        self.state = state
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(8, 8, 8, 8)

        title = QLabel("Results")
        title.setStyleSheet("font-size: 14px; font-weight: bold;")
        layout.addWidget(title)

        self.duty_label = QLabel("")
        layout.addWidget(self.duty_label)

        # Voltage / Current
        vg = QGroupBox("Voltage / Current")
        vl = QFormLayout(vg)
        self.vout_label = QLabel("--")
        vl.addRow("Vout:", self.vout_label)
        self.iout_label = QLabel("--")
        vl.addRow("Iout:", self.iout_label)
        self.iin_label = QLabel("--")
        vl.addRow("Iin:", self.iin_label)
        layout.addWidget(vg)

        # Ripple
        rg = QGroupBox("Ripple")
        rl = QFormLayout(rg)
        self.vripple_label = QLabel("--")
        rl.addRow("V ripple (pp):", self.vripple_label)
        self.ilripple_label = QLabel("--")
        rl.addRow("I_L ripple (pp):", self.ilripple_label)
        layout.addWidget(rg)

        # Losses & Efficiency
        lg = QGroupBox("Losses & Efficiency")
        ll = QFormLayout(lg)
        self.cond_loss_label = QLabel("--")
        ll.addRow("Conduction loss:", self.cond_loss_label)
        self.sw_loss_label = QLabel("--")
        ll.addRow("Switching loss:", self.sw_loss_label)
        self.total_loss_label = QLabel("--")
        ll.addRow("Total loss:", self.total_loss_label)
        self.eff_label = QLabel("--")
        ll.addRow("Efficiency:", self.eff_label)
        layout.addWidget(lg)

        layout.addStretch()

    def update_state(self, state):
        self.state = state
        res = state.results
        comp = state.computed

        self.duty_label.setText(
            f"Duty cycle: {state.design.duty_cycle * 100:.1f}%  "
            f"(Vout={state.design.vout:.2f}V, Vin={state.design.vin:.1f}V)"
        )

        # Voltage / Current
        self.vout_label.setText(format_eng(res.vout, "V"))
        self.iout_label.setText(format_eng(res.iout, "A"))
        self.iin_label.setText(format_eng(res.iin, "A"))

        # Ripple
        self.vripple_label.setText(format_eng(res.vout_ripple, "V"))
        self.ilripple_label.setText(format_eng(res.il_ripple, "A"))

        # Losses
        self.cond_loss_label.setText(format_eng(res.conduction_losses, "W"))
        self.sw_loss_label.setText(format_eng(res.switching_losses, "W"))
        total_loss = res.conduction_losses + res.switching_losses
        self.total_loss_label.setText(format_eng(total_loss, "W"))

        # Efficiency with color coding
        eff = res.efficiency
        eff_pct = f"{eff * 100:.1f}%"
        if eff > 0.95:
            color = "#a6e3a1"
        elif eff > 0.85:
            color = "#f9e2af"
        else:
            color = "#f38ba8"
        self.eff_label.setText(
            f'<span style="color:{color};font-weight:bold;font-size:14px;">{eff_pct}</span>'
        )
        self.eff_label.setTextFormat(Qt.TextFormat.RichText)
