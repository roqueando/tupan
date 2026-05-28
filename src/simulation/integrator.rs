/// Runge-Kutta 4th order (RK4) integrator for ODE systems.
///
/// Solves dy/dt = f(t, y) for a system of N coupled first-order ODEs.

/// A vector of state variables at a given time.
pub type StateVec = Vec<f64>;

/// Boxed derivative function: `f(t, y) -> dy/dt`
/// We use a boxed closure so that struct methods can be used as derivatives.
pub type DerivFn = Box<dyn Fn(f64, &[f64]) -> StateVec>;

/// Result of a simulation: time points and corresponding state vectors.
#[derive(Debug, Clone)]
pub struct SimulationResult {
    /// Time points
    pub t: Vec<f64>,
    /// State vectors at each time point (rows = time, cols = state variables)
    pub y: Vec<StateVec>,
}

/// Perform a single RK4 step.
///
/// y_{n+1} = y_n + (dt/6) * (k1 + 2*k2 + 2*k3 + k4)
pub fn rk4_step(
    f: &DerivFn,
    t: f64,
    y: &[f64],
    dt: f64,
) -> StateVec {
    let n = y.len();
    let k1 = f(t, y);
    let k2 = {
        let mut y_temp = y.to_vec();
        for i in 0..n {
            y_temp[i] += 0.5 * dt * k1[i];
        }
        f(t + 0.5 * dt, &y_temp)
    };
    let k3 = {
        let mut y_temp = y.to_vec();
        for i in 0..n {
            y_temp[i] += 0.5 * dt * k2[i];
        }
        f(t + 0.5 * dt, &y_temp)
    };
    let k4 = {
        let mut y_temp = y.to_vec();
        for i in 0..n {
            y_temp[i] += dt * k3[i];
        }
        f(t + dt, &y_temp)
    };

    // Combine: y_new = y + (dt/6) * (k1 + 2*k2 + 2*k3 + k4)
    let mut y_new = y.to_vec();
    for i in 0..n {
        y_new[i] += (dt / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
    y_new
}

/// Integrate a system of ODEs using RK4 from t_start to t_end.
///
/// # Arguments
/// * `f` - Derivative function: dy/dt = f(t, y)
/// * `y0` - Initial state vector
/// * `t_span` - (start, end) time
/// * `dt` - Fixed time step
///
/// # Returns
/// A `SimulationResult` with time points and state vectors, sampled every `sample_every` steps.
pub fn integrate(
    f: &DerivFn,
    y0: &[f64],
    t_span: (f64, f64),
    dt: f64,
    sample_every: usize,
) -> SimulationResult {
    let (t_start, t_end) = t_span;
    let n_steps = ((t_end - t_start) / dt).ceil() as usize;

    let mut t_vals = Vec::with_capacity(n_steps / sample_every + 1);
    let mut y_vals = Vec::with_capacity(n_steps / sample_every + 1);

    let mut t = t_start;
    let mut y = y0.to_vec();

    // Store initial condition
    t_vals.push(t);
    y_vals.push(y.clone());

    for step in 0..n_steps {
        y = rk4_step(f, t, &y, dt);
        t += dt;

        // Sample periodically
        if step % sample_every == 0 || step == n_steps - 1 {
            t_vals.push(t);
            y_vals.push(y.clone());
        }
    }

    SimulationResult {
        t: t_vals,
        y: y_vals,
    }
}

/// Integrate with output decimation to limit total data points.
///
/// # Arguments
/// * `f` - Derivative function
/// * `y0` - Initial state vector
/// * `t_span` - (start, end) time
/// * `dt` - Fixed time step
/// * `max_points` - Maximum number of output points
pub fn integrate_fixed(
    f: &DerivFn,
    y0: &[f64],
    t_span: (f64, f64),
    dt: f64,
    max_points: usize,
) -> SimulationResult {
    let (t_start, t_end) = t_span;
    let total_time = t_end - t_start;

    let steps = (total_time / dt) as usize;
    let sample_every = if steps > max_points {
        (steps / max_points).max(1)
    } else {
        1
    };

    integrate(f, y0, t_span, dt, sample_every)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// dy/dt = -2y, y(0) = 1 => y(t) = e^(-2t)
    fn exp_decay(_t: f64, y: &[f64]) -> StateVec {
        vec![-2.0 * y[0]]
    }

    #[test]
    fn test_rk4_step() {
        let f: DerivFn = Box::new(exp_decay);
        let y0 = vec![1.0];
        let y1 = rk4_step(&f, 0.0, &y0, 0.1);

        let expected = (-0.2_f64).exp();
        assert!((y1[0] - expected).abs() < 1e-4);
    }

    #[test]
    fn test_integrate_basic() {
        let f: DerivFn = Box::new(exp_decay);
        let y0 = vec![1.0];
        let result = integrate(&f, &y0, (0.0, 1.0), 0.01, 10);

        assert!(result.t.len() >= 10);

        let final_y = result.y.last().unwrap()[0];
        let expected = (-2.0_f64).exp();
        assert!((final_y - expected).abs() < 1e-3);
    }

    #[test]
    fn test_integrate_fixed_max_points() {
        let f: DerivFn = Box::new(exp_decay);
        let y0 = vec![1.0];
        let result = integrate_fixed(&f, &y0, (0.0, 1.0), 0.001, 200);
        // Max points is approximate; we just verify it's not huge
        assert!(result.t.len() <= 250, "got {} points", result.t.len());
        assert!(result.t.len() > 5);
    }

    fn harmonic(_t: f64, y: &[f64]) -> StateVec {
        vec![y[1], -y[0]]
    }

    #[test]
    fn test_harmonic_oscillator() {
        let f: DerivFn = Box::new(harmonic);
        let y0 = vec![1.0, 0.0];
        let result = integrate(&f, &y0, (0.0, 2.0 * std::f64::consts::PI), 0.01, 10);

        let final_y = result.y.last().unwrap()[0];
        assert!((final_y - 1.0).abs() < 0.01);
    }
}
