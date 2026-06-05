"""Runge-Kutta 4th order (RK4) integrator for ODE systems.

Solves dy/dt = f(t, y) for a system of N coupled first-order ODEs.
Uses NumPy arrays for vectorized operations.
"""

from dataclasses import dataclass, field
from typing import Callable, List
import numpy as np


@dataclass
class SimulationResult:
    """Result of a simulation: time points and corresponding state vectors."""
    t: List[float] = field(default_factory=list)
    y: List[np.ndarray] = field(default_factory=list)


DerivFn = Callable[[float, np.ndarray], np.ndarray]


def rk4_step(f: DerivFn, t: float, y: np.ndarray, dt: float) -> np.ndarray:
    """Perform a single RK4 step.

    y_{n+1} = y_n + (dt/6) * (k1 + 2*k2 + 2*k3 + k4)
    """
    k1 = f(t, y)
    k2 = f(t + 0.5 * dt, y + 0.5 * dt * k1)
    k3 = f(t + 0.5 * dt, y + 0.5 * dt * k2)
    k4 = f(t + dt, y + dt * k3)

    return y + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)


def integrate(f: DerivFn, y0: np.ndarray, t_span: tuple,
              dt: float, sample_every: int = 1) -> SimulationResult:
    """Integrate a system of ODEs using RK4 from t_start to t_end.

    Args:
        f: Derivative function: dy/dt = f(t, y)
        y0: Initial state vector (numpy array)
        t_span: (start, end) time
        dt: Fixed time step
        sample_every: Store results every `sample_every` steps

    Returns:
        A SimulationResult with time points and state vectors.
    """
    t_start, t_end = t_span
    n_steps = max(1, int(np.ceil((t_end - t_start) / dt)))

    result = SimulationResult()
    y = np.asarray(y0, dtype=np.float64)
    t = t_start

    result.t.append(t)
    result.y.append(y.copy())

    for step in range(n_steps):
        y = rk4_step(f, t, y, dt)
        t += dt

        if step % sample_every == 0 or step == n_steps - 1:
            result.t.append(t)
            result.y.append(y.copy())

    return result


def integrate_fixed(f: DerivFn, y0: np.ndarray, t_span: tuple,
                    dt: float, max_points: int = 5000) -> SimulationResult:
    """Integrate with output decimation to limit total data points.

    Args:
        f: Derivative function
        y0: Initial state vector
        t_span: (start, end) time
        dt: Fixed time step
        max_points: Maximum number of output points

    Returns:
        A SimulationResult with time points and state vectors.
    """
    t_start, t_end = t_span
    total_time = t_end - t_start
    steps = max(1, int(total_time / dt))
    sample_every = max(1, steps // max_points) if steps > max_points else 1

    return integrate(f, y0, t_span, dt, sample_every)
