#pragma once

#include "domain/Types.h"

namespace vsi_single {

/// Calculate the fundamental output voltage amplitude.
/// V1 = ma * Vdc  (full-bridge)
/// V1 = ma * Vdc / 2 (half-bridge)
double fundamental_output(double ma, double vdc, bool full_bridge);

/// Calculate RMS output voltage.
double rms_output(double ma, double vdc, bool full_bridge);

/// Calculate output current (resistive load).
double output_current(double vout_rms, double load_resistance);

/// Calculate input current (average DC side).
double input_current(double vout_rms, double iout_rms, double vdc);

/// Estimate conduction losses for a VSI.
double conduction_losses(double iout_rms, double r_switch, double v_f, double num_switches_conduction);

/// Estimate switching losses for a VSI.
double switching_losses(double vdc, double iout, double switching_freq, double t_rise, double t_fall, double num_switches);

/// Full analytical calculation for single-phase VSI.
ConverterResults calculate(const ConverterParams& params, bool full_bridge = true);

} // namespace vsi_single
