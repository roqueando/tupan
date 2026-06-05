"""Plot panel widget — Matplotlib FigureCanvas embedded in PySide6.

Shows analytical waveforms + optional simulation overlay.
Mirrors src/ui/plot_panel.rs.
"""

import numpy as np
import matplotlib
matplotlib.use("QtAgg")

from matplotlib.backends.backend_qtagg import FigureCanvasQTAgg
from matplotlib.figure import Figure
from PySide6.QtWidgets import QVBoxLayout, QWidget

from tupan.app.state import AppState
from tupan.domain import ConverterType
from tupan.ui.theme import get_colors


class PlotPanel(QWidget):
    """Widget with interactive Matplotlib plots for voltage and current waveforms."""

    def __init__(self, state: AppState, parent=None):
        super().__init__(parent)
        self.state = state

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)

        # Create figure with two subplots
        self.figure = Figure(figsize=(8, 4), dpi=100)
        self.canvas = FigureCanvasQTAgg(self.figure)

        self.ax_vout = self.figure.add_subplot(211)
        self.ax_il = self.figure.add_subplot(212)

        layout.addWidget(self.canvas)

        self._setup_style()

    def _setup_style(self):
        """Apply matplotlib styling based on theme."""
        is_dark = self.state.theme.value == "Dark"
        if is_dark:
            plt_style = {
                "figure.facecolor": "#11111b",
                "axes.facecolor": "#1e1e2e",
                "axes.edgecolor": "#6c7086",
                "axes.labelcolor": "#cdd6f4",
                "text.color": "#cdd6f4",
                "xtick.color": "#a6adc8",
                "ytick.color": "#a6adc8",
                "grid.color": "#313244",
                "grid.alpha": 0.5,
            }
        else:
            plt_style = {
                "figure.facecolor": "#dce0e8",
                "axes.facecolor": "#eff1f5",
                "axes.edgecolor": "#9ca0b0",
                "axes.labelcolor": "#4c4f69",
                "text.color": "#4c4f69",
                "xtick.color": "#5c5f77",
                "ytick.color": "#5c5f77",
                "grid.color": "#ccd0da",
                "grid.alpha": 0.5,
            }
        matplotlib.rcParams.update(plt_style)

    def update_state(self, state: AppState):
        """Refresh plots from state."""
        self.state = state
        self._plot()

    def refresh_theme(self):
        """Refresh matplotlib theme when dark/light mode changes."""
        self._setup_style()
        self._plot()

    def _plot(self):
        """Generate analytical plots for the active converter."""
        self.figure.clear()
        self.ax_vout = self.figure.add_subplot(211)
        self.ax_il = self.figure.add_subplot(212)

        # Always use buck plotting for now
        self._plot_dc_converter()

        self.figure.tight_layout(pad=2.0)
        self.canvas.draw()

    def _plot_dc_converter(self):
        """Plot voltage and current for DC-DC converters (buck/boost)."""
        f = self.state.design.frequency
        t_period = 1.0 / f
        n_points = 200
        dt_plot = t_period * 3.0 / n_points

        t_arr = np.arange(n_points) * dt_plot
        duty = self.state.design.duty_cycle
        vout_val = self.state.results.vout
        iout_val = self.state.results.iout
        il_ripple = self.state.results.il_ripple
        vout_ripple = self.state.results.vout_ripple

        phase = (t_arr / t_period) % 1.0

        # Vout waveform with ripple
        vout_wave = vout_val + np.where(
            phase < duty, 1.0, -1.0
        ) * vout_ripple * 0.5

        # Inductor current (triangular approximation)
        il_wave = np.where(
            phase < duty,
            iout_val - il_ripple / 2.0 + (phase / duty) * il_ripple,
            iout_val + il_ripple / 2.0 - ((phase - duty) / (1.0 - duty)) * il_ripple,
        )

        t_us = t_arr * 1e6

        self.ax_vout.plot(t_us, vout_wave,
                          color="#89b4fa", linewidth=1.5)
        self.ax_vout.set_ylabel("Vout (V)")
        self.ax_vout.set_title(f"Output Voltage ({vout_val:.2f} V)")
        self.ax_vout.grid(True, alpha=0.3)
        self.ax_vout.set_xlim(t_us.min(), t_us.max())

        self.ax_il.plot(t_us, il_wave,
                        color="#a6e3a1", linewidth=1.5)
        self.ax_il.set_ylabel("I_L (A)")
        self.ax_il.set_xlabel("Time (μs)")
        self.ax_il.set_title(f"Inductor Current ({iout_val:.2f} A avg)")
        self.ax_il.grid(True, alpha=0.3)
        self.ax_il.set_xlim(t_us.min(), t_us.max())

    def _plot_vsi(self):
        """Plot output current for VSI."""
        f_mod = self.state.params.output_frequency
        t_period = 1.0 / f_mod
        n_points = 500
        dt_plot = t_period * 3.0 / n_points

        t_arr = np.arange(n_points) * dt_plot

        v1 = self.state.results.vout  # peak fundamental
        vrms = self.state.results.rms_output or (v1 / np.sqrt(2))
        r_load = self.state.params.load_resistance

        # Fundamental output voltage
        omega = 2.0 * np.pi * f_mod
        v_fundamental = v1 * np.sin(omega * t_arr)

        # Output current (simplified: V/R for resistive load)
        i_wave = v_fundamental / r_load

        t_ms = t_arr * 1e3

        self.ax_vout.plot(t_ms, v_fundamental,
                          color="#89b4fa", linewidth=1.5)
        self.ax_vout.set_ylabel("Vout (V)")
        self.ax_vout.set_title(f"Output Voltage (Vpeak={v1:.1f} V, Vrms={vrms:.1f} V)")
        self.ax_vout.grid(True, alpha=0.3)
        self.ax_vout.set_xlim(t_ms.min(), t_ms.max())

        self.ax_il.plot(t_ms, i_wave,
                        color="#a6e3a1", linewidth=1.5)
        self.ax_il.set_ylabel("Iout (A)")
        self.ax_il.set_xlabel("Time (ms)")
        self.ax_il.set_title(f"Output Current")
        self.ax_il.grid(True, alpha=0.3)
        self.ax_il.set_xlim(t_ms.min(), t_ms.max())

    def plot_simulation(self, t_sim: list, vout_sim: list, il_sim: list):
        """Overlay simulation results on the existing plots."""
        if len(t_sim) == 0:
            return

        t_sim_us = np.array(t_sim) * 1e6
        vout_arr = np.array(vout_sim)
        il_arr = np.array(il_sim)

        if len(vout_arr) > 0:
            self.ax_vout.plot(t_sim_us, vout_arr,
                              color="#f38ba8", linewidth=0.8,
                              alpha=0.7, label="Simulation")
            self.ax_vout.legend()

        if len(il_arr) > 0:
            self.ax_il.plot(t_sim_us, il_arr,
                            color="#f9e2af", linewidth=0.8,
                            alpha=0.7, label="Simulation")
            self.ax_il.legend()

        self.canvas.draw()
