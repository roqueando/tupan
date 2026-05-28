/// Ripple calculation utilities shared across converter types.

/// Calculate the critical inductance for CCM boundary in a buck converter.
///
/// L_crit = (1 - D) * R / (2 * f)
pub fn buck_critical_inductance(duty: f64, load_resistance: f64, frequency: f64) -> f64 {
    if frequency <= 0.0 {
        return 0.0;
    }
    let d = duty.clamp(0.0, 1.0);
    (1.0 - d) * load_resistance / (2.0 * frequency)
}

/// Calculate the critical inductance for CCM boundary in a boost converter.
///
/// L_crit = D * (1 - D)² * R / (2 * f)
pub fn boost_critical_inductance(duty: f64, load_resistance: f64, frequency: f64) -> f64 {
    if frequency <= 0.0 {
        return 0.0;
    }
    let d = duty.clamp(0.0, 1.0);
    d * (1.0 - d).powi(2) * load_resistance / (2.0 * frequency)
}

/// Calculate the critical capacitance for a given output voltage ripple requirement.
///
/// For buck: C_min = ΔiL / (8 * f * ΔVout_req)
pub fn buck_min_capacitance(il_ripple: f64, frequency: f64, vout_ripple_req: f64) -> f64 {
    if frequency <= 0.0 || vout_ripple_req <= 0.0 {
        return 0.0;
    }
    il_ripple / (8.0 * frequency * vout_ripple_req)
}

/// Calculate the critical capacitance for a boost converter.
///
/// For boost: C_min = Iout * D / (f * ΔVout_req)
pub fn boost_min_capacitance(
    iout: f64,
    duty: f64,
    frequency: f64,
    vout_ripple_req: f64,
) -> f64 {
    if frequency <= 0.0 || vout_ripple_req <= 0.0 {
        return 0.0;
    }
    let d = duty.clamp(0.0, 1.0);
    iout * d / (frequency * vout_ripple_req)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buck_critical_inductance() {
        // D=0.25, R=10Ω, f=100kHz
        // L_crit = (1-0.25) * 10 / (2 * 100e3) = 0.75 * 10 / 200000 = 37.5 µH
        let l = buck_critical_inductance(0.25, 10.0, 100_000.0);
        assert!((l - 37.5e-6).abs() < 1e-9);
    }

    #[test]
    fn test_boost_critical_inductance() {
        // D=0.5, R=10Ω, f=100kHz
        // L_crit = 0.5 * (0.5)² * 10 / (2 * 100e3) = 0.5 * 0.25 * 10 / 200000 = 6.25 µH
        let l = boost_critical_inductance(0.5, 10.0, 100_000.0);
        assert!((l - 6.25e-6).abs() < 1e-9);
    }

    #[test]
    fn test_buck_min_capacitance() {
        // ΔiL=0.9A, f=100kHz, ΔVout_req=0.01V
        // C = 0.9 / (8 * 100e3 * 0.01) = 0.9 / 8000 = 112.5 uF
        let c = buck_min_capacitance(0.9, 100_000.0, 0.01);
        assert!((c - 112.5e-6).abs() < 1e-9);
    }
}
