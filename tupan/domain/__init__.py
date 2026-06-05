"""Domain types shared across all converter models."""

from dataclasses import dataclass, field
from enum import Enum
from typing import Optional


class ConverterType(Enum):
    Buck = "Buck"
    Boost = "Boost"
    VsiSinglePhase = "VSI Single-Phase"


@dataclass
class ConverterParams:
    """Parameters for the analytical engine (internal use)."""
    vin: float = 48.0
    vout_target: float = 12.0
    frequency: float = 100_000.0
    duty_cycle: float = 0.5
    inductance: float = 100e-6
    capacitance: float = 100e-6
    load_resistance: float = 10.0
    modulation_index: float = 0.8
    output_frequency: float = 60.0


@dataclass
class ConverterResults:
    """Analytical engine results — operating point and performance."""
    vout: float = 0.0
    iout: float = 0.0
    iin: float = 0.0
    vout_ripple: float = 0.0
    il_ripple: float = 0.0
    conduction_losses: float = 0.0
    switching_losses: float = 0.0
    efficiency: float = 0.0
    thd: Optional[float] = None
    rms_output: Optional[float] = None
    fundamental_amplitude: Optional[float] = None
