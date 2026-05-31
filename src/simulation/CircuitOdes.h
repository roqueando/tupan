#pragma once

#include "domain/Types.h"
#include <vector>

namespace circuit_odes {

/// Buck converter ODE.
/// States: y[0] = iL (inductor current), y[1] = vC (capacitor voltage)
struct BuckOde {
    double vin, l, c, r, frequency, duty;

    static BuckOde from_params(const ConverterParams& params);

    std::vector<double> derivatives(double t, const std::vector<double>& y) const;
};

/// Boost converter ODE.
struct BoostOde {
    double vin, l, c, r, frequency, duty;

    static BoostOde from_params(const ConverterParams& params);

    std::vector<double> derivatives(double t, const std::vector<double>& y) const;
};

/// Single-phase VSI with RL load ODE.
struct VsiOde {
    double vdc, r_load, l_load, carrier_freq, mod_freq, ma;

    static VsiOde from_params(const ConverterParams& params);

    std::vector<double> derivatives(double t, const std::vector<double>& y) const;

private:
    double pwm_voltage(double t) const;
};

} // namespace circuit_odes
