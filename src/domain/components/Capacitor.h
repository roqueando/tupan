#pragma once

namespace capacitor {

/// Calculate the required capacitance for a given output voltage ripple (buck).
double buck_required_capacitance(double il_ripple, double frequency, double vout_ripple);

/// Calculate the required capacitance (boost).
double boost_required_capacitance(double iout, double duty, double frequency, double vout_ripple);

/// Calculate RMS current through the output capacitor.
double capacitor_rms_current(double ripple_current);

/// Calculate capacitor voltage rating recommendation.
double recommended_voltage_rating(double max_voltage, double derating_factor);

} // namespace capacitor
