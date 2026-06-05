"""Design parameters and results dataclasses for the buck converter designer.

These represent the user-facing design specifications and the computed
component values that result from them.
"""

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class DesignParams:
    """Design-input parameters — what the user specifies.

    These are the operating conditions and design targets for a buck
    converter. Component values (L, C, R) are computed from these.
    """
    vin: float = 48.0            # Input voltage (V)
    vout: float = 12.0           # Target output voltage (V)
    duty_cycle: float = 0.25     # Derived from Vout/Vin, but user-overridable
    frequency: float = 100_000.0 # Switching frequency (Hz)
    iout_max: float = 5.0        # Maximum output current (A)
    delta_il_pct: float = 0.30   # Inductor current ripple as fraction of Iout,max
    delta_vo_pct: float = 0.01   # Output voltage ripple as fraction of Vout


@dataclass
class DesignResults:
    """Computed component values from the designer.

    These are the outputs of the design process — the component values
    needed to meet the design specifications.
    """
    delta_il_amps: float = 0.0   # Inductor current ripple in amperes
    delta_vo_volts: float = 0.0  # Output voltage ripple in volts
    inductance: float = 0.0      # Required inductance (H)
    capacitance: float = 0.0     # Required capacitance (F)
    load_resistance: float = 0.0 # Load resistance (Ω)
