"""Plot panel — Matplotlib waveforms over time.

Shows output voltage and inductor current over several switching
periods, with clear time-axis labels.
"""

import numpy as np
import matplotlib
matplotlib.use("QtAgg")

from matplotlib.backends.backend_qtagg import FigureCanvasQTAgg
from matplotlib.figure import Figure
from PySide6.QtWidgets import QVBoxLayout, QWidget

from tupan.app.state import AppState
from tupan.ui.theme import get_colors


class PlotPanel(QWidget):
    """Matplotlib plots for voltage and current waveforms over time."""

    def __init__(self, state, parent=None):
        super().__init__(parent)
        self.state = state

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)

        self.figure = Figure(figsize=(8, 4), dpi=100)
        self.canvas = FigureCanvasQTAgg(self.figure)

        self.ax_vout = self.figure.add_subplot(211)
        self.ax_il = self.figure.add_subplot(212)

        layout.addWidget(self.canvas)

        self._setup_style()

    def _setup_style(self):
        is_dark = self.state.theme.value == "Dark"
        if is_dark:
            matplotlib.rcParams.update({
                "figure.facecolor": "#11111b",
                "axes.facecolor": "#1e1e2e",
                "axes.edgecolor": "#6c7086",
                "axes.labelcolor": "#cdd6f4",
                "text.color": "#cdd6f4",
                "xtick.color": "#a6adc8",
                "ytick.color": "#a6adc8",
                "grid.color": "#313244",
                "grid.alpha": 0.5,
            })
        else:
            matplotlib.rcParams.update({
                "figure.facecolor": "#dce0e8",
                "axes.facecolor": "#eff1f5",
                "axes.edgecolor": "#9ca0b0",
                "axes.labelcolor": "#4c4f69",
                "text.color": "#4c4f69",
                "xtick.color": "#5c5f77",
                "ytick.color": "#5c5f77",
                "grid.color": "#ccd0da",
                "grid.alpha": 0.5,
            })

    def update_state(self, state):
        self.state = state
        self._plot()

    def refresh_theme(self):
        self._setup_style()
        self._plot()

    def _plot(self):
        self.figure.clear()
        self.ax_vout = self.figure.add_subplot(211)
        self.ax_il = self.figure.add_subplot(212)

        self._plot_waveforms()

        self.figure.tight_layout(pad=2.0)
        self.canvas.draw()

    def _plot_waveforms(self):
        """Plot Vout and IL over several switching periods.

        Shows 10 switching periods so the waveform pattern is visible
        over time. X-axis is in microseconds.
        """
        f = self.state.design.frequency
        t_period = 1.0 / f               # seconds per switching period
        n_periods = 10                    # show 10 switching periods
        n_points = 200 * n_periods        # resolution
        t_total = t_period * n_periods    # total time span
        dt = t_total / n_points

        t_arr = np.arange(n_points) * dt

        duty = self.state.design.duty_cycle
        vout_val = self.state.results.vout
        iout_val = self.state.results.iout
        il_ripple = self.state.results.il_ripple
        vout_ripple = self.state.results.vout_ripple

        # Normalized phase within each switching period (0..1)
        phase = (t_arr / t_period) % 1.0

        # Vout: square wave with ripple
        vout_wave = vout_val + np.where(
            phase < duty, 1.0, -1.0
        ) * vout_ripple * 0.5

        # IL: triangular wave
        il_wave = np.where(
            phase < duty,
            iout_val - il_ripple / 2.0 + (phase / duty) * il_ripple,
            iout_val + il_ripple / 2.0 - ((phase - duty) / (1.0 - duty)) * il_ripple,
        )

        # Time in microseconds
        t_us = t_arr * 1e6

        # ── Vout plot ──
        self.ax_vout.plot(t_us, vout_wave,
                          color="#89b4fa", linewidth=1.2)
        self.ax_vout.set_ylabel("Vout (V)")
        self.ax_vout.set_title(
            f"Output Voltage — {vout_val:.2f} V avg, "
            f"{vout_ripple*1e3:.2f} mV ripple"
        )
        self.ax_vout.grid(True, alpha=0.3)
        self.ax_vout.set_xlim(t_us.min(), t_us.max())

        # ── IL plot ──
        self.ax_il.plot(t_us, il_wave,
                        color="#a6e3a1", linewidth=1.2)
        self.ax_il.set_ylabel("I_L (A)")
        self.ax_il.set_xlabel("Time (μs)")
        self.ax_il.set_title(
            f"Inductor Current — {iout_val:.2f} A avg, "
            f"{il_ripple*1e3:.1f} mA ripple"
        )
        self.ax_il.grid(True, alpha=0.3)
        self.ax_il.set_xlim(t_us.min(), t_us.max())

    def plot_simulation_overlay(self, t_sim, vout_sim, il_sim):
        """Overlay RK4 simulation results on the analytical plots."""
        if not t_sim:
            return

        t_sim_us = np.array(t_sim) * 1e6
        vout_arr = np.array(vout_sim)
        il_arr = np.array(il_sim)

        if len(vout_arr) > 0:
            self.ax_vout.plot(t_sim_us, vout_arr,
                              color="#f38ba8", linewidth=0.8,
                              alpha=0.7, label="RK4 sim")
            self.ax_vout.legend(loc="upper right")

        if len(il_arr) > 0:
            self.ax_il.plot(t_sim_us, il_arr,
                            color="#f9e2af", linewidth=0.8,
                            alpha=0.7, label="RK4 sim")
            self.ax_il.legend(loc="upper right")

        self.canvas.draw()
