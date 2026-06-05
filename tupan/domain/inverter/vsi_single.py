"""Single-phase Voltage Source Inverter (VSI) analytical model — pure functions."""

from tupan.domain.metrics import thd as thd_module
from tupan.domain import ConverterResults

# Default loss parameters
R_SWITCH = 0.1     # IGBT/MOSFET typical Rds(on)
V_F = 1.0          # typical diode Vf for high voltage
T_RISE = 50e-9     # typical IGBT rise time
T_FALL = 50e-9     # typical IGBT fall time


def fundamental_output(ma: float, vdc: float, full_bridge: bool = True) -> float:
    """Calculate the fundamental output voltage amplitude.

    For a full-bridge VSI with sine-triangle PWM:
    V1 = ma * Vdc  (full-bridge)
    V1 = ma * Vdc / 2 (half-bridge)
    """
    return thd_module.fundamental_amplitude(ma, vdc, full_bridge)


def rms_output(ma: float, vdc: float, full_bridge: bool = True) -> float:
    """Calculate RMS output voltage."""
    return thd_module.rms_output_voltage(ma, vdc, full_bridge)


def output_current(vout_rms: float, load_resistance: float) -> float:
    """Calculate output current (resistive load, fundamental component)."""
    if load_resistance <= 0.0:
        return 0.0
    return vout_rms / load_resistance


def input_current(vout_rms: float, iout_rms: float, vdc: float) -> float:
    """Calculate input current (average DC side).

    Assuming ideal inverter: Pdc = Pac → Idc * Vdc = Vrms * Irms
    """
    if vdc <= 0.0:
        return 0.0
    return (vout_rms * iout_rms) / vdc


def conduction_losses(iout_rms: float,
                      r_switch: float = R_SWITCH,
                      v_f: float = V_F,
                      num_switches_conduction: float = 2.0) -> float:
    """Estimate conduction losses for a VSI (simplified).

    Includes IGBT/MOSFET and diode losses for all switches.
    Simplified: 2 switches conducting at any time.
    """
    i = abs(iout_rms)
    p_sw = i * i * r_switch * num_switches_conduction
    p_diode = i * v_f * num_switches_conduction * 0.3
    return p_sw + p_diode


def switching_losses(vdc: float, iout: float, switching_freq: float,
                     t_rise: float = T_RISE,
                     t_fall: float = T_FALL,
                     num_switches: float = 4.0) -> float:
    """Estimate switching losses for a VSI.

    Simplified: P_sw_total = num_switches * P_sw_per_switch
    """
    if switching_freq <= 0.0:
        return 0.0
    p_per_switch = 0.5 * vdc * abs(iout) * (t_rise + t_fall) * switching_freq
    return p_per_switch * num_switches


def calculate(vin: float, modulation_index: float, frequency: float,
              output_frequency: float, load_resistance: float,
              inductance: float, capacitance: float,
              full_bridge: bool = True,
              r_switch: float = R_SWITCH,
              v_f: float = V_F,
              t_rise: float = T_RISE,
              t_fall: float = T_FALL) -> ConverterResults:
    """Full analytical calculation for single-phase VSI.

    Returns a ConverterResults with all metrics computed.
    """
    v1 = fundamental_output(modulation_index, vin, full_bridge)
    vrms = rms_output(modulation_index, vin, full_bridge)
    iout = output_current(vrms, load_resistance)
    iin = input_current(vrms, iout, vin)

    # THD approximation for bipolar PWM
    thd_val = thd_module.pwm_thd_approximate(modulation_index, bipolar=True)

    num_switches = 4.0 if full_bridge else 2.0
    num_conducting = 2.0  # 2 switches always conducting in full bridge

    cond_losses = conduction_losses(iout, r_switch, v_f, num_conducting)
    sw_losses = switching_losses(vin, iout, frequency, t_rise, t_fall, num_switches)

    total_losses = cond_losses + sw_losses
    p_out = vrms * iout
    efficiency_val = p_out / (p_out + total_losses) if (p_out + total_losses) > 0.0 else 1.0

    return ConverterResults(
        vout=v1,  # peak fundamental
        iout=iout,
        iin=iin,
        vout_ripple=0.0,  # Not applicable for VSI (AC output)
        il_ripple=0.0,    # Not applicable directly
        conduction_losses=cond_losses,
        switching_losses=sw_losses,
        efficiency=efficiency_val,
        thd=thd_val,
        rms_output=vrms,
        fundamental_amplitude=v1,
    )
