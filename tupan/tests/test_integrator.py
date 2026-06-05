"""Tests for RK4 integrator — ported from src/simulation/integrator.rs."""

import math
import numpy as np
from tupan.simulation.integrator import (
    rk4_step,
    integrate,
    integrate_fixed,
)


def test_rk4_step():
    """dy/dt = -2y, y(0)=1 -> y(0.1) ~= e^(-0.2)"""
    def f(t, y):
        return np.array([-2.0 * y[0]])

    y0 = np.array([1.0])
    y1 = rk4_step(f, 0.0, y0, 0.1)

    expected = math.exp(-0.2)
    assert abs(y1[0] - expected) < 1e-4


def test_integrate_basic():
    """dy/dt = -2y, y(0)=1, integrate from 0 to 1"""
    def f(t, y):
        return np.array([-2.0 * y[0]])

    y0 = np.array([1.0])
    result = integrate(f, y0, (0.0, 1.0), 0.01, 10)

    assert len(result.t) >= 10
    final_y = result.y[-1][0]
    expected = math.exp(-2.0)
    assert abs(final_y - expected) < 1e-3


def test_integrate_fixed_max_points():
    """Verify max_points limits output size."""
    def f(t, y):
        return np.array([-2.0 * y[0]])

    y0 = np.array([1.0])
    result = integrate_fixed(f, y0, (0.0, 1.0), 0.001, 200)

    assert len(result.t) <= 250
    assert len(result.t) > 5


def test_harmonic_oscillator():
    """dy/dt = [y[1], -y[0]], y(0)=[1,0], t in [0, 2*pi]"""
    def harmonic(t, y):
        return np.array([y[1], -y[0]])

    y0 = np.array([1.0, 0.0])
    result = integrate(harmonic, y0, (0.0, 2.0 * math.pi), 0.01, 10)

    final_y = result.y[-1][0]
    assert abs(final_y - 1.0) < 0.01
