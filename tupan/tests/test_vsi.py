"""Tests for VSI module — ported from src/domain/inverter/vsi_single.rs."""

import pytest
from tupan.domain.inverter.vsi_single import (
    fundamental_output,
    rms_output,
    output_current,
    conduction_losses,
    switching_losses,
    calculate,
)
from tupan.domain import ConverterParams


def test_fundamental_output_full_bridge():
    """ma=0.8, Vdc=300V -> V1=240V"""
    v1 = fundamental_output(0.8, 300.0, full_bridge=True)
    assert abs(v1 - 240.0) < 1e-6


def test_fundamental_output_half_bridge():
    """ma=0.8, Vdc=300V -> V1=120V"""
    v1 = fundamental_output(0.8, 300.0, full_bridge=False)
    assert abs(v1 - 120.0) < 1e-6


def test_full_calculation_full_bridge():
    """Full VSI calculation with typical params."""
    result = calculate(
        vin=300.0,
        modulation_index=0.8,
        frequency=10000.0,
        output_frequency=60.0,
        load_resistance=10.0,
        inductance=100e-6,
        capacitance=100e-6,
        full_bridge=True,
    )
    assert abs(result.vout - 240.0) < 1.0
    assert result.thd is not None
    assert result.thd > 0.0
    assert result.efficiency > 0.7


def test_output_current():
    i = output_current(169.7, 10.0)
    assert abs(i - 16.97) < 0.01


def test_conduction_losses_vsi():
    """I=10A, Rsw=0.1, Vf=1.0V, 2 switches -> 26W"""
    losses = conduction_losses(10.0, 0.1, 1.0, 2.0)
    assert abs(losses - 26.0) < 0.01
