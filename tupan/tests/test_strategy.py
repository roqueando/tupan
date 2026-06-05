"""Tests for the ConverterStrategy pattern and BuckStrategy."""

import pytest
from tupan.domain.design_params import DesignParams, DesignResults
from tupan.domain.converters import (
    clamp_duty, get_strategy, get_all_strategies, register_strategy,
    ConverterStrategy,
)
from tupan.domain.converters.buck import BuckStrategy, BUCK
from tupan.domain import ConverterResults


# ── clamp_duty ──

def test_clamp_duty():
    assert clamp_duty(0.5) == 0.5
    assert clamp_duty(0.0) == 0.01
    assert clamp_duty(1.0) == 0.99
    assert clamp_duty(-0.1) == 0.01
    assert clamp_duty(2.0) == 0.99


# ── Registry ──

def test_registry_has_buck():
    assert get_strategy("BuckStrategy") is not None
    assert get_strategy("BuckStrategy").label() == "Buck"


def test_get_all_strategies():
    all_s = get_all_strategies()
    assert len(all_s) >= 1
    assert any(s.label() == "Buck" for s in all_s)


# ── BuckStrategy.name ──

def test_buck_strategy_name():
    assert BUCK.name() == "Buck Converter"
    assert BUCK.label() == "Buck"


# ── BuckStrategy.compute_components ──

def test_buck_compute_defaults():
    params = DesignParams()
    result = BUCK.compute_components(params)

    assert abs(result.delta_il_amps - 1.5) < 1e-6
    assert abs(result.delta_vo_volts - 0.12) < 1e-6
    assert abs(result.load_resistance - 2.4) < 1e-6
    assert abs(result.inductance - 60e-6) < 1e-9

    c_expected = 0.75 / (8.0 * 60e-6 * 0.12 * 100_000.0 * 100_000.0)
    assert abs(result.capacitance - c_expected) < 1e-12


def test_buck_compute_higher_current():
    params = DesignParams(iout_max=10.0)
    result = BUCK.compute_components(params)

    assert abs(result.delta_il_amps - 3.0) < 1e-6
    assert abs(result.load_resistance - 1.2) < 1e-6
    assert abs(result.inductance - 30e-6) < 1e-9


def test_buck_compute_more_ripple():
    params = DesignParams(delta_il_pct=0.50)
    result = BUCK.compute_components(params)

    assert abs(result.delta_il_amps - 2.5) < 1e-6
    expected_l = 12.0 * 0.75 / (2.5 * 100_000.0)
    assert abs(result.inductance - expected_l) < 1e-9


def test_buck_compute_higher_frequency():
    params = DesignParams(frequency=200_000.0)
    result = BUCK.compute_components(params)

    assert abs(result.inductance - 30e-6) < 1e-9
    c_higher = 0.75 / (8.0 * 30e-6 * 0.12 * 4e10)
    assert abs(result.capacitance - c_higher) < 1e-12


def test_buck_compute_duty_override():
    params = DesignParams(duty_cycle=0.50, vout=24.0)
    result = BUCK.compute_components(params)

    expected_l = 24.0 * 0.5 / (1.5 * 100_000.0)
    assert abs(result.inductance - expected_l) < 1e-9


def test_buck_compute_low_current():
    """20mA — very low current."""
    params = DesignParams(iout_max=0.02)
    result = BUCK.compute_components(params)

    assert abs(result.load_resistance - 600.0) < 1e-6
    assert abs(result.delta_il_amps - 0.006) < 1e-6
    assert abs(result.inductance - 0.015) < 1e-9


def test_buck_compute_zero_iout():
    params = DesignParams(iout_max=0.0)
    result = BUCK.compute_components(params)
    assert result.load_resistance > 0
    assert result.inductance == 0.0


def test_buck_compute_zero_freq():
    params = DesignParams(frequency=0.0)
    result = BUCK.compute_components(params)
    assert result.inductance == 0.0
    assert result.capacitance == 0.0


# ── BuckStrategy.analyze ──

def test_buck_analyze_defaults():
    """Full pipeline: design -> compute -> analyze."""
    from tupan.domain.converters.buck import BUCK
    params = DesignParams()
    components = BUCK.compute_components(params)
    results = BUCK.analyze(params, components)

    assert results is not None
    assert abs(results.vout - 12.0) < 0.01
    assert abs(results.iout - 5.0) < 0.01
    assert abs(results.iin - 1.25) < 0.01
    assert results.efficiency > 0.9


def test_buck_analyze_different_load():
    """With different Iout,max, check results change."""
    params = DesignParams(iout_max=10.0)
    components = BUCK.compute_components(params)
    results = BUCK.analyze(params, components)

    # R = 12/10 = 1.2, Iout = 12/1.2 = 10A
    assert abs(results.iout - 10.0) < 0.01
    assert abs(results.iin - 2.5) < 0.01


# ── Utility methods ──

def test_duty_from_vout():
    assert abs(BUCK.duty_from_vout(48.0, 12.0) - 0.25) < 1e-6
    assert abs(BUCK.duty_from_vout(0.0, 12.0) - 0.01) < 1e-6


def test_vout_from_duty():
    assert abs(BUCK.vout_from_duty(48.0, 0.25) - 12.0) < 1e-6
    assert abs(BUCK.vout_from_duty(48.0, 0.5) - 24.0) < 1e-6
