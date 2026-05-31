#pragma once

/// Shared utilities for converter calculations.

#include <cmath>

namespace converter_common {

/// Pi constant
constexpr double PI = 3.14159265358979323846;

/// Calculate the angular frequency from frequency in Hz.
inline double angular_frequency(double f) {
    return 2.0 * PI * f;
}

/// Calculate the switching period from frequency.
inline double switching_period(double f) {
    return 1.0 / f;
}

/// Clamp a value between min and max.
inline double clamp(double value, double min, double max) {
    if (value < min) return min;
    if (value > max) return max;
    return value;
}

/// Duty cycle clamped to valid range (0..1).
inline double valid_duty_cycle(double d) {
    return clamp(d, 0.01, 0.99);
}

} // namespace converter_common
