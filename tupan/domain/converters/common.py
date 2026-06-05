"""Shared utilities for converter calculations."""

import math


def angular_frequency(f: float) -> float:
    """Calculate the angular frequency from frequency in Hz."""
    return 2.0 * math.pi * f


def switching_period(f: float) -> float:
    """Calculate the switching period from frequency."""
    return 1.0 / f


def clamp(value: float, low: float, high: float) -> float:
    """Clamp a value between low and high."""
    if value < low:
        return low
    if value > high:
        return high
    return value


def valid_duty_cycle(d: float) -> float:
    """Duty cycle clamped to valid range (0.01 .. 0.99)."""
    return clamp(d, 0.01, 0.99)
