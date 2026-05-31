#include "Capacitor.h"
#include <algorithm>
#include <cmath>

namespace capacitor {

double buck_required_capacitance(double il_ripple, double frequency, double vout_ripple) {
    if (frequency <= 0.0 || vout_ripple <= 0.0) return 0.0;
    return il_ripple / (8.0 * frequency * vout_ripple);
}

double boost_required_capacitance(double iout, double duty, double frequency, double vout_ripple) {
    if (frequency <= 0.0 || vout_ripple <= 0.0) return 0.0;
    double d = std::clamp(duty, 0.0, 1.0);
    return iout * d / (frequency * vout_ripple);
}

double capacitor_rms_current(double ripple_current) {
    return ripple_current / std::sqrt(12.0);
}

double recommended_voltage_rating(double max_voltage, double derating_factor) {
    return max_voltage * derating_factor;
}

} // namespace capacitor
