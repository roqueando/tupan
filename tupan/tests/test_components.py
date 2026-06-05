"""Tests for component design modules — ported from Rust tests."""

import math
import pytest


# ── Inductor ──
from tupan.domain.components.inductor import (
    buck_required_inductance,
    boost_required_inductance,
    peak_current,
    rms_current,
)


def test_buck_required_inductance():
    """Vin=48V, D=0.25, f=100kHz, dIL=0.9A"""
    l = buck_required_inductance(48.0, 0.25, 100_000.0, 0.9)
    # The formula: L = Vin * D * (1-D) / (f * dIL)
    expected = 48.0 * 0.25 * 0.75 / (100_000.0 * 0.9)
    assert abs(l - expected) < 1e-9


def test_peak_current():
    """I_avg=1.2A, dIL=0.9A -> I_peak=1.65A"""
    i_peak = peak_current(1.2, 0.9)
    assert abs(i_peak - 1.65) < 1e-6


def test_rms_current():
    """I_avg=1.2A, dIL=0.9A -> I_rms = sqrt(1.44 + 0.81/12)"""
    i_rms = rms_current(1.2, 0.9)
    expected = math.sqrt(1.2**2 + 0.9**2 / 12.0)
    assert abs(i_rms - expected) < 1e-6


# ── Capacitor ──
from tupan.domain.components.capacitor import (
    buck_required_capacitance,
    capacitor_rms_current,
    recommended_voltage_rating,
)


def test_buck_required_capacitance():
    """dIL=0.9A, f=100kHz, dVout=0.01V -> 112.5 uF"""
    c = buck_required_capacitance(0.9, 100_000.0, 0.01)
    assert abs(c - 112.5e-6) < 1e-9


def test_capacitor_rms_current():
    """dIL=0.9A -> I_c_rms = 0.9/sqrt(12)"""
    i_rms = capacitor_rms_current(0.9)
    expected = 0.9 / math.sqrt(12.0)
    assert abs(i_rms - expected) < 1e-6


def test_recommended_voltage_rating():
    """Vout=12V, margin=1.5 -> 18V"""
    rating = recommended_voltage_rating(12.0, 1.5)
    assert abs(rating - 18.0) < 1e-6


# ── Load ──
from tupan.domain.components.load import (
    resistive_power,
    rl_time_constant,
)


def test_resistive_power():
    """Vout=12V, R=10 -> 14.4W"""
    p = resistive_power(12.0, 10.0)
    assert abs(p - 14.4) < 1e-6


def test_rl_time_constant():
    """L=100uH, R=10 -> 10us"""
    tau = rl_time_constant(100e-6, 10.0)
    assert abs(tau - 10e-6) < 1e-9
