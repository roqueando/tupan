"""Tests for BuckStrategy — all buck converter logic lives inside the class."""

import pytest
from tupan.domain.design_params import DesignParams
from tupan.domain.converters.buck import BUCK, BuckStrategy


# ── Analytical method tests ──

def test_output_voltage():
    """Buck: Vin=48V, D=0.25 -> Vout=12V"""
    vout = BuckStrategy._output_voltage(48.0, 0.25)
    assert abs(vout - 12.0) < 1e-6


def test_output_voltage_duty_100():
    """Clamped to 0.99"""
    vout = BuckStrategy._output_voltage(48.0, 1.0)
    assert abs(vout - 47.52) < 1e-6


def test_required_duty_cycle():
    """Vin=48V, Vout=12V -> D=0.25"""
    d = BuckStrategy._required_duty_cycle(48.0, 12.0)
    assert abs(d - 0.25) < 1e-6


def test_required_duty_cycle_zero_vin():
    d = BuckStrategy._required_duty_cycle(0.0, 12.0)
    assert abs(d - 0.0) < 1e-6


def test_inductor_current_ripple():
    """Buck: Vin=48V, D=0.25, f=100kHz, L=100uH -> 0.9A"""
    ripple = BuckStrategy._inductor_current_ripple(48.0, 0.25, 100_000.0, 100e-6)
    assert abs(ripple - 0.9) < 1e-6


def test_output_voltage_ripple():
    """dIL=0.9, f=100kHz, C=100uF -> 0.01125V"""
    ripple = BuckStrategy._output_voltage_ripple(0.9, 100_000.0, 100e-6)
    assert abs(ripple - 0.01125) < 1e-6


def test_output_current():
    i = BuckStrategy._output_current(12.0, 10.0)
    assert abs(i - 1.2) < 1e-6


def test_input_current():
    i = BuckStrategy._input_current(1.2, 0.25)
    assert abs(i - 0.3) < 1e-6


def test_conduction_losses():
    """I=1.2A, D=0.25, Rsw=0.1, RL=0.05, Vf=0.7 -> 0.738W"""
    losses = BuckStrategy._conduction_losses(1.2, 0.25, 0.1, 0.05, 0.7)
    assert abs(losses - 0.738) < 1e-6


def test_switching_losses():
    """Vin=48V, I=1.2A, f=100kHz, tr=20ns, tf=20ns -> 0.1152W"""
    losses = BuckStrategy._switching_losses(48.0, 1.2, 100_000.0, 20e-9, 20e-9)
    assert abs(losses - 0.1152) < 1e-6


# ── Strategy interface tests ──

def test_strategy_compute_components():
    """Strategy compute matches expected L, C, R for default params."""
    params = DesignParams()
    result = BUCK.compute_components(params)

    assert abs(result.delta_il_amps - 1.5) < 1e-6
    assert abs(result.delta_vo_volts - 0.12) < 1e-6
    assert abs(result.load_resistance - 2.4) < 1e-6
    assert abs(result.inductance - 60e-6) < 1e-9

    c_expected = 0.75 / (8.0 * 60e-6 * 0.12 * 100_000.0 * 100_000.0)
    assert abs(result.capacitance - c_expected) < 1e-12


def test_strategy_analyze_full_pipeline():
    """End-to-end: DesignParams -> compute -> analyze -> reasonable results."""
    params = DesignParams(vin=48.0, vout=12.0, iout_max=5.0)
    components = BUCK.compute_components(params)
    results = BUCK.analyze(params, components)

    assert abs(results.vout - 12.0) < 0.01
    assert abs(results.iout - 5.0) < 0.01
    assert results.efficiency > 0.9


def test_strategy_different_specs():
    """Strategy adapts to different design targets."""
    # Higher frequency -> lower L and C
    params = DesignParams(frequency=200_000.0)
    components = BUCK.compute_components(params)
    assert components.inductance < 60e-6  # lower than 60uH at 100kHz

    # Higher Iout,max -> lower R
    params2 = DesignParams(iout_max=10.0)
    components2 = BUCK.compute_components(params2)
    assert abs(components2.load_resistance - 1.2) < 1e-6


def test_strategy_vout_duty_helpers():
    """Strategy utility methods work correctly."""
    assert abs(BUCK.duty_from_vout(48.0, 12.0) - 0.25) < 1e-6
    assert abs(BUCK.vout_from_duty(48.0, 0.25) - 12.0) < 1e-6
