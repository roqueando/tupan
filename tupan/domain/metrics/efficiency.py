"""Efficiency calculation utilities — pure functions."""


def efficiency(p_out: float, total_losses: float) -> float:
    """Calculate efficiency from output power and total losses.

    η = Pout / (Pout + Ploss)
    """
    if (p_out + total_losses) <= 0.0:
        return 1.0
    return p_out / (p_out + total_losses)


def efficiency_percent(eff: float) -> str:
    """Format efficiency as percentage string."""
    return f"{eff * 100.0:.1f}%"


def mosfet_power_loss(i_drain: float, r_ds_on: float, duty: float,
                      v_ds: float, t_rise: float, t_fall: float,
                      frequency: float) -> float:
    """Calculate power dissipation in a MOSFET.

    P_mosfet = I²*R_ds_on*D + 0.5*Vin*I*(t_rise+t_fall)*f
    """
    d = max(0.0, min(1.0, duty))
    i = abs(i_drain)
    p_conduction = i * i * r_ds_on * d
    if frequency > 0.0:
        p_switching = 0.5 * v_ds * i * (t_rise + t_fall) * frequency
    else:
        p_switching = 0.0
    return p_conduction + p_switching


def diode_power_loss(i_forward: float, v_f: float,
                     conduction_fraction: float) -> float:
    """Calculate power dissipation in a diode (simplified).

    P_diode = If * Vf * conduction_fraction
    """
    return abs(i_forward) * v_f * max(0.0, min(1.0, conduction_fraction))
