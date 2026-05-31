#include "Integrator.h"

namespace integrator {

std::vector<double> rk4_step(const DerivFn& f, double t, const std::vector<double>& y, double dt) {
    size_t n = y.size();

    auto k1 = f(t, y);

    std::vector<double> y_temp(n);
    for (size_t i = 0; i < n; ++i)
        y_temp[i] = y[i] + 0.5 * dt * k1[i];
    auto k2 = f(t + 0.5 * dt, y_temp);

    for (size_t i = 0; i < n; ++i)
        y_temp[i] = y[i] + 0.5 * dt * k2[i];
    auto k3 = f(t + 0.5 * dt, y_temp);

    for (size_t i = 0; i < n; ++i)
        y_temp[i] = y[i] + dt * k3[i];
    auto k4 = f(t + dt, y_temp);

    std::vector<double> y_new(n);
    for (size_t i = 0; i < n; ++i) {
        y_new[i] = y[i] + (dt / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
    return y_new;
}

SimulationResult integrate(
    const DerivFn& f,
    const std::vector<double>& y0,
    double t_start, double t_end,
    double dt,
    size_t sample_every)
{
    size_t n_steps = static_cast<size_t>(std::ceil((t_end - t_start) / dt));

    SimulationResult result;
    result.t.reserve(n_steps / sample_every + 1);
    result.y.reserve(n_steps / sample_every + 1);

    double t = t_start;
    std::vector<double> y = y0;

    result.t.push_back(t);
    result.y.push_back(y);

    for (size_t step = 0; step < n_steps; ++step) {
        y = rk4_step(f, t, y, dt);
        t += dt;

        if (step % sample_every == 0 || step == n_steps - 1) {
            result.t.push_back(t);
            result.y.push_back(y);
        }
    }

    return result;
}

SimulationResult integrate_fixed(
    const DerivFn& f,
    const std::vector<double>& y0,
    double t_start, double t_end,
    double dt,
    size_t max_points)
{
    double total_time = t_end - t_start;
    size_t steps = static_cast<size_t>(total_time / dt);
    size_t sample_every = (steps > max_points) ? (steps / max_points) : 1;
    return integrate(f, y0, t_start, t_end, dt, sample_every);
}

} // namespace integrator
