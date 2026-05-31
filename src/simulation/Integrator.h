#pragma once

#include "domain/Types.h"
#include <functional>
#include <vector>

namespace integrator {

/// Derivative function: f(t, y) -> return vector of derivatives.
using DerivFn = std::function<std::vector<double>(double, const std::vector<double>&)>;

/// Perform a single RK4 step.
std::vector<double> rk4_step(const DerivFn& f, double t, const std::vector<double>& y, double dt);

/// Integrate a system of ODEs using RK4 from t_start to t_end.
SimulationResult integrate(
    const DerivFn& f,
    const std::vector<double>& y0,
    double t_start, double t_end,
    double dt,
    size_t sample_every = 1
);

/// Integrate with output decimation to limit total data points.
SimulationResult integrate_fixed(
    const DerivFn& f,
    const std::vector<double>& y0,
    double t_start, double t_end,
    double dt,
    size_t max_points = 5000
);

} // namespace integrator
