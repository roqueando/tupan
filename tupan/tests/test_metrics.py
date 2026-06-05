"""Tests for metrics modules — ported from Rust tests."""

import math
import pytest

# ── Efficiency ──
from tupan.domain.metrics.efficiency import (
    efficiency,
    efficiency_percent,
    mosfet_power_loss,
    diode_power_loss,
)


def test_efficiency():
    eff = efficiency(100.0, 10.0)
    assert abs(eff - 100.0 / 110.0) < 1e-6


def test_efficiency_format():
    s = efficiency_percent(0.912)
    assert s == "91.2%"


def test_mosfet_power_loss():
    """I=1.2A, Rds=0.1, D=0.25, Vds=48V, tr=20ns, tf=20ns, f=100kHz"""
    loss = mosfet_power_loss(1.2, 0.1, 0.25, 48.0, 20e-9, 20e-9, 100_000.0)
    assert abs(loss - 0.1512) < 1e-6


def test_diode_power_loss():
    """If=1.2A, Vf=0.7V, conducts 75% of time"""
    loss = diode_power_loss(1.2, 0.7, 0.75)
    assert abs(loss - 0.63) < 1e-6


# ── Ripple ──
from tupan.domain.metrics.ripple import (
    buck_critical_inductance,
    boost_critical_inductance,
    buck_min_capacitance,
)


def test_buck_critical_inductance():
    """D=0.25, R=10, f=100kHz -> 37.5 uH"""
    l = buck_critical_inductance(0.25, 10.0, 100_000.0)
    assert abs(l - 37.5e-6) < 1e-9


def test_boost_critical_inductance():
    """D=0.5, R=10, f=100kHz -> 6.25 uH"""
    l = boost_critical_inductance(0.5, 10.0, 100_000.0)
    assert abs(l - 6.25e-6) < 1e-9


def test_buck_min_capacitance():
    """dIL=0.9, f=100kHz, dVout=0.01V -> 112.5 uF"""
    c = buck_min_capacitance(0.9, 100_000.0, 0.01)
    assert abs(c - 112.5e-6) < 1e-9


# ── THD ──
from tupan.domain.metrics.thd import (
    thd_from_harmonics,
    fundamental_amplitude,
    rms_output_voltage,
    pwm_thd_approximate,
)


def test_thd_from_harmonics():
    """Fund=100V, harmonics: 3rd=20V, 5th=10V, 7th=5V -> THD~0.229"""
    harmonics = [20.0, 10.0, 5.0]
    thd = thd_from_harmonics(harmonics, 100.0)
    assert abs(thd - 0.2291) < 0.001


def test_fundamental_amplitude():
    """ma=0.8, Vdc=300V, full-bridge -> 240V"""
    v1 = fundamental_amplitude(0.8, 300.0, True)
    assert abs(v1 - 240.0) < 1e-6

    v1_half = fundamental_amplitude(0.8, 300.0, False)
    assert abs(v1_half - 120.0) < 1e-6


def test_rms_output_voltage():
    """ma=0.8, Vdc=300V, full-bridge -> Vrms=240/sqrt(2)~169.7V"""
    vrms = rms_output_voltage(0.8, 300.0, True)
    assert abs(vrms - 240.0 / math.sqrt(2.0)) < 1e-6


def test_pwm_thd_approximate():
    thd = pwm_thd_approximate(0.8, bipolar=True)
    assert thd > 0.5
    assert thd < 10.0
