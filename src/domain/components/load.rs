/// Load models for converter analysis.

/// Calculate output power for a resistive load.
pub fn resistive_power(vout: f64, load_resistance: f64) -> f64 {
    if load_resistance <= 0.0 {
        return 0.0;
    }
    vout * vout / load_resistance
}

/// Calculate output current for a resistive load.
pub fn resistive_current(vout: f64, load_resistance: f64) -> f64 {
    if load_resistance <= 0.0 {
        return 0.0;
    }
    vout / load_resistance
}

/// Calculate load time constant for RL load.
pub fn rl_time_constant(inductance: f64, resistance: f64) -> f64 {
    if resistance <= 0.0 {
        return 0.0;
    }
    inductance / resistance
}

/// Calculate the corner frequency for an RC load.
pub fn rc_corner_frequency(capacitance: f64, resistance: f64) -> f64 {
    if capacitance <= 0.0 || resistance <= 0.0 {
        return 0.0;
    }
    1.0 / (2.0 * std::f64::consts::PI * resistance * capacitance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resistive_power() {
        let p = resistive_power(12.0, 10.0);
        assert!((p - 14.4).abs() < 1e-6);
    }

    #[test]
    fn test_rl_time_constant() {
        let tau = rl_time_constant(100e-6, 10.0);
        assert!((tau - 10e-6).abs() < 1e-9);
    }
}
