"""SliderSpinBox — reusable QWidget combining a logarithmic QSlider with a QDoubleSpinBox.

The slider and spinbox are two-way bound: changing one updates the other.
The slider uses a logarithmic scale for wide-range parameters like frequency.
"""

from PySide6.QtCore import Qt, Signal
from PySide6.QtWidgets import QWidget, QHBoxLayout, QSlider, QDoubleSpinBox
import math


class SliderSpinBox(QWidget):
    """A widget combining a logarithmic slider with a numeric spinbox.

    The slider provides coarse adjustment, the spinbox provides precise input.
    Two-way binding ensures they stay in sync.

    Args:
        label: Display label text
        min_val: Minimum value (used for both slider and spinbox)
        max_val: Maximum value
        initial: Initial value
        suffix: Unit suffix (e.g. "Hz", "V")
        decimals: Number of decimal places for the spinbox
        parent: Parent widget
    """

    value_changed = Signal(float)

    def __init__(self, label: str = "", min_val: float = 1.0,
                 max_val: float = 1_000_000.0, initial: float = 100_000.0,
                 suffix: str = "", decimals: int = 1, parent=None):
        super().__init__(parent)
        self._min_val = min_val
        self._max_val = max_val
        self._log_min = math.log10(min_val) if min_val > 0 else 0
        self._log_max = math.log10(max_val) if max_val > 0 else 6
        self._is_updating = False

        layout = QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(4)

        # Slider (logarithmic)
        self.slider = QSlider(Qt.Orientation.Horizontal)
        self.slider.setRange(0, 1000)
        self.slider.valueChanged.connect(self._on_slider_changed)
        layout.addWidget(self.slider, 1)

        # Spinbox
        self.spinbox = QDoubleSpinBox()
        self.spinbox.setRange(min_val, max_val)
        self.spinbox.setDecimals(decimals)
        self.spinbox.setKeyboardTracking(False)
        if suffix:
            self.spinbox.setSuffix(f" {suffix}")
        self.spinbox.valueChanged.connect(self._on_spinbox_changed)
        layout.addWidget(self.spinbox)

        # Set initial value
        self._set_value_internal(initial)

    def _log_to_linear(self, log_pos: float) -> float:
        """Convert slider position (0-1000) to log-scale value."""
        if self._log_max == self._log_min:
            return self._min_val
        fraction = log_pos / 1000.0
        log_val = self._log_min + fraction * (self._log_max - self._log_min)
        return 10.0 ** log_val

    def _linear_to_log(self, value: float) -> int:
        """Convert value to slider position (0-1000) on log scale."""
        if value <= 0:
            return 0
        log_val = math.log10(value)
        if self._log_max == self._log_min:
            return 0
        fraction = (log_val - self._log_min) / (self._log_max - self._log_min)
        return max(0, min(1000, int(round(fraction * 1000))))

    def _on_slider_changed(self, pos: int):
        """Slider moved → update spinbox."""
        if self._is_updating:
            return
        self._is_updating = True
        value = self._log_to_linear(float(pos))
        self.spinbox.setValue(value)
        self._is_updating = False
        self.value_changed.emit(value)

    def _on_spinbox_changed(self, value: float):
        """Spinbox changed → update slider."""
        if self._is_updating:
            return
        self._is_updating = True
        self.slider.setValue(self._linear_to_log(value))
        self._is_updating = False
        self.value_changed.emit(value)

    def _set_value_internal(self, value: float):
        """Set value without emitting signals."""
        self._is_updating = True
        value = max(self._min_val, min(self._max_val, value))
        self.spinbox.setValue(value)
        self.slider.setValue(self._linear_to_log(value))
        self._is_updating = False

    def set_value(self, value: float):
        """Set value and emit signal."""
        self._set_value_internal(value)
        self.value_changed.emit(value)

    def value(self) -> float:
        """Get current value."""
        return self.spinbox.value()

    def blockSignals(self, block: bool):
        """Block/unblock signals on both sub-widgets."""
        super().blockSignals(block)
        self.slider.blockSignals(block)
        self.spinbox.blockSignals(block)
