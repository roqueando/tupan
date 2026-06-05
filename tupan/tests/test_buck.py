"""Tests for buck converter module — ported from src/domain/converters/buck.rs."""

import pytest
from tupan.domain.converters.buck import (
    output_voltage,
    required_duty_cycle,
    inductor_current_ripple,
    output_voltage_ripple,
    output_current,
    input_current,
    conduction_losses,
    switching_losses,
    calculate,
)


def test_output_voltage():
    """Buck: Vin=48V, D=0.25 -> Vout=12V"""
    vout = output_voltage(48.0, 0.25)
    assert abs(vout - 12.0) < 1e-6


def test_output_voltage_duty_100():
    """Clamped to 0.99"""
    vout = output_voltage(48.0, 1.0)
    assert abs(vout - 47.52) < 1e-6


def test_required_duty_cycle():
    """Vin=48V, Vout=12V -> D=0.25"""
    d = required_duty_cycle(48.0, 12.0)
    assert abs(d - 0.25) < 1e-6


def test_required_duty_cycle_zero_vin():
    d = required_duty_cycle(0.0, 12.0)
    assert abs(d - 0.0) < 1e-6


def test_inductor_current_ripple():
    """Buck: Vin=48V, D=0.25, f=100kHz, L=100uH -> 0.9A"""
    ripple = inductor_current_ripple(48.0, 0.25, 100_000.0, 100e-6)
    assert abs(ripple - 0.9) < 1e-6


def test_output_voltage_ripple():
    """dIL=0.9, f=100kHz, C=100uF -> 0.01125V"""
    ripple = output_voltage_ripple(0.9, 100_000.0, 100e-6)
    assert abs(ripple - 0.01125) < 1e-6


def test_output_current():
    i = output_current(12.0, 10.0)
    assert abs(i - 1.2) < 1e-6


def test_input_current():
    i = input_current(1.2, 0.25)
    assert abs(i - 0.3) < 1e-6


def test_full_calculation():
    result = calculate(
        vin=48.0,
        vout_target=12.0,
        frequency=100_000.0,
        duty_cycle=0.25,
        inductance=100e-6,
        capacitance=100e-6,
        load_resistance=10.0,
    )
    assert abs(result.vout - 12.0) < 0.01
    assert abs(result.iout - 1.2) < 0.01
    assert result.efficiency > 0.8
    assert result.vout_ripple > 0.0
    assert result.il_ripple > 0.0


def test_conduction_losses():
    """I=1.2A, D=0.25, Rsw=0.1, RL=0.05, Vf=0.7 -> 0.738W"""
    losses = conduction_losses(1.2, 0.25, 0.1, 0.05, 0.7)
    assert abs(losses - 0.738) < 1e-6


def test_switching_losses():
    """Vin=48V, I=1.2A, f=100kHz, tr=20ns, tf=20ns -> 0.1152W"""
    losses = switching_losses(48.0, 1.2, 100_000.0, 20e-9, 20e-9)
    assert abs(losses - 0.1152) < 1e-6
