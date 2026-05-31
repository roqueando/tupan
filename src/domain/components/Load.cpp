#include "Load.h"
#include <cmath>

namespace load {

double resistive_power(double vout, double load_resistance) {
    if (load_resistance <= 0.0) return 0.0;
    return vout * vout / load_resistance;
}

double resistive_current(double vout, double load_resistance) {
    if (load_resistance <= 0.0) return 0.0;
    return vout / load_resistance;
}

double rl_time_constant(double inductance, double resistance) {
    if (resistance <= 0.0) return 0.0;
    return inductance / resistance;
}

double rc_corner_frequency(double capacitance, double resistance) {
    if (capacitance <= 0.0 || resistance <= 0.0) return 0.0;
    return 1.0 / (2.0 * 3.14159265358979323846 * resistance * capacitance);
}

} // namespace load
