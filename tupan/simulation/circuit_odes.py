"""ODE systems for power converter circuits.

Each class defines the state-space model of a converter.
State variables are typically inductor current(s) and capacitor voltage(s).
Uses NumPy arrays for state vectors.
"""

import numpy as np
from tupan.domain import ConverterParams


class BuckOde:
    """Buck converter ODE system.

    States:
      y[0] = iL  (inductor current)
      y[1] = vC  (capacitor voltage = Vout)

    During switch ON (duty cycle):
      diL/dt = (Vin - vC) / L
      dvC/dt = (iL - vC/R) / C

    During switch OFF:
      diL/dt = -vC / L
      dvC/dt = (iL - vC/R) / C
    """
    def __init__(self, vin: float, l: float, c: float, r: float,
                 frequency: float, duty: float):
        self.vin = vin
        self.l = l
        self.c = c
        self.r = r
        self.frequency = frequency
        self.duty = duty

    @classmethod
    def from_params(cls, params: ConverterParams) -> 'BuckOde':
        return cls(
            vin=params.vin,
            l=params.inductance,
            c=params.capacitance,
            r=params.load_resistance,
            frequency=params.frequency,
            duty=params.duty_cycle,
        )

    def switching(self, t: float) -> float:
        """Compute the switching function at time t (1=ON, 0=OFF)."""
        period = 1.0 / self.frequency
        phase = (t % period) / period
        return 1.0 if phase < self.duty else 0.0

    def derivatives(self, t: float, y: np.ndarray) -> np.ndarray:
        """Compute derivatives at time t for state vector y."""
        il, vc = y[0], y[1]
        s = self.switching(t)

        dil_dt = (s * self.vin - vc) / self.l
        dvc_dt = (il - vc / self.r) / self.c

        return np.array([dil_dt, dvc_dt])


class BoostOde:
    """Boost converter ODE system.

    States:
      y[0] = iL  (inductor current)
      y[1] = vC  (capacitor voltage = Vout)

    During switch ON:
      diL/dt = Vin / L
      dvC/dt = -vC / (R*C)

    During switch OFF:
      diL/dt = (Vin - vC) / L
      dvC/dt = (iL - vC/R) / C
    """
    def __init__(self, vin: float, l: float, c: float, r: float,
                 frequency: float, duty: float):
        self.vin = vin
        self.l = l
        self.c = c
        self.r = r
        self.frequency = frequency
        self.duty = duty

    @classmethod
    def from_params(cls, params: ConverterParams) -> 'BoostOde':
        return cls(
            vin=params.vin,
            l=params.inductance,
            c=params.capacitance,
            r=params.load_resistance,
            frequency=params.frequency,
            duty=params.duty_cycle,
        )

    def switching(self, t: float) -> float:
        period = 1.0 / self.frequency
        phase = (t % period) / period
        return 1.0 if phase < self.duty else 0.0

    def derivatives(self, t: float, y: np.ndarray) -> np.ndarray:
        il, vc = y[0], y[1]
        s = self.switching(t)

        dil_dt = (self.vin - (1.0 - s) * vc) / self.l
        dvc_dt = ((1.0 - s) * il - vc / self.r) / self.c

        return np.array([dil_dt, dvc_dt])


class VsiOde:
    """Single-phase VSI with RL load ODE system.

    States:
      y[0] = i_out  (output current)

    For a simple RL load with PWM voltage input:
      di/dt = (V_pwm(t) - R*i) / L_load

    Where V_pwm(t) = ±Vdc/2 depending on PWM state.
    """
    def __init__(self, vdc: float, r_load: float, l_load: float,
                 carrier_freq: float, mod_freq: float, ma: float):
        self.vdc = vdc
        self.r_load = r_load
        self.l_load = l_load
        self.carrier_freq = carrier_freq
        self.mod_freq = mod_freq
        self.ma = ma

    @classmethod
    def from_params(cls, params: ConverterParams) -> 'VsiOde':
        return cls(
            vdc=params.vin,
            r_load=params.load_resistance,
            l_load=params.inductance,  # using inductance as load inductance for RL load
            carrier_freq=params.frequency,
            mod_freq=params.output_frequency,
            ma=params.modulation_index,
        )

    def pwm_voltage(self, t: float) -> float:
        """Generate the PWM voltage at time t."""
        import math
        omega_m = 2.0 * math.pi * self.mod_freq
        omega_c = 2.0 * math.pi * self.carrier_freq

        v_ref = self.ma * math.sin(omega_m * t)
        phase_c = (omega_c * t) % (2.0 * math.pi)

        if phase_c < math.pi:
            triangle = phase_c / math.pi * 2.0 - 1.0
        else:
            triangle = 1.0 - (phase_c - math.pi) / math.pi * 2.0

        return self.vdc / 2.0 if v_ref >= triangle else -self.vdc / 2.0

    def derivatives(self, t: float, y: np.ndarray) -> np.ndarray:
        i = y[0]
        v_pwm = self.pwm_voltage(t)
        di_dt = (v_pwm - self.r_load * i) / self.l_load
        return np.array([di_dt])
