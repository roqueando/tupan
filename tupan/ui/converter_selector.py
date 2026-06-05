"""Converter type selector using the Strategy registry.
"""

from PySide6.QtCore import Signal
from PySide6.QtWidgets import QWidget, QHBoxLayout, QPushButton, QLabel

from tupan.app.state import AppState
from tupan.domain.converters import get_all_strategies, ConverterStrategy


class ConverterSelector(QWidget):
    """Button group for selecting converter type."""

    converter_changed = Signal(object)  # ConverterStrategy

    def __init__(self, state: AppState, parent=None):
        super().__init__(parent)
        self.state = state
        self.buttons: dict[str, QPushButton] = {}
        self._setup_ui()

    def _setup_ui(self):
        layout = QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(4)

        layout.addWidget(QLabel("Converter:"))

        strategies = get_all_strategies()
        for s in strategies:
            btn = QPushButton(s.label())
            btn.setCheckable(True)
            btn.setChecked(s.label() == self.state.strategy.label())
            btn.clicked.connect(lambda checked, st=s: self._select(st))
            self.buttons[s.label()] = btn
            layout.addWidget(btn)

        layout.addStretch()

    def _select(self, strategy: ConverterStrategy):
        """Handle converter selection."""
        if strategy.label() == self.state.strategy.label():
            return
        for lbl, btn in self.buttons.items():
            btn.setChecked(lbl == strategy.label())
        self.state.strategy = strategy
        self.state.reset_params()
        self.converter_changed.emit(strategy)

    def update_state(self, state: AppState):
        """Refresh UI to match state."""
        self.state = state
        for lbl, btn in self.buttons.items():
            btn.setChecked(lbl == state.strategy.label())
