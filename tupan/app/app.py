"""Main application window."""

from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QMainWindow, QWidget, QLabel,
    QFileDialog, QMessageBox, QToolBar, QSizePolicy,
)
from PySide6.QtGui import QAction

from tupan.app.state import AppState
from tupan.app.persistence import save_project, load_project
from tupan.ui.workspace import WorkspaceWidget
from tupan.schematic.schemdraw_layout import draw_converter, ComponentLabels


class TupanApp(QMainWindow):

    def __init__(self):
        super().__init__()
        self.state = AppState()
        self.state.recalculate()

        self.setWindowTitle("tupan")
        self.setMinimumSize(1200, 760)
        self.resize(1200, 800)

        self.workspace = WorkspaceWidget(self.state)
        self.setCentralWidget(self.workspace)

        self._setup_toolbar()

    def _setup_toolbar(self):
        tb = QToolBar("Main Toolbar")
        tb.setMovable(False)
        self.addToolBar(tb)

        # title = QLabel("Tupan")
        # title.setStyleSheet("font-size: 16px; font-weight: bold; padding: 0 8px;")
        # tb.addWidget(title)

        # tb.addSeparator()

        save_act = QAction("Save", self)
        save_act.setToolTip("Save project to JSON")
        save_act.triggered.connect(self._save_project)
        tb.addAction(save_act)

        load_act = QAction("Open", self)
        load_act.setToolTip("Load project from JSON")
        load_act.triggered.connect(self._load_project)
        tb.addAction(load_act)

        export_act = QAction("Export SVG", self)
        export_act.setToolTip("Export schematic as SVG")
        export_act.triggered.connect(self._export_svg)
        tb.addAction(export_act)

        tb.addSeparator()

        self.status_label = QLabel(self.state.status_message)
        self.status_label.setStyleSheet("padding: 0 8px;")
        tb.addWidget(self.status_label)

        spacer = QWidget()
        spacer.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Preferred)
        tb.addWidget(spacer)

    def _save_project(self):
        path, _ = QFileDialog.getSaveFileName(
            self, "Save Project", "project.tupan.json",
            "Tupan Project (*.json);;All Files (*)"
        )
        if path:
            try:
                save_project(path, self.state)
                self.state.status_message = f"Saved to {path}"
                self.status_label.setText(self.state.status_message)
            except Exception as e:
                QMessageBox.warning(self, "Save Error", str(e))

    def _load_project(self):
        path, _ = QFileDialog.getOpenFileName(
            self, "Load Project", "",
            "JSON (*.json);;All Files (*)"
        )
        if path:
            try:
                self.state = load_project(path)
                self.state.status_message = f"Loaded from {path}"
                self.status_label.setText(self.state.status_message)
                self.workspace.update_state(self.state)
            except Exception as e:
                QMessageBox.warning(self, "Load Error", str(e))

    def _export_svg(self):
        from tupan.ui.schematic_view import format_eng

        path, _ = QFileDialog.getSaveFileName(
            self, "Export SVG", "schematic.svg", "SVG (*.svg)"
        )
        if path:
            try:
                labels = ComponentLabels(
                    vin=format_eng(self.state.design.vin, "V"),
                    vout=format_eng(self.state.results.vout, "V"),
                    inductance=format_eng(self.state.computed.inductance, "H"),
                    capacitance=format_eng(self.state.computed.capacitance, "F"),
                    load=format_eng(self.state.computed.load_resistance, "Ohm"),
                    frequency=format_eng(self.state.design.frequency, "Hz"),
                    duty_cycle=f"{self.state.design.duty_cycle * 100:.1f}%",
                )
                from tupan.domain import ConverterType
                png_data = draw_converter(ConverterType.Buck, labels)
                with open(path, 'wb') as f:
                    f.write(png_data)
                self.state.status_message = f"SVG exported to {path}"
                self.status_label.setText(self.state.status_message)
            except Exception as e:
                QMessageBox.warning(self, "Export Error", str(e))
