"""Tests for the ConverterStrategy pattern and registry."""

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


def test_register_custom_strategy():
    """Can register a new strategy and find it in the registry."""
    class MockStrategy(ConverterStrategy):
        def name(self): return "Mock"
        def label(self): return "Mock"
        def compute_components(self, p): return DesignResults()
        def analyze(self, p, c): return ConverterResults()

    register_strategy(MockStrategy())
    assert get_strategy("MockStrategy") is not None
    assert any(s.label() == "Mock" for s in get_all_strategies())


# ── BuckStrategy basics ──

def test_buck_strategy_name():
    assert BUCK.name() == "Buck Converter"
    assert BUCK.label() == "Buck"
    assert isinstance(BUCK, BuckStrategy)
    assert isinstance(BUCK, ConverterStrategy)


# ── BUCK singleton is registered ──

def test_buck_is_registered():
    assert get_strategy("BuckStrategy") is BUCK
