#pragma once

namespace ripple {

/// Calculate the critical inductance for CCM boundary in a buck converter.
double buck_critical_inductance(double duty, double load_resistance, double frequency);

/// Calculate the critical inductance for CCM boundary in a boost converter.
double boost_critical_inductance(double duty, double load_resistance, double frequency);

/// Calculate the minimum capacitance for buck voltage ripple requirement.
double buck_min_capacitance(double il_ripple, double frequency, double vout_ripple_req);

/// Calculate the minimum capacitance for boost voltage ripple requirement.
double boost_min_capacitance(double iout, double duty, double frequency, double vout_ripple_req);

} // namespace ripple
