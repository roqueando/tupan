"""Buck converter designer — computes required component values from design specs.

This module contains the core design functions that take user-facing
design parameters (Vin, Vout, Iout,max, ΔiL%, ΔVo%, frequency) and
compute the required L, C, and R values.
"""

from tupan.domain.design_params import DesignParams, DesignResults


def clamp_duty(d: float) -> float:
    """Clamp duty cycle to valid range (0.01 .. 0.99)."""
    return max(0.01, min(0.99, d))


def design_buck(params: DesignParams) -> DesignResults:
    """Compute required component values for a buck converter.

    Args:
        params: Design specifications (Vin, Vout, frequency, Iout,max, etc.)

    Returns:
        DesignResults with computed L, C, R, ΔiL(A), ΔVo(V)
    """
    # Constrain duty cycle
    duty = clamp_duty(params.duty_cycle)

    # Compute derived quantities
    vin = params.vin
    vout = params.vout
    freq = params.frequency
    iout_max = params.iout_max

    delta_il_amps = params.delta_il_pct * iout_max
    delta_vo_volts = params.delta_vo_pct * vout

    # Load resistance from Ohm's law (full load = max current)
    r_load = vout / iout_max if iout_max > 0.0 else 10.0

    # Inductance: L = Vout * (1 - D) / (ΔiL_A * f)
    # This is the buck formula for the required inductance to achieve the
    # specified current ripple.
    if delta_il_amps > 0.0 and freq > 0.0:
        l_value = (vout * (1.0 - duty)) / (delta_il_amps * freq)
    else:
        l_value = 0.0

    # Capacitance: C = (1 - D) / (8 * L * ΔVo_V * f²)
    # This is the buck formula for the required capacitance to achieve the
    # specified output voltage ripple.
    if l_value > 0.0 and delta_vo_volts > 0.0 and freq > 0.0:
        c_value = (1.0 - duty) / (8.0 * l_value * delta_vo_volts * freq * freq)
    else:
        c_value = 0.0

    return DesignResults(
        delta_il_amps=delta_il_amps,
        delta_vo_volts=delta_vo_volts,
        inductance=l_value,
        capacitance=c_value,
        load_resistance=r_load,
    )
