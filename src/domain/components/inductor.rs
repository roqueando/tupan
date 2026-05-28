/// Inductor design and parameter calculation.

/// Calculate the required inductance for a given current ripple specification.
///
/// For buck: L = Vin * D * (1-D) / (f * ΔiL)
pub fn buck_required_inductance(vin: f64, duty: f64, frequency: f64, ripple_current: f64) -> f64 {
    if frequency <= 0.0 || ripple_current <= 0.0 {
        return 0.0;
    }
    let d = duty.clamp(0.0, 1.0);
    vin * d * (1.0 - d) / (frequency * ripple_current)
}

/// Calculate the required inductance for a boost converter.
///
/// For boost: L = Vin * D / (f * ΔiL)
pub fn boost_required_inductance(vin: f64, duty: f64, frequency: f64, ripple_current: f64) -> f64 {
    if frequency <= 0.0 || ripple_current <= 0.0 {
        return 0.0;
    }
    let d = duty.clamp(0.0, 1.0);
    vin * d / (frequency * ripple_current)
}

/// Calculate the peak current through the inductor (for saturation check).
///
/// I_peak = I_avg + ΔiL / 2
pub fn peak_current(i_avg: f64, ripple_current: f64) -> f64 {
    i_avg.abs() + ripple_current / 2.0
}

/// Calculate the RMS current through the inductor.
///
/// I_rms = sqrt(I_avg² + (ΔiL² / 12))
pub fn rms_current(i_avg: f64, ripple_current: f64) -> f64 {
    let i_avg = i_avg.abs();
    let ripple_sq = ripple_current * ripple_current;
    (i_avg * i_avg + ripple_sq / 12.0).sqrt()
}

/// Calculate energy stored in inductor at peak current.
///
/// E = 0.5 * L * I_peak²
pub fn stored_energy(inductance: f64, peak_current: f64) -> f64 {
    0.5 * inductance * peak_current * peak_current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buck_required_inductance() {
        // Vin=48V, D=0.25, f=100kHz, ΔiL=0.5A
        // L = 48 * 0.25 * 0.75 / (100e3 * 0.5) = 9 / 50000 = 180 µH
        let l = buck_required_inductance(48.0, 0.25, 100_000.0, 0.5);
        assert!((l - 180e-6).abs() < 1e-9);
    }

    #[test]
    fn test_peak_current() {
        let peak = peak_current(1.0, 0.5);
        assert!((peak - 1.25).abs() < 1e-6);
    }

    #[test]
    fn test_rms_current() {
        // I_avg=1.0, ΔiL=0.5
        // I_rms = sqrt(1 + 0.25/12) = sqrt(1.020833) ≈ 1.01036
        let rms = rms_current(1.0, 0.5);
        assert!((rms - 1.01036).abs() < 0.001);
    }
}
