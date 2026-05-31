#pragma once

namespace load {

/// Calculate output power for a resistive load.
double resistive_power(double vout, double load_resistance);

/// Calculate output current for a resistive load.
double resistive_current(double vout, double load_resistance);

/// Calculate load time constant for RL load.
double rl_time_constant(double inductance, double resistance);

/// Calculate the corner frequency for an RC load.
double rc_corner_frequency(double capacitance, double resistance);

} // namespace load
