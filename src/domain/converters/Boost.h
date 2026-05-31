#pragma once

#include "domain/Types.h"

namespace boost_converter {

/// Calculate output voltage for a boost converter in CCM.
/// Vout = Vin / (1 - D)
double output_voltage(double vin, double duty);

/// Calculate the required duty cycle to achieve a target output voltage.
/// D = 1 - Vin / Vout
double required_duty_cycle(double vin, double vout_target);

/// Calculate inductor current ripple (peak-to-peak) in CCM.
/// ΔiL = Vin * D / (f * L)
double inductor_current_ripple(double vin, double duty, double frequency, double inductance);

/// Calculate output voltage ripple (peak-to-peak) in CCM.
/// ΔVout = Iout * D / (f * C)
double output_voltage_ripple(double iout, double duty, double frequency, double capacitance);

/// Calculate average output current.
double output_current(double vout, double load_resistance);

/// Calculate input current (average).
/// Iin = Iout / (1 - D)
double input_current(double iout, double duty);

/// Estimate conduction losses.
double conduction_losses(double iin, double iout, double duty, double r_switch, double r_inductor, double v_f);

/// Estimate switching losses.
double switching_losses(double vin, double iin, double frequency, double t_rise, double t_fall);

/// Full analytical calculation for boost converter.
ConverterResults calculate(const ConverterParams& params);

} // namespace boost_converter
