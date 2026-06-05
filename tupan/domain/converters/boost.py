"""Boost converter analytical model — pure functions.

All functions in this module are stateless and have no GUI dependency.
"""

from tupan.domain.converters.common import valid_duty_cycle

# Default loss parameters
R_SWITCH = 0.1       # 100 mOhm typical MOSFET Rds(on)
R_INDUCTOR = 0.05    # 50 mOhm typical inductor DCR
V_F = 0.7            # typical Schottky diode forward voltage
T_RISE = 20e-9       # 20 ns typical rise time
T_FALL = 20e-9       # 20 ns typical fall time


def output_voltage(vin: float, duty: float) -> float:
    """Calculate output voltage for a boost converter in CCM.

    Vout = Vin / (1 - D)
    """
    d = valid_duty_cycle(duty)
    return vin / (1.0 - d)


def required_duty_cycle(vin: float, vout_target: float) -> float:
    """Calculate the required duty cycle to achieve a target output voltage.

    D = 1 - Vin / Vout
    """
    if vin <= 0.0 or vout_target <= vin:
        return 0.01  # minimum duty
    d = 1.0 - vin / vout_target
    return valid_duty_cycle(d)


def inductor_current_ripple(vin: float, duty: float, frequency: float,
                            inductance: float) -> float:
    """Calculate inductor current ripple (peak-to-peak) in CCM.

    ΔiL = Vin * D / (f * L)
    """
    d = valid_duty_cycle(duty)
    if inductance <= 0.0 or frequency <= 0.0:
        return 0.0
    return (vin * d) / (frequency * inductance)


def output_voltage_ripple(iout: float, duty: float, frequency: float,
                          capacitance: float) -> float:
    """Calculate output voltage ripple (peak-to-peak) in CCM.

    ΔVout = Iout * D / (f * C)
    """
    d = valid_duty_cycle(duty)
    if frequency <= 0.0 or capacitance <= 0.0:
        return 0.0
    return (iout * d) / (frequency * capacitance)


def output_current(vout: float, load_resistance: float) -> float:
    """Calculate average output current."""
    if load_resistance <= 0.0:
        return 0.0
    return vout / load_resistance


def input_current(iout: float, duty: float) -> float:
    """Calculate input current (average).

    For an ideal boost: Pin = Pout, so Iin * Vin = Vout * Iout
    Iin = Iout / (1 - D)
    """
    d = valid_duty_cycle(duty)
    return iout / (1.0 - d)


def conduction_losses(iin: float, iout: float, duty: float,
                      r_switch: float = R_SWITCH,
                      r_inductor: float = R_INDUCTOR,
                      v_f: float = V_F) -> float:
    """Estimate conduction losses for a boost converter.

    P_cond = I²*R_switch*D + I²*R_L + I*Vf
    """
    d = valid_duty_cycle(duty)
    iin_abs = abs(iin)
    iout_abs = abs(iout)
    p_switch = iin_abs * iin_abs * r_switch * d
    p_inductor = iin_abs * iin_abs * r_inductor
    p_diode = iout_abs * v_f
    return p_switch + p_inductor + p_diode


def switching_losses(vin: float, iin: float, frequency: float,
                     t_rise: float = T_RISE,
                     t_fall: float = T_FALL) -> float:
    """Estimate switching losses for a boost converter.

    P_sw = Vin * Iin * (t_rise + t_fall) * f / 2
    """
    if frequency <= 0.0:
        return 0.0
    return vin * abs(iin) * (t_rise + t_fall) * frequency * 0.5


def calculate(vin: float, vout_target: float, frequency: float,
              duty_cycle: float, inductance: float, capacitance: float,
              load_resistance: float,
              r_switch: float = R_SWITCH,
              r_inductor: float = R_INDUCTOR,
              v_f: float = V_F,
              t_rise: float = T_RISE,
              t_fall: float = T_FALL):
    """Full analytical calculation for boost converter.

    Returns a ConverterResults dataclass with all computed metrics.
    """
    from tupan.domain import ConverterResults

    duty = required_duty_cycle(vin, vout_target)
    vout = output_voltage(vin, duty)
    iout = output_current(vout, load_resistance)
    iin = input_current(iout, duty)
    il_ripple = inductor_current_ripple(vin, duty, frequency, inductance)
    vout_ripple = output_voltage_ripple(iout, duty, frequency, capacitance)

    cond_losses = conduction_losses(iin, iout, duty, r_switch, r_inductor, v_f)
    sw_losses = switching_losses(vin, iin, frequency, t_rise, t_fall)

    total_losses = cond_losses + sw_losses
    p_out = vout * iout
    efficiency_val = p_out / (p_out + total_losses) if (p_out + total_losses) > 0.0 else 1.0

    return ConverterResults(
        vout=vout,
        iout=iout,
        iin=iin,
        vout_ripple=vout_ripple,
        il_ripple=il_ripple,
        conduction_losses=cond_losses,
        switching_losses=sw_losses,
        efficiency=efficiency_val,
        thd=None,
        rms_output=None,
        fundamental_amplitude=None,
    )
