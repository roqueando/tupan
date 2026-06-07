"""Results panel — operating point and performance metrics."""

from PySide6.QtCore import Qt
from PySide6.QtWidgets import QWidget, QVBoxLayout, QFormLayout, QLabel, QGroupBox

from tupan.app.state import AppState
from tupan.ui.schematic_view import format_eng
from tupan.ui.theme import COLORS


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
        self.duty_label.setWordWrap(True)
        layout.addWidget(self.duty_label)

        vg = QGroupBox("Voltage / Current")
        vl = QFormLayout(vg)
        self.vout_label = QLabel("--")
        self.vout_label.setWordWrap(True)
        vl.addRow("Vout:", self.vout_label)
        self.iout_label = QLabel("--")
        self.iout_label.setWordWrap(True)
        vl.addRow("Iout:", self.iout_label)
        self.iin_label = QLabel("--")
        self.iin_label.setWordWrap(True)
        vl.addRow("Iin:", self.iin_label)
        layout.addWidget(vg)

        rg = QGroupBox("Ripple")
        rl = QFormLayout(rg)
        self.vripple_label = QLabel("--")
        self.vripple_label.setWordWrap(True)
        rl.addRow("V ripple (pp):", self.vripple_label)
        self.ilripple_label = QLabel("--")
        self.ilripple_label.setWordWrap(True)
        rl.addRow("I_L ripple (pp):", self.ilripple_label)
        layout.addWidget(rg)

        lg = QGroupBox("Losses & Efficiency")
        ll = QFormLayout(lg)
        self.cond_loss_label = QLabel("--")
        self.cond_loss_label.setWordWrap(True)
        ll.addRow("Conduction loss:", self.cond_loss_label)
        self.sw_loss_label = QLabel("--")
        self.sw_loss_label.setWordWrap(True)
        ll.addRow("Switching loss:", self.sw_loss_label)
        self.total_loss_label = QLabel("--")
        self.total_loss_label.setWordWrap(True)
        ll.addRow("Total loss:", self.total_loss_label)
        self.eff_label = QLabel("--")
        self.eff_label.setWordWrap(True)
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

        self.vout_label.setText(format_eng(res.vout, "V"))
        self.iout_label.setText(format_eng(res.iout, "A"))
        self.iin_label.setText(format_eng(res.iin, "A"))
        self.vripple_label.setText(format_eng(res.vout_ripple, "V"))
        self.ilripple_label.setText(format_eng(res.il_ripple, "A"))
        self.cond_loss_label.setText(format_eng(res.conduction_losses, "W"))
        self.sw_loss_label.setText(format_eng(res.switching_losses, "W"))
        total_loss = res.conduction_losses + res.switching_losses
        self.total_loss_label.setText(format_eng(total_loss, "W"))

        # Efficiency color — use theme colors
        eff = res.efficiency
        eff_pct = f"{eff * 100:.1f}%"
        if eff > 0.95:
            color = COLORS["success"]
        elif eff > 0.85:
            color = COLORS["warning"]
        else:
            color = COLORS["error"]

        self.eff_label.setText(
            f'<span style="color:{color};font-weight:bold;font-size:14px;">{eff_pct}</span>'
        )
        self.eff_label.setTextFormat(Qt.TextFormat.RichText)
