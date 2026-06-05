"""Tests for ODE system models — ported from src/simulation/circuit_odes.rs."""

import numpy as np
from tupan.domain import ConverterParams
from tupan.simulation.circuit_odes import BuckOde, BoostOde, VsiOde
from tupan.simulation.integrator import integrate_fixed


def test_buck_ode_steady_state():
    """Buck: Vin=48V, D=0.25, L=100uH, C=100uF, R=10 -> Vout~12V"""
    params = ConverterParams(
        vin=48.0,
        vout_target=12.0,
        frequency=100_000.0,
        duty_cycle=0.25,
        inductance=100e-6,
        capacitance=100e-6,
        load_resistance=10.0,
    )
    ode = BuckOde.from_params(params)

    vout_est = params.vin * params.duty_cycle
    iout_est = vout_est / params.load_resistance
    y0 = np.array([iout_est, vout_est])

    result = integrate_fixed(
        lambda t, y: ode.derivatives(t, y),
        y0, (0.0, 0.005), 1e-8, 5000,
    )
    final_y = result.y[-1]
    assert abs(final_y[1] - vout_est) < 1.0, \
        f"Vout should be near {vout_est} V, got {final_y[1]} V"


def test_boost_ode_steady_state():
    """Boost: Vin=12V, D=0.5, L=100uH, C=100uF, R=10 -> Vout~24V"""
    params = ConverterParams(
        vin=12.0,
        vout_target=24.0,
        frequency=100_000.0,
        duty_cycle=0.5,
        inductance=100e-6,
        capacitance=100e-6,
        load_resistance=10.0,
    )
    ode = BoostOde.from_params(params)

    vout_est = params.vin / (1.0 - params.duty_cycle)
    iout_est = vout_est / params.load_resistance
    iin_est = iout_est / (1.0 - params.duty_cycle)
    y0 = np.array([iin_est, vout_est])

    result = integrate_fixed(
        lambda t, y: ode.derivatives(t, y),
        y0, (0.0, 0.005), 1e-8, 5000,
    )
    final_y = result.y[-1]
    assert abs(final_y[1] - vout_est) < 2.0, \
        f"Vout should be near {vout_est} V, got {final_y[1]} V"


def test_vsi_ode_basic():
    """VSI: check that ODE integration runs and produces non-zero current."""
    params = ConverterParams(
        vin=300.0,
        modulation_index=0.8,
        frequency=10_000.0,
        output_frequency=60.0,
        inductance=1e-3,
        load_resistance=10.0,
    )
    ode = VsiOde.from_params(params)

    y0 = np.array([0.0])
    result = integrate_fixed(
        lambda t, y: ode.derivatives(t, y),
        y0, (0.0, 0.05), 1e-6, 10000,
    )
    final_y = result.y[-1]
    assert abs(final_y[0]) < 50.0, \
        f"Current should be reasonable, got {final_y[0]} A"
    assert abs(final_y[0]) > 0.1, \
        f"Current should be non-zero, got {final_y[0]} A"
