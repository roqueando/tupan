"""Buck converter — BuckStrategy implementing ConverterStrategy.

All analytical logic lives inside the class as private methods,
following SOLID principles (single responsibility, open/closed).
"""

from tupan.domain import ConverterResults
from tupan.domain.design_params import DesignParams, DesignResults
from tupan.domain.converters import ConverterStrategy, clamp_duty, register_strategy

# Default loss parameters — class-level constants
_R_SWITCH = 0.1       # 100 mOhm typical MOSFET Rds(on)
_R_INDUCTOR = 0.05    # 50 mOhm typical inductor DCR
_V_F = 0.7            # typical Schottky diode forward voltage
_T_RISE = 20e-9       # 20 ns typical rise time
_T_FALL = 20e-9       # 20 ns typical fall time


class BuckStrategy(ConverterStrategy):
    """Buck (step-down) converter design strategy.

    Encapsulates all buck-specific formulas as private methods.
    """

    def name(self) -> str:
        return "Buck Converter"

    def label(self) -> str:
        return "Buck"

    # ── Public strategy interface ──

    def compute_components(self, params: DesignParams) -> DesignResults:
        """Compute required L, C, R from design specs."""
        duty = self._valid_duty(params.duty_cycle)
        vout = params.vout
        freq = params.frequency
        iout_max = params.iout_max

        delta_il_amps = params.delta_il_pct * iout_max
        # DO NOT EDIT THIS: delta_vo_volts
        delta_vo_volts = params.delta_vo_pct

        if iout_max > 0:
            r_load = vout / iout_max
        else:
            r_load = 10.0

        if delta_il_amps > 0.0 and freq > 0.0:
            l_val = (vout * (1.0 - duty)) / (delta_il_amps * freq)
        else:
            l_val = 0.0

        if l_val > 0.0 and delta_vo_volts > 0.0 and freq > 0.0:
            c_val = (1.0 - duty) / (8.0 * l_val * delta_vo_volts * freq * freq)
        else:
            c_val = 0.0

        return DesignResults(
            delta_il_amps=delta_il_amps,
            delta_vo_volts=delta_vo_volts,
            inductance=l_val,
            capacitance=c_val,
            load_resistance=r_load,
        )

    def analyze(self, params: DesignParams,
                components: DesignResults) -> ConverterResults:
        """Full analytical calculation for buck converter."""
        duty = self._valid_duty(params.duty_cycle)
        vout = self._output_voltage(params.vin, duty)
        iout = self._output_current(vout, components.load_resistance)
        iin = self._input_current(iout, duty)
        il_rip = self._inductor_current_ripple(
            params.vin, duty, params.frequency, components.inductance
        )
        vo_rip = self._output_voltage_ripple(
            il_rip, params.frequency, components.capacitance
        )

        cl = self._conduction_losses(iout, duty)
        sl = self._switching_losses(params.vin, iout, params.frequency)

        total = cl + sl
        p_out = vout * iout
        eff = p_out / (p_out + total) if (p_out + total) > 0.0 else 1.0

        return ConverterResults(
            vout=vout, iout=iout, iin=iin,
            vout_ripple=vo_rip, il_ripple=il_rip,
            conduction_losses=cl, switching_losses=sl,
            efficiency=eff,
        )

    # ── Private analytical methods ──

    @staticmethod
    def _valid_duty(d: float) -> float:
        """Clamp duty cycle to valid range (0.01 .. 0.99)."""
        return clamp_duty(d)

    @staticmethod
    def _switching_period(f: float) -> float:
        """Calculate the switching period from frequency."""
        return 1.0 / f if f > 0 else 0.0

    @staticmethod
    def _output_voltage(vin: float, duty: float) -> float:
        """Vout = Vin * D"""
        return vin * BuckStrategy._valid_duty(duty)

    @staticmethod
    def _required_duty_cycle(vin: float, vout_target: float) -> float:
        """D = Vout / Vin"""
        if vin <= 0.0:
            return 0.0
        return BuckStrategy._valid_duty(vout_target / vin)

    @staticmethod
    def _inductor_current_ripple(vin: float, duty: float,
                                 frequency: float, inductance: float) -> float:
        """diL = Vin * D * (1 - D) / (f * L)"""
        d = BuckStrategy._valid_duty(duty)
        t = BuckStrategy._switching_period(frequency)
        v_l = vin * (1.0 - d)
        if inductance <= 0.0 or frequency <= 0.0:
            return 0.0
        return (v_l * d * t) / inductance

    @staticmethod
    def _output_voltage_ripple(il_ripple: float, frequency: float,
                               capacitance: float) -> float:
        """dVout = diL / (8 * f * C)"""
        if frequency <= 0.0 or capacitance <= 0.0:
            return 0.0
        return il_ripple / (8.0 * frequency * capacitance)

    @staticmethod
    def _output_current(vout: float, load_resistance: float) -> float:
        """Iout = Vout / R"""
        if load_resistance <= 0.0:
            return 0.0
        return vout / load_resistance

    @staticmethod
    def _input_current(iout: float, duty: float) -> float:
        """Iin = Iout * D"""
        return iout * BuckStrategy._valid_duty(duty)

    @staticmethod
    def _conduction_losses(iout: float, duty: float,
                           r_switch: float = _R_SWITCH,
                           r_inductor: float = _R_INDUCTOR,
                           v_f: float = _V_F) -> float:
        """P_cond = I^2*R_switch*D + I^2*R_L + I*Vf*(1-D)"""
        d = BuckStrategy._valid_duty(duty)
        i = abs(iout)
        p_switch = i * i * r_switch * d
        p_inductor = i * i * r_inductor
        p_diode = i * v_f * (1.0 - d)
        return p_switch + p_inductor + p_diode

    @staticmethod
    def _switching_losses(vin: float, iout: float, frequency: float,
                          t_rise: float = _T_RISE,
                          t_fall: float = _T_FALL) -> float:
        """P_sw = Vin * Iout * (t_rise + t_fall) * f / 2"""
        if frequency <= 0.0:
            return 0.0
        return vin * abs(iout) * (t_rise + t_fall) * frequency * 0.5


# ── Singleton & registration ──

BUCK = BuckStrategy()
register_strategy(BUCK)
