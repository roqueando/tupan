"""Application state — single source of truth for the Tupan application.

Now uses DesignParams as the user-facing parameter model, with the designer
module computing required component values (L, C, R) from design specs.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Optional

from tupan.domain import ConverterType, ConverterResults
from tupan.domain.design_params import DesignParams, DesignResults
from tupan.domain.designer import design_buck, clamp_duty


class Theme(Enum):
    Dark = "Dark"
    Light = "Light"


@dataclass
class AppState:
    """Main application state — single source of truth."""
    theme: Theme = Theme.Dark
    status_message: str = "Ready"
    active_converter: ConverterType = ConverterType.Buck
    design: DesignParams = field(default_factory=DesignParams)
    computed: DesignResults = field(default_factory=DesignResults)
    results: ConverterResults = field(default_factory=ConverterResults)
    show_numerical_sim: bool = False
    show_schematic: bool = True

    # Simulation results (populated after RK4 run)
    sim_t: list = field(default_factory=list)
    sim_vout: list = field(default_factory=list)
    sim_il: list = field(default_factory=list)

    def recalculate(self):
        """Full recalculation pipeline.

        1. Sync Vout ↔ Duty Cycle (whichever changed last)
        2. Compute ΔiL(A), ΔVo(V), L, C, R from design specs
        3. Feed into analytical engine for operating point results
        """
        self.status_message = "Calculating..."

        # ── Step 1: Ensure Vout and Duty are consistent ──
        # Duty takes priority: Duty was likely the last thing edited
        # Vout = Vin * Duty
        # But if Vout was edited, Duty = Vout / Vin
        # We handle this by always ensuring consistency:
        # duty_cycle always reflects Vout / Vin, EXCEPT when user
        # explicitly overrode it. We handle this in the UI layer by
        # setting a flag or swapping. For now, we keep it simple:
        # duty = vout / vin, clamped.
        d_from_vout = self.design.vout / self.design.vin if self.design.vin > 0 else 0.0
        self.design.duty_cycle = clamp_duty(d_from_vout)

        # Recalculate Vout from the clamped duty cycle
        self.design.vout = self.design.vin * self.design.duty_cycle

        # ── Step 2: Compute component values from design specs ──
        self.computed = design_buck(self.design)

        # ── Step 3: Feed into analytical engine ──
        if self.active_converter == ConverterType.Buck:
            from tupan.domain.converters.buck import calculate as buck_calc
            self.results = buck_calc(
                vin=self.design.vin,
                vout_target=self.design.vout,
                frequency=self.design.frequency,
                duty_cycle=self.design.duty_cycle,
                inductance=self.computed.inductance,
                capacitance=self.computed.capacitance,
                load_resistance=self.computed.load_resistance,
            )

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
        """Reset to default design parameters for buck converter."""
        self.design = DesignParams()
        self.recalculate()
