/// Total Harmonic Distortion (THD) calculation for inverter outputs.

/// Calculate THD from harmonic amplitudes.
///
/// THD = sqrt(Σ(Vh²)) / V1  (for h >= 2)
pub fn thd_from_harmonics(harmonics: &[f64], fundamental: f64) -> f64 {
    if fundamental.abs() <= 1e-12 {
        return 0.0;
    }
    let sum_sq: f64 = harmonics.iter().map(|h| h * h).sum();
    sum_sq.sqrt() / fundamental.abs()
}

/// Theoretical THD for a bipolar PWM sine wave with given modulation index.
///
/// Simplified model based on double Fourier series:
/// For a sine-triangle PWM with natural sampling:
/// V_an = Vdc/2 * ma * sin(ωo*t) + harmonics at m*ωc ± n*ωo
///
/// This provides an approximate THD based on the modulation index.
/// For ma = 0.8, THD ≈ 85-105% typically for bipolar PWM.
pub fn pwm_thd_approximate(modulation_index: f64, is_bipolar: bool) -> f64 {
    let ma = modulation_index.clamp(0.0, 1.0);

    if is_bipolar {
        // Bipolar PWM: higher THD at low modulation index
        // Approximate formula based on empirical data
        // THD ≈ sqrt( (1.12 / ma)² - 1 ) — rough approximation
        if ma < 0.01 {
            return 10.0; // very high THD at very low modulation
        }
        let thd_sq = (1.12 / ma).powi(2) - 1.0;
        thd_sq.sqrt().min(10.0)
    } else {
        // Unipolar PWM (future): lower THD
        if ma < 0.01 {
            return 5.0;
        }
        let thd_sq = (0.6 / ma).powi(2) - 1.0;
        thd_sq.sqrt().min(5.0)
    }
}

/// Calculate the fundamental component amplitude for a PWM inverter.
///
/// For bipolar sine-triangle PWM:
/// V1_fundamental = ma * Vdc / 2  (for single-phase half-bridge)
/// V1_fundamental = ma * Vdc       (for single-phase full-bridge)
pub fn fundamental_amplitude(modulation_index: f64, vdc: f64, is_full_bridge: bool) -> f64 {
    let ma = modulation_index.clamp(0.0, 1.0);
    if is_full_bridge {
        ma * vdc
    } else {
        ma * vdc / 2.0
    }
}

/// Calculate RMS output voltage for a PWM inverter (fundamental + harmonics).
///
/// Simplified: Vrms ≈ Vdc * sqrt(ma² / 2 + ...) — for sine PWM
/// First-order: Vrms_fundamental = V1_fundamental / sqrt(2)
pub fn rms_output_voltage(modulation_index: f64, vdc: f64, is_full_bridge: bool) -> f64 {
    let v1 = fundamental_amplitude(modulation_index, vdc, is_full_bridge);
    v1 / std::f64::consts::SQRT_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thd_from_harmonics() {
        // Fundamental=100V, harmonics: 3rd=20V, 5th=10V, 7th=5V
        // THD = sqrt(400+100+25)/100 = sqrt(525)/100 ≈ 0.229
        let harmonics = vec![20.0, 10.0, 5.0];
        let thd = thd_from_harmonics(&harmonics, 100.0);
        assert!((thd - 0.2291).abs() < 0.001);
    }

    #[test]
    fn test_fundamental_amplitude() {
        // ma=0.8, Vdc=300V, full-bridge → V1 = 0.8 * 300 = 240V
        let v1 = fundamental_amplitude(0.8, 300.0, true);
        assert!((v1 - 240.0).abs() < 1e-6);

        // half-bridge → V1 = 0.8 * 300 / 2 = 120V
        let v1_half = fundamental_amplitude(0.8, 300.0, false);
        assert!((v1_half - 120.0).abs() < 1e-6);
    }

    #[test]
    fn test_rms_output_voltage() {
        // ma=0.8, Vdc=300V, full-bridge
        // V1 = 240V, Vrms = 240 / sqrt(2) ≈ 169.7V
        let vrms = rms_output_voltage(0.8, 300.0, true);
        assert!((vrms - 240.0 / std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_pwm_thd_approximate() {
        let thd = pwm_thd_approximate(0.8, true);
        assert!(thd > 0.5); // THD should be significant for bipolar PWM
        assert!(thd < 10.0); // But not absurd
    }
}
