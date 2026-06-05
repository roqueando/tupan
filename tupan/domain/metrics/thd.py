"""Total Harmonic Distortion (THD) calculation for inverter outputs — pure functions."""

import math


def thd_from_harmonics(harmonics: list, fundamental: float) -> float:
    """Calculate THD from harmonic amplitudes.

    THD = sqrt(Σ(Vh²)) / V1  (for h >= 2)
    """
    if abs(fundamental) <= 1e-12:
        return 0.0
    sum_sq = sum(h * h for h in harmonics)
    return math.sqrt(sum_sq) / abs(fundamental)


def pwm_thd_approximate(modulation_index: float, bipolar: bool = True) -> float:
    """Theoretical THD for a PWM sine wave with given modulation index.

    Simplified model based on double Fourier series for sine-triangle PWM.

    For ma = 0.8, THD ≈ 85-105% typically for bipolar PWM.
    """
    ma = max(0.0, min(1.0, modulation_index))

    if bipolar:
        if ma < 0.01:
            return 10.0
        thd_sq = (1.12 / ma) ** 2 - 1.0
        return min(math.sqrt(thd_sq), 10.0)
    else:
        if ma < 0.01:
            return 5.0
        thd_sq = (0.6 / ma) ** 2 - 1.0
        return min(math.sqrt(thd_sq), 5.0)


def fundamental_amplitude(modulation_index: float, vdc: float,
                          is_full_bridge: bool = True) -> float:
    """Calculate the fundamental component amplitude for a PWM inverter.

    For bipolar sine-triangle PWM:
    V1_fundamental = ma * Vdc / 2  (single-phase half-bridge)
    V1_fundamental = ma * Vdc       (single-phase full-bridge)
    """
    ma = max(0.0, min(1.0, modulation_index))
    if is_full_bridge:
        return ma * vdc
    else:
        return ma * vdc / 2.0


def rms_output_voltage(modulation_index: float, vdc: float,
                       is_full_bridge: bool = True) -> float:
    """Calculate RMS output voltage for a PWM inverter (fundamental).

    Vrms_fundamental = V1_fundamental / sqrt(2)
    """
    v1 = fundamental_amplitude(modulation_index, vdc, is_full_bridge)
    return v1 / math.sqrt(2.0)
