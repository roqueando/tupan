"""KnobSpinBox — a potentiometer-style knob combined with a QDoubleSpinBox.

Provides a rotary knob (QDial) for analog-style adjustment plus a numeric
spinbox for precise entry. The knob and spinbox are two-way bound.

Supports both linear and logarithmic scales for the knob.
"""

from PySide6.QtCore import Qt, Signal
from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout,
    QDial, QDoubleSpinBox, QLabel,
)
import math


class KnobSpinBox(QWidget):
    """Potentiometer-style knob with integrated numeric spinbox.

    Args:
        label: Short label text (e.g. "Vin", "Vout")
        lo: Minimum value
        hi: Maximum value
        step: Step increment for spinbox
        initial: Initial value
        suffix: Unit suffix (e.g. "V", "A", "%")
        decimals: Number of decimal places in the spinbox
        logarithmic: If True, knob follows log scale (good for frequency)
        knob_steps: Number of steps in the knob dial (resolution)
    """

    value_changed = Signal(float)

    def __init__(self, label: str = "", lo: float = 0.0, hi: float = 100.0,
                 step: float = 1.0, initial: float = 0.0,
                 suffix: str = "", decimals: int = 2,
                 logarithmic: bool = False, knob_steps: int = 100,
                 parent=None):
        super().__init__(parent)
        self._lo = lo
        self._hi = hi
        self._step = step
        self._logarithmic = logarithmic
        self._knob_steps = knob_steps
        self._log_lo = math.log10(lo) if logarithmic and lo > 0 else 0
        self._log_hi = math.log10(hi) if logarithmic and hi > 0 else 0
        self._is_updating = False

        layout = QVBoxLayout(self)
        layout.setContentsMargins(2, 2, 2, 2)
        layout.setSpacing(2)

        # Label on top
        self.label = QLabel(label)
        self.label.setAlignment(Qt.AlignmentFlag.AlignCenter)
        font = self.label.font()
        font.setPointSize(9)
        font.setBold(True)
        self.label.setFont(font)
        layout.addWidget(self.label)

        # Knob center
        knob_layout = QHBoxLayout()
        knob_layout.addStretch()

        self.knob = QDial()
        self.knob.setRange(0, knob_steps)
        self.knob.setNotchesVisible(True)
        self.knob.setFixedSize(64, 64)
        self.knob.valueChanged.connect(self._on_knob)
        knob_layout.addWidget(self.knob)

        knob_layout.addStretch()
        layout.addLayout(knob_layout)

        # Spinbox below knob
        self.spinbox = QDoubleSpinBox()
        self.spinbox.setRange(lo, hi)
        self.spinbox.setSingleStep(step)
        self.spinbox.setDecimals(decimals)
        self.spinbox.setKeyboardTracking(False)
        if suffix:
            self.spinbox.setSuffix(f" {suffix}")
        self.spinbox.valueChanged.connect(self._on_spinbox)

        spin_layout = QHBoxLayout()
        spin_layout.addStretch()
        spin_layout.addWidget(self.spinbox)
        spin_layout.addStretch()
        layout.addLayout(spin_layout)

        # Set initial value
        self.set_value(initial)

    def _value_to_knob(self, value: float) -> int:
        """Convert a value to knob position (0..knob_steps)."""
        if self._logarithmic and value > 0:
            log_v = math.log10(value)
            fraction = (log_v - self._log_lo) / (self._log_hi - self._log_lo) \
                if self._log_hi > self._log_lo else 0
        else:
            fraction = (value - self._lo) / (self._hi - self._lo) \
                if self._hi > self._lo else 0
        return max(0, min(self._knob_steps, int(round(fraction * self._knob_steps))))

    def _knob_to_value(self, knob_pos: int) -> float:
        """Convert knob position to a value."""
        fraction = knob_pos / self._knob_steps if self._knob_steps > 0 else 0
        if self._logarithmic:
            log_v = self._log_lo + fraction * (self._log_hi - self._log_lo)
            return 10.0 ** log_v
        else:
            return self._lo + fraction * (self._hi - self._lo)

    def _on_knob(self, pos: int):
        """Knob turned → update spinbox."""
        if self._is_updating:
            return
        self._is_updating = True
        value = self._knob_to_value(pos)
        if self._step > 0 and not self._logarithmic:
            value = round(value / self._step) * self._step
            value = max(self._lo, min(self._hi, value))
        self.spinbox.setValue(value)
        self._is_updating = False
        self.value_changed.emit(value)

    def _on_spinbox(self, value: float):
        """Spinbox changed → update knob."""
        if self._is_updating:
            return
        self._is_updating = True
        self.knob.setValue(self._value_to_knob(value))
        self._is_updating = False
        self.value_changed.emit(value)

    def set_value(self, value: float):
        """Set value without triggering signal loops."""
        self._is_updating = True
        value = max(self._lo, min(self._hi, value))
        self.spinbox.setValue(value)
        self.knob.setValue(self._value_to_knob(value))
        self._is_updating = False

    def value(self) -> float:
        return self.spinbox.value()

    def blockSignals(self, block: bool):
        super().blockSignals(block)
        self.knob.blockSignals(block)
        self.spinbox.blockSignals(block)
