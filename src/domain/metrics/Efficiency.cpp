#include "Efficiency.h"
#include <cmath>
#include <string>

namespace efficiency {

double efficiency(double p_out, double total_losses) {
    if ((p_out + total_losses) <= 0.0) return 1.0;
    return p_out / (p_out + total_losses);
}

std::string efficiency_percent(double eff) {
    char buf[16];
    std::snprintf(buf, sizeof(buf), "%.1f%%", eff * 100.0);
    return std::string(buf);
}

double mosfet_power_loss(double i_drain, double r_ds_on, double duty,
                         double v_ds, double t_rise, double t_fall,
                         double frequency)
{
    double d = std::clamp(duty, 0.0, 1.0);
    double i = std::abs(i_drain);
    double p_conduction = i * i * r_ds_on * d;
    double p_switching = (frequency > 0.0)
        ? 0.5 * v_ds * i * (t_rise + t_fall) * frequency
        : 0.0;
    return p_conduction + p_switching;
}

double diode_power_loss(double i_forward, double v_f, double conduction_fraction) {
    return std::abs(i_forward) * v_f * std::clamp(conduction_fraction, 0.0, 1.0);
}

} // namespace efficiency
