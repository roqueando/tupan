#include "Ripple.h"
#include <algorithm>
#include <cmath>

namespace ripple {

double buck_critical_inductance(double duty, double load_resistance, double frequency) {
    if (frequency <= 0.0) return 0.0;
    double d = std::clamp(duty, 0.0, 1.0);
    return (1.0 - d) * load_resistance / (2.0 * frequency);
}

double boost_critical_inductance(double duty, double load_resistance, double frequency) {
    if (frequency <= 0.0) return 0.0;
    double d = std::clamp(duty, 0.0, 1.0);
    return d * std::pow(1.0 - d, 2) * load_resistance / (2.0 * frequency);
}

double buck_min_capacitance(double il_ripple, double frequency, double vout_ripple_req) {
    if (frequency <= 0.0 || vout_ripple_req <= 0.0) return 0.0;
    return il_ripple / (8.0 * frequency * vout_ripple_req);
}

double boost_min_capacitance(double iout, double duty, double frequency, double vout_ripple_req) {
    if (frequency <= 0.0 || vout_ripple_req <= 0.0) return 0.0;
    double d = std::clamp(duty, 0.0, 1.0);
    return iout * d / (frequency * vout_ripple_req);
}

} // namespace ripple
