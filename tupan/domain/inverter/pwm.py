"""PWM (Pulse Width Modulation) generation and analysis — pure functions."""

import math
from tupan.domain.converters.common import angular_frequency


def generate_pwm(ma: float, modulation_freq: float, carrier_freq: float,
                 num_periods: float, dt: float):
    """Generate a sine-triangle PWM switching function.

    For a given modulation index (ma) and carrier ratio (mf), generates
    the PWM switching pattern as a list of (time, value) pairs.

    Args:
        ma: Modulation index (0..1)
        modulation_freq: Modulating sine wave frequency (Hz)
        carrier_freq: Carrier triangle wave frequency (Hz)
        num_periods: Number of modulation periods to generate
        dt: Time step between samples (seconds)

    Returns:
        List of (time, switching_state) tuples where switching_state is
        1 for high, -1 for low (bipolar).
    """
    ma = max(0.0, min(1.0, ma))
    t_total = num_periods / modulation_freq
    n_points = int(t_total / dt)
    omega_m = angular_frequency(modulation_freq)
    omega_c = angular_frequency(carrier_freq)

    samples = []
    for i in range(n_points):
        t = i * dt
        if t > t_total:
            break
        v_mod = ma * math.sin(omega_m * t)
        phase_c = (omega_c * t) % (2.0 * math.pi)
        if phase_c < math.pi:
            triangle = phase_c / math.pi * 2.0 - 1.0
        else:
            fall_phase = phase_c - math.pi
            triangle = 1.0 - fall_phase / math.pi * 2.0

        state = 1.0 if v_mod >= triangle else -1.0
        samples.append((t, state))

    return samples


def duty_cycle_at_time(t: float, ma: float, mod_freq: float,
                       carrier_freq: float) -> float:
    """Calculate the instantaneous duty cycle at a given time.

    Returns the PWM switching state (-1 or 1) at time t for a
    sine-triangle PWM.
    """
    ma = max(0.0, min(1.0, ma))
    omega_m = angular_frequency(mod_freq)
    omega_c = angular_frequency(carrier_freq)

    v_mod = ma * math.sin(omega_m * t)
    phase_c = (omega_c * t) % (2.0 * math.pi)
    if phase_c < math.pi:
        triangle = phase_c / math.pi * 2.0 - 1.0
    else:
        fall_phase = phase_c - math.pi
        triangle = 1.0 - fall_phase / math.pi * 2.0

    return 1.0 if v_mod >= triangle else -1.0


def frequency_modulation_ratio(carrier_freq: float,
                               modulation_freq: float) -> float:
    """Calculate the frequency modulation ratio mf = fc / fm."""
    if modulation_freq <= 0.0:
        return 0.0
    return carrier_freq / modulation_freq
