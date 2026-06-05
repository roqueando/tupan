"""Parameter panel — buck converter design specification inputs.

Layout:
  ⚡ Input Conditions
    Vin, Vout, Duty Cycle (sliders + spinboxes)
    Frequency (SliderSpinBox with log slider)
  📐 Design Targets
    Iout,max, ΔiL%, ΔVo% (sliders + spinboxes)
  🔧 Computed Components
    ΔiL(A), ΔVo(V), L, C, R (read-only labels)
  🧮 Numerical sim toggle
"""

from PySide6.QtCore import Qt, Signal
from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QFormLayout,
    QDoubleSpinBox, QCheckBox, QLabel, QGroupBox,
)

from tupan.app.state import AppState
from tupan.domain import ConverterType
from tupan.ui.slider_spinbox import SliderSpinBox
from tupan.ui.schematic_view import format_eng


class DoubleSliderRow(QWidget):
    """A labeled row with a QDoubleSpinBox + linear QSlider, two-way bound."""
    value_changed = Signal(float)

    def __init__(self, label: str, lo: float, hi: float,
                 step: float, suffix: str = "", initial: float = 0,
                 decimals: int = 2, parent=None):
        super().__init__(parent)
        self._is_updating = False

        layout = QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(4)

        lbl = QLabel(label)
        lbl.setFixedWidth(60)
        layout.addWidget(lbl)

        slider = QWidget()  # placeholder for proportional space
        self.slider = type('Slider', (), {})()  # dummy

        # For numeric precision, use a horizontal layout with spinbox
        self.spinbox = QDoubleSpinBox()
        self.spinbox.setRange(lo, hi)
        self.spinbox.setSingleStep(step)
        self.spinbox.setDecimals(decimals)
        self.spinbox.setKeyboardTracking(False)
        if suffix:
            self.spinbox.setSuffix(f" {suffix}")
        self.spinbox.valueChanged.connect(self._on_spinbox)
        layout.addWidget(self.spinbox)

    def _on_spinbox(self, value):
        self.value_changed.emit(value)

    def set_value(self, value):
        self.spinbox.blockSignals(True)
        self.spinbox.setValue(value)
        self.spinbox.blockSignals(False)

    def value(self):
        return self.spinbox.value()

    def blockSignals(self, block):
        super().blockSignals(block)
        self.spinbox.blockSignals(block)


class ParamPanel(QWidget):
    """Design parameter panel for the buck converter."""

    params_changed = Signal()

    def __init__(self, state, parent=None):
        super().__init__(parent)
        self.state = state
        self._setup_ui()

    def _setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(4, 4, 4, 4)
        layout.setSpacing(6)

        # ── Input Conditions ──
        input_group = QGroupBox("Input Conditions")
        input_layout = QFormLayout(input_group)
        input_layout.setSpacing(4)

        self.vin_row = DoubleSliderRow("Vin:", 1.0, 500.0, 1.0, "V", 48.0)
        self.vin_row.value_changed.connect(self._on_vin)
        input_layout.addRow("", self.vin_row)

        self.vout_row = DoubleSliderRow("Vout:", 0.5, 500.0, 0.5, "V", 12.0)
        self.vout_row.value_changed.connect(self._on_vout)
        input_layout.addRow("", self.vout_row)

        self.duty_row = DoubleSliderRow("Duty:", 1.0, 99.0, 0.5, "%", 25.0, decimals=1)
        self.duty_row.value_changed.connect(self._on_duty)
        input_layout.addRow("", self.duty_row)

        layout.addWidget(input_group)

        # ── Switching Frequency (with log slider) ──
        freq_group = QGroupBox("Switching")
        freq_layout = QVBoxLayout(freq_group)
        self.freq_slider = SliderSpinBox(
            min_val=100.0, max_val=1_000_000.0,
            initial=100_000.0, suffix="Hz", decimals=1
        )
        self.freq_slider.value_changed.connect(self._on_freq)
        freq_layout.addWidget(self.freq_slider)
        layout.addWidget(freq_group)

        # ── Design Targets ──
        target_group = QGroupBox("Design Targets")
        target_layout = QFormLayout(target_group)
        target_layout.setSpacing(4)

        self.iout_row = DoubleSliderRow("Iout,max:", 0.01, 100.0, 0.1, "A", 5.0, decimals=3)
        self.iout_row.value_changed.connect(self._on_iout)
        target_layout.addRow("", self.iout_row)

        self.dil_row = DoubleSliderRow("\u0394iL:", 1.0, 100.0, 0.5, "%", 30.0, decimals=1)
        self.dil_row.value_changed.connect(self._on_dil)
        target_layout.addRow("", self.dil_row)

        self.dvo_row = DoubleSliderRow("\u0394Vo:", 0.01, 50.0, 0.1, "%", 1.0, decimals=2)
        self.dvo_row.value_changed.connect(self._on_dvo)
        target_layout.addRow("", self.dvo_row)

        layout.addWidget(target_group)

        # ── Computed Components ──
        comp_group = QGroupBox("Computed Components")
        comp_layout = QFormLayout(comp_group)
        comp_layout.setSpacing(2)

        self.dil_a_label = QLabel("--")
        comp_layout.addRow("\u0394iL (A):", self.dil_a_label)

        self.dvo_v_label = QLabel("--")
        comp_layout.addRow("\u0394Vo (V):", self.dvo_v_label)

        self.l_label = QLabel("--")
        comp_layout.addRow("L:", self.l_label)

        self.c_label = QLabel("--")
        comp_layout.addRow("C:", self.c_label)

        self.r_label = QLabel("--")
        comp_layout.addRow("R:", self.r_label)

        layout.addWidget(comp_group)

        # ── Numerical simulation toggle ──
        self.sim_cb = QCheckBox("Numerical simulation")
        self.sim_cb.setToolTip("Enable RK4 time-domain simulation overlay on plots")
        self.sim_cb.stateChanged.connect(self._on_sim)
        layout.addWidget(self.sim_cb)

        layout.addStretch()

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
        c = self.state.computed

        self.vin_row.blockSignals(True)
        self.vin_row.set_value(d.vin)
        self.vin_row.blockSignals(False)

        self.vout_row.blockSignals(True)
        self.vout_row.set_value(d.vout)
        self.vout_row.blockSignals(False)

        self.duty_row.blockSignals(True)
        self.duty_row.set_value(d.duty_cycle * 100.0)
        self.duty_row.blockSignals(False)

        self.freq_slider.blockSignals(True)
        self.freq_slider.set_value(d.frequency)
        self.freq_slider.blockSignals(False)

        self.iout_row.blockSignals(True)
        self.iout_row.set_value(d.iout_max)
        self.iout_row.blockSignals(False)

        self.dil_row.blockSignals(True)
        self.dil_row.set_value(d.delta_il_pct * 100.0)
        self.dil_row.blockSignals(False)

        self.dvo_row.blockSignals(True)
        self.dvo_row.set_value(d.delta_vo_pct * 100.0)
        self.dvo_row.blockSignals(False)

        # Computed values (read-only)
        self.dil_a_label.setText(format_eng(c.delta_il_amps, "A"))
        self.dvo_v_label.setText(format_eng(c.delta_vo_volts, "V"))
        self.l_label.setText(format_eng(c.inductance, "H"))
        self.c_label.setText(format_eng(c.capacitance, "F"))
        self.r_label.setText(format_eng(c.load_resistance, "\u03A9"))

        self.sim_cb.blockSignals(True)
        self.sim_cb.setChecked(self.state.show_numerical_sim)
        self.sim_cb.blockSignals(False)

    def update_state(self, state):
        """Update panel to reflect a new state (e.g. after load)."""
        self.state = state
        self._sync_from_state()
