"""Tests for PWM module — ported from src/domain/inverter/pwm.rs."""

import math
from tupan.domain.inverter.pwm import (
    generate_pwm,
    duty_cycle_at_time,
    frequency_modulation_ratio,
)


def test_generate_pwm_basic():
    """Basic PWM generation sanity check."""
    samples = generate_pwm(
        ma=0.8,
        modulation_freq=60.0,
        carrier_freq=10000.0,
        num_periods=1.0,
        dt=1e-5,
    )
    assert len(samples) > 100
    # Verify there are both high and low states
    states = set(s[1] for s in samples)
    assert 1.0 in states
    assert -1.0 in states


def test_duty_cycle_at_time():
    """Verify duty_cycle_at_time returns -1 or 1."""
    state = duty_cycle_at_time(0.0, 0.8, 60.0, 10000.0)
    assert state in (-1.0, 1.0)

    # At t=0, sin(0)=0, triangle starts at -1, so v_ref >= triangle -> 1.0
    state2 = duty_cycle_at_time(0.0, 0.5, 60.0, 1000.0)
    assert state2 == 1.0


def test_frequency_modulation_ratio():
    mf = frequency_modulation_ratio(10000.0, 60.0)
    assert abs(mf - 10000.0 / 60.0) < 1e-6

    mf_zero = frequency_modulation_ratio(10000.0, 0.0)
    assert mf_zero == 0.0
