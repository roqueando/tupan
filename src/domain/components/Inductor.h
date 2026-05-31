#pragma once

namespace inductor {

/// Calculate the required inductance for a given current ripple (buck).
double buck_required_inductance(double vin, double duty, double frequency, double ripple_current);

/// Calculate the required inductance for a given current ripple (boost).
double boost_required_inductance(double vin, double duty, double frequency, double ripple_current);

/// Calculate the peak current through the inductor.
double peak_current(double i_avg, double ripple_current);

/// Calculate the RMS current through the inductor.
double rms_current(double i_avg, double ripple_current);

/// Calculate energy stored in inductor at peak current.
double stored_energy(double inductance, double peak_current);

} // namespace inductor
