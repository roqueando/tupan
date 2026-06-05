"""Tests for boost converter module — ported from src/domain/converters/boost.rs."""

import pytest
from tupan.domain.converters.boost import (
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
    """Boost: Vin=12V, D=0.5 -> Vout=24V"""
    vout = output_voltage(12.0, 0.5)
    assert abs(vout - 24.0) < 1e-6


def test_output_voltage_higher_duty():
    """Boost: Vin=12V, D=0.75 -> Vout=48V"""
    vout = output_voltage(12.0, 0.75)
    assert abs(vout - 48.0) < 1e-6


def test_required_duty_cycle():
    """Vin=12V, Vout=24V -> D=0.5"""
    d = required_duty_cycle(12.0, 24.0)
    assert abs(d - 0.5) < 1e-6


def test_required_duty_cycle_vout_less_than_vin():
    """Vout <= Vin -> minimum duty"""
    d = required_duty_cycle(24.0, 12.0)
    assert abs(d - 0.01) < 1e-6


def test_inductor_current_ripple():
    """Boost: Vin=12V, D=0.5, f=100kHz, L=100uH -> 0.6A"""
    ripple = inductor_current_ripple(12.0, 0.5, 100_000.0, 100e-6)
    assert abs(ripple - 0.6) < 1e-6


def test_output_voltage_ripple():
    """Iout=2.4A, D=0.5, f=100kHz, C=100uF -> 0.12V"""
    ripple = output_voltage_ripple(2.4, 0.5, 100_000.0, 100e-6)
    assert abs(ripple - 0.12) < 1e-6


def test_full_calculation():
    result = calculate(
        vin=12.0,
        vout_target=24.0,
        frequency=100_000.0,
        duty_cycle=0.5,
        inductance=100e-6,
        capacitance=100e-6,
        load_resistance=10.0,
    )
    assert abs(result.vout - 24.0) < 0.01
    assert abs(result.iout - 2.4) < 0.01
    assert abs(result.iin - 4.8) < 0.01
    assert result.efficiency > 0.8


def test_input_current():
    """Boost: Iout=2.4A, D=0.5 -> Iin=4.8A"""
    iin = input_current(2.4, 0.5)
    assert abs(iin - 4.8) < 1e-6
