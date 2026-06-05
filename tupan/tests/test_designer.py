"""Tests for the buck converter designer module."""

import pytest
from tupan.domain.design_params import DesignParams, DesignResults
from tupan.domain.designer import design_buck, clamp_duty


def test_clamp_duty():
    assert clamp_duty(0.5) == 0.5
    assert clamp_duty(0.0) == 0.01
    assert clamp_duty(1.0) == 0.99
    assert clamp_duty(-0.1) == 0.01
    assert clamp_duty(2.0) == 0.99


def test_design_buck_defaults():
    """Test with default params: Vin=48, Vout=12, Iout,max=5A, ΔiL=30%, ΔVo=1%."""
    params = DesignParams()
    result = design_buck(params)

    # Duty = 12/48 = 0.25
    assert result is not None

    # ΔiL in amps = 30% of 5A = 1.5A
    assert abs(result.delta_il_amps - 1.5) < 1e-6

    # ΔVo in volts = 1% of 12V = 0.12V
    assert abs(result.delta_vo_volts - 0.12) < 1e-6

    # R = Vout / Iout,max = 12 / 5 = 2.4Ω
    assert abs(result.load_resistance - 2.4) < 1e-6

    # L = Vout*(1-D) / (ΔiL_A * f) = 12*0.75 / (1.5*100k) = 9 / 150000 = 60μH
    assert abs(result.inductance - 60e-6) < 1e-9

    # C = (1-D) / (8*L*ΔVo_V*f²) = 0.75 / (8*60e-6*0.12*1e10)
    #   = 0.75 / (8*60e-6*0.12*1e10) = 0.75 / 576000 = 1.302μF... let me calculate
    c_expected = 0.75 / (8.0 * 60e-6 * 0.12 * 100_000.0 * 100_000.0)
    assert abs(result.capacitance - c_expected) < 1e-12


def test_design_buck_higher_current():
    """Iout,max=10A → lower L, lower R."""
    params = DesignParams(iout_max=10.0)
    result = design_buck(params)

    # ΔiL = 30% of 10A = 3A
    assert abs(result.delta_il_amps - 3.0) < 1e-6

    # R = 12/10 = 1.2Ω
    assert abs(result.load_resistance - 1.2) < 1e-6

    # L = 12*0.75 / (3*100k) = 9/300000 = 30μH
    assert abs(result.inductance - 30e-6) < 1e-9


def test_design_buck_more_ripple():
    """ΔiL=50% → more ripple allowed → smaller L."""
    params = DesignParams(delta_il_pct=0.50)
    result = design_buck(params)

    # ΔiL = 50% of 5A = 2.5A
    assert abs(result.delta_il_amps - 2.5) < 1e-6

    # L = 12*0.75 / (2.5*100k) = 9/250000 = 36μH
    expected_l = 12.0 * 0.75 / (2.5 * 100_000.0)
    assert abs(result.inductance - expected_l) < 1e-9


def test_design_buck_higher_frequency():
    """f=200kHz → L and C both go down."""
    params = DesignParams(frequency=200_000.0)
    result = design_buck(params)

    # L = 12*0.75 / (1.5*200k) = 9/300000 = 30μH
    assert abs(result.inductance - 30e-6) < 1e-9

    # C ∝ 1/f² → with double freq, C is 1/4 of original
    c_original = 0.75 / (8.0 * 60e-6 * 0.12 * 1e10)
    c_higher = 0.75 / (8.0 * 30e-6 * 0.12 * 4e10)
    assert abs(result.capacitance - c_higher) < 1e-12
    assert result.capacitance < c_original


def test_design_buck_duty_override():
    """D=50% → Vout effectively = Vin*D=24V."""
    params = DesignParams(duty_cycle=0.50, vout=24.0)
    result = design_buck(params)

    # L = 24*(1-0.5) / (1.5*100k) = 12/150000 = 80μH
    expected_l = 24.0 * 0.5 / (1.5 * 100_000.0)
    assert abs(result.inductance - expected_l) < 1e-9


def test_design_buck_zero_iout():
    """Edge case: Iout,max=0."""
    params = DesignParams(iout_max=0.0)
    result = design_buck(params)
    assert result.load_resistance > 0  # defaults to 10
    assert result.inductance == 0.0  # ΔiL_A = 0 → can't compute L


def test_design_buck_low_current():
    """Iout,max=20mA (0.02A) — very low current."""
    params = DesignParams(iout_max=0.02)
    result = design_buck(params)

    # R = 12 / 0.02 = 600 Ohm
    assert abs(result.load_resistance - 600.0) < 1e-6

    # dIL_A = 30% of 0.02 = 0.006A = 6mA
    assert abs(result.delta_il_amps - 0.006) < 1e-6

    # L = 12*0.75/(0.006*100k) = 9/600 = 0.015H = 15mH
    assert abs(result.inductance - 0.015) < 1e-9


def test_design_buck_zero_freq():
    """Edge case: frequency=0."""
    params = DesignParams(frequency=0.0)
    result = design_buck(params)
    assert result.inductance == 0.0
    assert result.capacitance == 0.0
