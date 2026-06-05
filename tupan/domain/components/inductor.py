"""Inductor design and parameter calculation — pure functions."""

import math


def buck_required_inductance(vin: float, duty: float, frequency: float,
                             ripple_current: float) -> float:
    """Calculate the required inductance for a given current ripple in buck.

    L = Vin * D * (1-D) / (f * ΔiL)
    """
    if frequency <= 0.0 or ripple_current <= 0.0:
        return 0.0
    d = max(0.0, min(1.0, duty))
    return vin * d * (1.0 - d) / (frequency * ripple_current)


def boost_required_inductance(vin: float, duty: float, frequency: float,
                              ripple_current: float) -> float:
    """Calculate the required inductance for a given current ripple in boost.

    L = Vin * D / (f * ΔiL)
    """
    if frequency <= 0.0 or ripple_current <= 0.0:
        return 0.0
    d = max(0.0, min(1.0, duty))
    return vin * d / (frequency * ripple_current)


def peak_current(i_avg: float, ripple_current: float) -> float:
    """Calculate the peak current through the inductor (for saturation check).

    I_peak = I_avg + ΔiL / 2
    """
    return abs(i_avg) + ripple_current / 2.0


def rms_current(i_avg: float, ripple_current: float) -> float:
    """Calculate the RMS current through the inductor.

    I_rms = sqrt(I_avg² + (ΔiL)² / 12)
    """
    return math.sqrt(i_avg ** 2 + ripple_current ** 2 / 12.0)
