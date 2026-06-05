"""Application state — single source of truth.

Uses the ConverterStrategy pattern to dispatch design/analysis to
the correct converter type.
"""

from dataclasses import dataclass, field
from enum import Enum

from tupan.domain import ConverterResults
from tupan.domain.design_params import DesignParams, DesignResults
from tupan.domain.converters.buck import BUCK
from tupan.domain.converters import clamp_duty


class Theme(Enum):
    Dark = "Dark"
    Light = "Light"


@dataclass
class AppState:
    """Main application state — single source of truth."""
    theme: Theme = Theme.Dark
    status_message: str = "Ready"
    design: DesignParams = field(default_factory=DesignParams)
    computed: DesignResults = field(default_factory=DesignResults)
    results: ConverterResults = field(default_factory=ConverterResults)
    show_numerical_sim: bool = False
    show_schematic: bool = True

    # Strategy — which converter we're designing for
    strategy = BUCK

    # Simulation results (populated after RK4 run)
    sim_t: list = field(default_factory=list)
    sim_vout: list = field(default_factory=list)
    sim_il: list = field(default_factory=list)

    def recalculate(self):
        """Full recalculation pipeline using the active strategy.

        1. Sync Vout ↔ Duty Cycle
        2. Compute L, C, R from design specs → DesignResults
        3. Feed into analytical engine → ConverterResults
        """
        self.status_message = "Calculating..."

        # Sync Vout and Duty
        d = self.design.vout / self.design.vin if self.design.vin > 0 else 0.0
        self.design.duty_cycle = clamp_duty(d)
        self.design.vout = self.design.vin * self.design.duty_cycle

        # Compute components via strategy
        self.computed = self.strategy.compute_components(self.design)

        # Analyze via strategy
        self.results = self.strategy.analyze(self.design, self.computed)

        self.status_message = "Ready"

    def on_vout_changed(self, new_vout: float):
        """User edited Vout → recalculate Duty from Vout/Vin."""
        if self.design.vin > 0:
            self.design.vout = max(0.5, min(500.0, new_vout))
            d = self.design.vout / self.design.vin
            self.design.duty_cycle = clamp_duty(d)
            self.design.vout = self.design.vin * self.design.duty_cycle
            self.recalculate()

    def on_duty_changed(self, new_duty_pct: float):
        """User edited Duty % → recalculate Vout = Vin * D."""
        d = clamp_duty(new_duty_pct / 100.0)
        self.design.duty_cycle = d
        self.design.vout = self.design.vin * d
        self.recalculate()

    def on_vin_changed(self, new_vin: float):
        """User edited Vin → keep Vout target, recalc Duty."""
        self.design.vin = max(1.0, min(1000.0, new_vin))
        d = self.design.vout / self.design.vin if self.design.vin > 0 else 0.0
        self.design.duty_cycle = clamp_duty(d)
        self.design.vout = self.design.vin * self.design.duty_cycle
        self.recalculate()

    def reset_params(self):
        """Reset to default design parameters."""
        self.design = DesignParams()
        self.recalculate()
