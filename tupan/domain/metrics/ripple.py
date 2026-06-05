"""Ripple calculation utilities shared across converter types — pure functions."""


def buck_critical_inductance(duty: float, load_resistance: float,
                             frequency: float) -> float:
    """Calculate the critical inductance for CCM boundary in a buck converter.

    L_crit = (1 - D) * R / (2 * f)
    """
    if frequency <= 0.0:
        return 0.0
    d = max(0.0, min(1.0, duty))
    return (1.0 - d) * load_resistance / (2.0 * frequency)


def boost_critical_inductance(duty: float, load_resistance: float,
                              frequency: float) -> float:
    """Calculate the critical inductance for CCM boundary in a boost converter.

    L_crit = D * (1 - D)² * R / (2 * f)
    """
    if frequency <= 0.0:
        return 0.0
    d = max(0.0, min(1.0, duty))
    return d * (1.0 - d) ** 2 * load_resistance / (2.0 * frequency)


def buck_min_capacitance(il_ripple: float, frequency: float,
                         vout_ripple_req: float) -> float:
    """Calculate the minimum capacitance for a given output voltage ripple.

    For buck: C_min = ΔiL / (8 * f * ΔVout_req)
    """
    if frequency <= 0.0 or vout_ripple_req <= 0.0:
        return 0.0
    return il_ripple / (8.0 * frequency * vout_ripple_req)


def boost_min_capacitance(iout: float, duty: float, frequency: float,
                          vout_ripple_req: float) -> float:
    """Calculate the minimum capacitance for a boost converter.

    For boost: C_min = Iout * D / (f * ΔVout_req)
    """
    if frequency <= 0.0 or vout_ripple_req <= 0.0:
        return 0.0
    d = max(0.0, min(1.0, duty))
    return iout * d / (frequency * vout_ripple_req)
