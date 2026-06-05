"""Main workspace layout with 3-panel QSplitter.

Left: Design params + computed components
Center: Schematic + Waveform plots
Right: Results (operating point + performance)
"""

from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QSplitter, QScrollArea,
    QGroupBox,
)

from tupan.app.state import AppState
from tupan.ui.converter_selector import ConverterSelector
from tupan.ui.param_panel import ParamPanel
from tupan.ui.result_panel import ResultPanel
from tupan.ui.schematic_view import SchematicView
from tupan.ui.plot_panel import PlotPanel


class WorkspaceWidget(QWidget):

    def __init__(self, state, parent=None):
        super().__init__(parent)
        self.state = state
        self._setup_ui()

    def _setup_ui(self):
        layout = QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)

        splitter = QSplitter(Qt.Orientation.Horizontal)

        # ── Left panel: Parameters ──
        left = QWidget()
        left_layout = QVBoxLayout(left)
        left_layout.setContentsMargins(8, 8, 8, 8)

        self.converter_selector = ConverterSelector(self.state)
        self.converter_selector.converter_changed.connect(
            self._on_converter_changed
        )
        left_layout.addWidget(self.converter_selector)

        left_layout.addSpacing(8)

        self.param_panel = ParamPanel(self.state)
        self.param_panel.params_changed.connect(self._on_params_changed)
        left_layout.addWidget(self.param_panel)

        left_layout.addStretch()
        splitter.addWidget(left)

        # ── Center panel: Schematic + Plots ──
        center = QWidget()
        center_layout = QVBoxLayout(center)
        center_layout.setContentsMargins(8, 8, 8, 8)

        sg = QGroupBox("Schematic")
        sgl = QVBoxLayout(sg)
        self.schematic_view = SchematicView(self.state)
        sgl.addWidget(self.schematic_view)
        center_layout.addWidget(sg)

        pg = QGroupBox("Waveforms")
        pgl = QVBoxLayout(pg)
        self.plot_panel = PlotPanel(self.state)
        pgl.addWidget(self.plot_panel)
        center_layout.addWidget(pg)

        splitter.addWidget(center)

        # ── Right panel: Results ──
        right_scroll = QScrollArea()
        right_scroll.setWidgetResizable(True)
        right_scroll.setHorizontalScrollBarPolicy(
            Qt.ScrollBarPolicy.ScrollBarAlwaysOff
        )
        self.result_panel = ResultPanel(self.state)
        right_scroll.setWidget(self.result_panel)
        splitter.addWidget(right_scroll)

        splitter.setSizes([320, 480, 220])
        splitter.setStretchFactor(0, 0)
        splitter.setStretchFactor(1, 1)
        splitter.setStretchFactor(2, 0)

        layout.addWidget(splitter)

    def _on_converter_changed(self, ct):
        self.param_panel.update_state(self.state)
        self.result_panel.update_state(self.state)
        self.schematic_view.update_state(self.state)
        self.plot_panel.update_state(self.state)

    def _on_params_changed(self):
        """Design parameter changed → recalculate already happened in param_panel."""
        self.result_panel.update_state(self.state)
        self.schematic_view.update_state(self.state)
        self.plot_panel.update_state(self.state)

    def refresh_plot_theme(self):
        self.plot_panel.refresh_theme()

    def update_state(self, state):
        self.state = state
        self.converter_selector.update_state(state)
        self.param_panel.update_state(state)
        self.result_panel.update_state(state)
        self.schematic_view.update_state(state)
        self.plot_panel.update_state(state)
