/// Capacitor design and parameter calculation.

/// Calculate the required capacitance for a given output voltage ripple specification.
///
/// For buck: C = ΔiL / (8 * f * ΔVout)
pub fn buck_required_capacitance(il_ripple: f64, frequency: f64, vout_ripple: f64) -> f64 {
    if frequency <= 0.0 || vout_ripple <= 0.0 {
        return 0.0;
    }
    il_ripple / (8.0 * frequency * vout_ripple)
}

/// Calculate the required capacitance for a boost converter.
///
/// For boost: C = Iout * D / (f * ΔVout)
pub fn boost_required_capacitance(iout: f64, duty: f64, frequency: f64, vout_ripple: f64) -> f64 {
    if frequency <= 0.0 || vout_ripple <= 0.0 {
        return 0.0;
    }
    let d = duty.clamp(0.0, 1.0);
    iout * d / (frequency * vout_ripple)
}

/// Calculate the RMS current through the output capacitor.
///
/// For buck: I_c_rms = ΔiL / sqrt(12)
pub fn capacitor_rms_current(ripple_current: f64) -> f64 {
    ripple_current / (12.0_f64).sqrt()
}

/// Calculate capacitor voltage rating recommendation (derated).
///
/// Recommend voltage rating = V_max * derating_factor (typically 1.5x for ceramic)
pub fn recommended_voltage_rating(max_voltage: f64, derating_factor: f64) -> f64 {
    max_voltage * derating_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buck_required_capacitance() {
        // ΔiL=0.9A, f=100kHz, ΔVout=0.01V
        // C = 0.9 / (8 * 100e3 * 0.01) = 0.9 / 8000 = 112.5 µF
        let c = buck_required_capacitance(0.9, 100_000.0, 0.01);
        assert!((c - 112.5e-6).abs() < 1e-9);
    }

    #[test]
    fn test_capacitor_rms_current() {
        let rms = capacitor_rms_current(0.9);
        assert!((rms - 0.9 / (12.0_f64).sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_recommended_voltage_rating() {
        let rating = recommended_voltage_rating(12.0, 1.5);
        assert!((rating - 18.0).abs() < 1e-6);
    }
}
