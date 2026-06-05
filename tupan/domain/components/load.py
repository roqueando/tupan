"""Load models for converter analysis — pure functions."""


def resistive_power(vout: float, load_resistance: float) -> float:
    """Calculate output power for a resistive load."""
    if load_resistance <= 0.0:
        return 0.0
    return vout * vout / load_resistance


def resistive_current(vout: float, load_resistance: float) -> float:
    """Calculate output current for a resistive load."""
    if load_resistance <= 0.0:
        return 0.0
    return vout / load_resistance


def rl_time_constant(inductance: float, resistance: float) -> float:
    """Calculate load time constant for RL load."""
    if resistance <= 0.0:
        return 0.0
    return inductance / resistance


def rc_corner_frequency(capacitance: float, resistance: float) -> float:
    """Calculate the corner frequency for an RC load."""
    if capacitance <= 0.0 or resistance <= 0.0:
        return 0.0
    return 1.0 / (2.0 * 3.141592653589793 * resistance * capacitance)
