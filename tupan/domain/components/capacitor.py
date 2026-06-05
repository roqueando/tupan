"""Capacitor design and parameter calculation — pure functions."""

import math


def buck_required_capacitance(il_ripple: float, frequency: float,
                              vout_ripple: float) -> float:
    """Calculate the required capacitance for a given output voltage ripple in buck.

    C = ΔiL / (8 * f * ΔVout)
    """
    if frequency <= 0.0 or vout_ripple <= 0.0:
        return 0.0
    return il_ripple / (8.0 * frequency * vout_ripple)


def boost_required_capacitance(iout: float, duty: float, frequency: float,
                               vout_ripple: float) -> float:
    """Calculate the required capacitance for a given output voltage ripple in boost.

    C = Iout * D / (f * ΔVout)
    """
    if frequency <= 0.0 or vout_ripple <= 0.0:
        return 0.0
    d = max(0.0, min(1.0, duty))
    return iout * d / (frequency * vout_ripple)


def capacitor_rms_current(ripple_current: float) -> float:
    """Calculate the RMS current through the output capacitor.

    For buck: I_c_rms = ΔiL / sqrt(12)
    """
    return ripple_current / math.sqrt(12.0)


def recommended_voltage_rating(vout: float, safety_margin: float = 1.5) -> float:
    """Calculate a recommended capacitor voltage rating with safety margin."""
    return vout * safety_margin
