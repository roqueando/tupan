"""Parameter panel — buck converter design specification inputs.

All parameters use KnobSpinBox (potentiometer-style knob + spinbox).
"""

from PySide6.QtCore import Signal
from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QGridLayout,
    QCheckBox, QGroupBox,
)

from tupan.app.state import AppState
from tupan.ui.knob_spinbox import KnobSpinBox
from tupan.ui.schematic_view import format_eng
from tupan.ui.computed_values import ComputedValuesWidget


class ParamPanel(QWidget):
    """Design parameter panel for the buck converter, using rotary knobs."""

    params_changed = Signal()

    def __init__(self, state, parent=None):
        super().__init__(parent)
        self.state = state
        self._setup_ui()

    def _setup_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(4, 4, 4, 4)
        main_layout.setSpacing(8)

        # ── Input Conditions (row of knobs) ──
        input_group = QGroupBox("Input Conditions")
        input_grid = QGridLayout(input_group)
        input_grid.setSpacing(4)
        input_grid.setContentsMargins(8, 8, 8, 8)

        self.vin_knob = KnobSpinBox("Vin", 1.0, 500.0, 1.0, 48.0, "V", decimals=1)
        self.vin_knob.value_changed.connect(self._on_vin)
        input_grid.addWidget(self.vin_knob, 0, 0)

        self.vout_knob = KnobSpinBox("Vout", 0.5, 500.0, 0.5, 12.0, "V", decimals=1)
        self.vout_knob.value_changed.connect(self._on_vout)
        input_grid.addWidget(self.vout_knob, 0, 1)

        self.duty_knob = KnobSpinBox("Duty", 1.0, 99.0, 0.5, 25.0, "%", decimals=1)
        self.duty_knob.value_changed.connect(self._on_duty)
        input_grid.addWidget(self.duty_knob, 0, 2)

        main_layout.addWidget(input_group)

        # ── Switching Frequency (single knob, full width) ──
        freq_group = QGroupBox("Switching")
        freq_layout = QHBoxLayout(freq_group)
        freq_layout.setContentsMargins(8, 8, 8, 8)

        self.freq_knob = KnobSpinBox("Frequency", 100.0, 1_000_000.0, 1.0,
                                      100_000.0, "Hz", decimals=1,
                                      logarithmic=True)
        self.freq_knob.value_changed.connect(self._on_freq)
        freq_layout.addWidget(self.freq_knob)

        main_layout.addWidget(freq_group)

        # ── Design Targets (row of knobs) ──
        target_group = QGroupBox("Design Targets")
        target_grid = QGridLayout(target_group)
        target_grid.setSpacing(4)
        target_grid.setContentsMargins(8, 8, 8, 8)

        self.iout_knob = KnobSpinBox("Iout,max", 0.01, 100.0, 0.1, 5.0, "A",
                                      decimals=3)
        self.iout_knob.value_changed.connect(self._on_iout)
        target_grid.addWidget(self.iout_knob, 0, 0)

        self.dil_knob = KnobSpinBox("\u0394iL", 1.0, 100.0, 0.5, 30.0, "%", decimals=1)
        self.dil_knob.value_changed.connect(self._on_dil)
        target_grid.addWidget(self.dil_knob, 0, 1)

        self.dvo_knob = KnobSpinBox("\u0394Vo", 0.01, 50.0, 0.1, 1.0, "%", decimals=2)
        self.dvo_knob.value_changed.connect(self._on_dvo)
        target_grid.addWidget(self.dvo_knob, 0, 2)

        main_layout.addWidget(target_group)

        # ── Computed Components (read-only labels) ──
        self.computed_widget = ComputedValuesWidget()
        main_layout.addWidget(self.computed_widget)

        # ── Numerical simulation toggle ──
        self.sim_cb = QCheckBox("Numerical simulation")
        self.sim_cb.setToolTip("Enable RK4 time-domain simulation overlay on plots")
        self.sim_cb.stateChanged.connect(self._on_sim)
        main_layout.addWidget(self.sim_cb)

        main_layout.addStretch()

    def _on_vin(self, value):
        self.state.on_vin_changed(value)
        self._sync_from_state()
        self.params_changed.emit()

    def _on_vout(self, value):
        self.state.on_vout_changed(value)
        self._sync_from_state()
        self.params_changed.emit()

    def _on_duty(self, value):
        self.state.on_duty_changed(value)
        self._sync_from_state()
        self.params_changed.emit()

    def _on_freq(self, value):
        self.state.design.frequency = value
        self.state.recalculate()
        self._sync_from_state()
        self.params_changed.emit()

    def _on_iout(self, value):
        self.state.design.iout_max = value
        self.state.recalculate()
        self._sync_from_state()
        self.params_changed.emit()

    def _on_dil(self, value):
        self.state.design.delta_il_pct = value / 100.0
        self.state.recalculate()
        self._sync_from_state()
        self.params_changed.emit()

    def _on_dvo(self, value):
        self.state.design.delta_vo_pct = value / 100.0
        self.state.recalculate()
        self._sync_from_state()
        self.params_changed.emit()

    def _on_sim(self, state):
        self.state.show_numerical_sim = bool(state)
        self.params_changed.emit()

    def _sync_from_state(self):
        """Update all widget values to reflect current state."""
        d = self.state.design

        self.vin_knob.blockSignals(True)
        self.vin_knob.set_value(d.vin)
        self.vin_knob.blockSignals(False)

        self.vout_knob.blockSignals(True)
        self.vout_knob.set_value(d.vout)
        self.vout_knob.blockSignals(False)

        self.duty_knob.blockSignals(True)
        self.duty_knob.set_value(d.duty_cycle * 100.0)
        self.duty_knob.blockSignals(False)

        self.freq_knob.blockSignals(True)
        self.freq_knob.set_value(d.frequency)
        self.freq_knob.blockSignals(False)

        self.iout_knob.blockSignals(True)
        self.iout_knob.set_value(d.iout_max)
        self.iout_knob.blockSignals(False)

        self.dil_knob.blockSignals(True)
        self.dil_knob.set_value(d.delta_il_pct * 100.0)
        self.dil_knob.blockSignals(False)

        self.dvo_knob.blockSignals(True)
        self.dvo_knob.set_value(d.delta_vo_pct * 100.0)
        self.dvo_knob.blockSignals(False)

        self.computed_widget.update_values(self.state.computed)

        self.sim_cb.blockSignals(True)
        self.sim_cb.setChecked(self.state.show_numerical_sim)
        self.sim_cb.blockSignals(False)

    def update_state(self, state):
        self.state = state
        self._sync_from_state()
