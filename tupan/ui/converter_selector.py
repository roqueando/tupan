"""Converter type selector.

Currently focused on Buck converter. Boost and VSI are shown but
deferred for future implementation.
"""

from PySide6.QtCore import Signal
from PySide6.QtWidgets import QWidget, QHBoxLayout, QPushButton, QLabel

from tupan.app.state import AppState
from tupan.domain import ConverterType


class ConverterSelector(QWidget):
    """Button group for selecting converter type."""

    converter_changed = Signal(ConverterType)

    def __init__(self, state: AppState, parent=None):
        super().__init__(parent)
        self.state = state
        self.buttons = {}
        self._setup_ui()

    def _setup_ui(self):
        layout = QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(4)

        layout.addWidget(QLabel("Converter:"))

        # Buck — fully supported
        self._add_button(ConverterType.Buck, "Buck", layout)

        # Boost — shown but disabled (deferred)
        boost_btn = self._add_button(ConverterType.Boost, "Boost", layout)
        boost_btn.setEnabled(False)
        boost_btn.setToolTip("Boost converter coming soon")

        # VSI — shown but disabled (deferred)
        vsi_btn = self._add_button(ConverterType.VsiSinglePhase, "VSI", layout)
        vsi_btn.setEnabled(False)
        vsi_btn.setToolTip("VSI coming soon")

        layout.addStretch()

    def _add_button(self, conv_type, label, layout):
        btn = QPushButton(label)
        btn.setCheckable(True)
        btn.setChecked(conv_type == self.state.active_converter)
        if conv_type == ConverterType.Buck:
            btn.clicked.connect(lambda checked, ct=conv_type: self._select(ct))
        self.buttons[conv_type] = btn
        layout.addWidget(btn)
        return btn

    def _select(self, conv_type: ConverterType):
        """Handle converter selection."""
        if conv_type == self.state.active_converter:
            return
        for ct, btn in self.buttons.items():
            btn.setChecked(ct == conv_type)
        self.state.active_converter = conv_type
        self.state.reset_params()
        self.converter_changed.emit(conv_type)

    def update_state(self, state: AppState):
        """Refresh UI to match state."""
        self.state = state
        for ct, btn in self.buttons.items():
            btn.setChecked(ct == state.active_converter)
