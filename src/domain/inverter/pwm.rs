/// PWM (Pulse Width Modulation) generation and analysis.

use crate::domain::converters::common::angular_frequency;

/// Generate a sine-triangle PWM switching function.
///
/// For a given modulation index (ma) and carrier ratio (mf), generates
/// the PWM switching pattern as a list of (time, value) pairs.
///
/// Returns a Vec of (time, switching_state) where switching_state is 1 for high, 0 for low.
pub fn generate_pwm(
    ma: f64,
    modulation_freq: f64,
    carrier_freq: f64,
    num_periods: f64,
    dt: f64,
) -> Vec<(f64, f64)> {
    let ma = ma.clamp(0.0, 1.0);
    let t_total = num_periods / modulation_freq;
    let n_points = (t_total / dt) as usize;
    let omega_m = angular_frequency(modulation_freq);
    let omega_c = angular_frequency(carrier_freq);

    let mut samples = Vec::with_capacity(n_points);

    for i in 0..n_points {
        let t = i as f64 * dt;
        if t > t_total {
            break;
        }
        // Modulating sine wave
        let v_mod = ma * (omega_m * t).sin();
        // Carrier triangle wave (bipolar)
        let phase_c = (omega_c * t) % (2.0 * std::f64::consts::PI);
        let triangle = if phase_c < std::f64::consts::PI {
            // Rising: 0 to 1
            phase_c / std::f64::consts::PI * 2.0 - 1.0
        } else {
            // Falling: 1 to -1
            let fall_phase = phase_c - std::f64::consts::PI;
            1.0 - fall_phase / std::f64::consts::PI * 2.0
        };

        let state = if v_mod >= triangle { 1.0 } else { -1.0 };
        samples.push((t, state));
    }

    samples
}

/// Calculate the duty cycle for a given reference angle in sine PWM.
///
/// d(t) = 0.5 * (1 + ma * sin(ωm * t))
pub fn duty_cycle_at_time(ma: f64, omega_m: f64, t: f64) -> f64 {
    (0.5 * (1.0 + ma * (omega_m * t).sin())).clamp(0.0, 1.0)
}

/// Calculate the average switching frequency (effective switching events per second).
pub fn average_switching_frequency(carrier_freq: f64) -> f64 {
    carrier_freq
}

/// Calculate the frequency modulation ratio.
///
/// mf = f_carrier / f_modulation
pub fn frequency_modulation_ratio(carrier_freq: f64, modulation_freq: f64) -> f64 {
    if modulation_freq <= 0.0 {
        return 0.0;
    }
    carrier_freq / modulation_freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pwm_basic() {
        let samples = generate_pwm(0.8, 60.0, 10000.0, 1.0, 1e-5);
        assert!(!samples.is_empty());
        // Samples should be either 1.0 or -1.0
        for (_, state) in &samples {
            assert!((*state - 1.0).abs() < 1e-6 || (*state + 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_duty_cycle_at_time() {
        // At t=0, sin(0)=0 → d=0.5
        let d = duty_cycle_at_time(0.8, 2.0 * std::f64::consts::PI * 60.0, 0.0);
        assert!((d - 0.5).abs() < 1e-6);

        // At t = π/(2ω), sin(π/2)=1 → d = 0.5*(1+ma) = 0.9
        let omega = 2.0 * std::f64::consts::PI * 60.0;
        let t = std::f64::consts::PI / (2.0 * omega);
        let d = duty_cycle_at_time(0.8, omega, t);
        assert!((d - 0.9).abs() < 1e-5); // relaxed tolerance for fp precision
    }

    #[test]
    fn test_frequency_modulation_ratio() {
        let mf = frequency_modulation_ratio(10000.0, 60.0);
        assert!((mf - 10000.0 / 60.0).abs() < 1e-6);
    }
}
