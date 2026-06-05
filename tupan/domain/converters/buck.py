"""Buck converter analytical model — pure functions.

All functions in this module are stateless and have no GUI dependency.
They take float parameters and return float or ConverterResults dataclass.
"""

from tupan.domain.converters.common import switching_period, valid_duty_cycle

# Default loss parameters
R_SWITCH = 0.1       # 100 mOhm typical MOSFET Rds(on)
R_INDUCTOR = 0.05    # 50 mOhm typical inductor DCR
V_F = 0.7            # typical Schottky diode forward voltage
T_RISE = 20e-9       # 20 ns typical rise time
T_FALL = 20e-9       # 20 ns typical fall time


def output_voltage(vin: float, duty: float) -> float:
    """Calculate output voltage for a buck converter in CCM.

    Vout = Vin * D
    """
    return vin * valid_duty_cycle(duty)


def required_duty_cycle(vin: float, vout_target: float) -> float:
    """Calculate the required duty cycle to achieve a target output voltage.

    D = Vout / Vin
    """
    if vin <= 0.0:
        return 0.0
    return valid_duty_cycle(vout_target / vin)


def inductor_current_ripple(vin: float, duty: float, frequency: float,
                            inductance: float) -> float:
    """Calculate inductor current ripple (peak-to-peak) in CCM.

    ΔiL = Vin * D * (1 - D) / (f * L)
    """
    d = valid_duty_cycle(duty)
    t = switching_period(frequency)
    v_l = vin * (1.0 - d)  # voltage across inductor during on-time
    if inductance <= 0.0 or frequency <= 0.0:
        return 0.0
    return (v_l * d * t) / inductance


def output_voltage_ripple(il_ripple: float, frequency: float,
                          capacitance: float) -> float:
    """Calculate output voltage ripple (peak-to-peak) in CCM.

    ΔVout = ΔiL / (8 * f * C)
    """
    if frequency <= 0.0 or capacitance <= 0.0:
        return 0.0
    return il_ripple / (8.0 * frequency * capacitance)


def output_current(vout: float, load_resistance: float) -> float:
    """Calculate average output current."""
    if load_resistance <= 0.0:
        return 0.0
    return vout / load_resistance


def input_current(iout: float, duty: float) -> float:
    """Calculate input current (average).

    Iin = Iout * D
    """
    return iout * valid_duty_cycle(duty)


def conduction_losses(iout: float, duty: float,
                      r_switch: float = R_SWITCH,
                      r_inductor: float = R_INDUCTOR,
                      v_f: float = V_F) -> float:
    """Estimate conduction losses for a buck converter.

    P_cond = I²*R_switch*D + I²*R_L + I*Vf*(1-D)
    """
    d = valid_duty_cycle(duty)
    i = abs(iout)
    p_switch = i * i * r_switch * d
    p_inductor = i * i * r_inductor
    p_diode = i * v_f * (1.0 - d)
    return p_switch + p_inductor + p_diode


def switching_losses(vin: float, iout: float, frequency: float,
                     t_rise: float = T_RISE,
                     t_fall: float = T_FALL) -> float:
    """Estimate switching losses for a buck converter.

    P_sw = Vin * Iout * (t_rise + t_fall) * f / 2
    """
    if frequency <= 0.0:
        return 0.0
    return vin * abs(iout) * (t_rise + t_fall) * frequency * 0.5


def calculate(vin: float, vout_target: float, frequency: float,
              duty_cycle: float, inductance: float, capacitance: float,
              load_resistance: float,
              r_switch: float = R_SWITCH,
              r_inductor: float = R_INDUCTOR,
              v_f: float = V_F,
              t_rise: float = T_RISE,
              t_fall: float = T_FALL):
    """Full analytical calculation for buck converter.

    Returns a dict with all computed metrics.
    """
    from tupan.domain import ConverterResults

    duty = required_duty_cycle(vin, vout_target)
    vout = output_voltage(vin, duty)
    iout = output_current(vout, load_resistance)
    iin = input_current(iout, duty)
    il_ripple = inductor_current_ripple(vin, duty, frequency, inductance)
    vout_ripple = output_voltage_ripple(il_ripple, frequency, capacitance)

    cond_losses = conduction_losses(iout, duty, r_switch, r_inductor, v_f)
    sw_losses = switching_losses(vin, iout, frequency, t_rise, t_fall)

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
