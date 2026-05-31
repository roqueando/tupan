#pragma once

#include <string>

namespace efficiency {

/// Calculate efficiency from output power and total losses.
double efficiency(double p_out, double total_losses);

/// Format efficiency as percentage string.
std::string efficiency_percent(double eff);

/// Calculate power dissipation in a MOSFET.
double mosfet_power_loss(double i_drain, double r_ds_on, double duty,
                         double v_ds, double t_rise, double t_fall,
                         double frequency);

/// Calculate power dissipation in a diode.
double diode_power_loss(double i_forward, double v_f, double conduction_fraction);

} // namespace efficiency
