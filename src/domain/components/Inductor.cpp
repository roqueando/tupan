#include "Inductor.h"
#include <algorithm>
#include <cmath>

namespace inductor {

double buck_required_inductance(double vin, double duty, double frequency, double ripple_current) {
    if (frequency <= 0.0 || ripple_current <= 0.0) return 0.0;
    double d = std::clamp(duty, 0.0, 1.0);
    return vin * d * (1.0 - d) / (frequency * ripple_current);
}

double boost_required_inductance(double vin, double duty, double frequency, double ripple_current) {
    if (frequency <= 0.0 || ripple_current <= 0.0) return 0.0;
    double d = std::clamp(duty, 0.0, 1.0);
    return vin * d / (frequency * ripple_current);
}

double peak_current(double i_avg, double ripple_current) {
    return std::abs(i_avg) + ripple_current / 2.0;
}

double rms_current(double i_avg, double ripple_current) {
    double ia = std::abs(i_avg);
    double rip_sq = ripple_current * ripple_current;
    return std::sqrt(ia * ia + rip_sq / 12.0);
}

double stored_energy(double inductance, double peak_current) {
    return 0.5 * inductance * peak_current * peak_current;
}

} // namespace inductor
