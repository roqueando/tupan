use crate::domain::converters::common::valid_duty_cycle;
use crate::domain::{ConverterParams, ConverterResults};

/// Calculate output voltage for a boost converter in CCM.
///
/// Vout = Vin / (1 - D)
pub fn output_voltage(vin: f64, duty: f64) -> f64 {
    let d = valid_duty_cycle(duty);
    vin / (1.0 - d)
}

/// Calculate the required duty cycle to achieve a target output voltage.
///
/// D = 1 - Vin / Vout
pub fn required_duty_cycle(vin: f64, vout_target: f64) -> f64 {
    if vin <= 0.0 || vout_target <= vin {
        return 0.01; // minimum duty
    }
    let d = 1.0 - vin / vout_target;
    valid_duty_cycle(d)
}

/// Calculate inductor current ripple (peak-to-peak) in CCM.
///
/// ΔiL = Vin * D / (f * L)
/// (during on-time, voltage across inductor = Vin)
pub fn inductor_current_ripple(vin: f64, duty: f64, frequency: f64, inductance: f64) -> f64 {
    let d = valid_duty_cycle(duty);
    if inductance <= 0.0 || frequency <= 0.0 {
        return 0.0;
    }
    (vin * d) / (frequency * inductance)
}

/// Calculate output voltage ripple (peak-to-peak) in CCM.
///
/// ΔVout = Iout * D / (f * C)
pub fn output_voltage_ripple(iout: f64, duty: f64, frequency: f64, capacitance: f64) -> f64 {
    let d = valid_duty_cycle(duty);
    if frequency <= 0.0 || capacitance <= 0.0 {
        return 0.0;
    }
    (iout * d) / (frequency * capacitance)
}

/// Calculate average output current.
pub fn output_current(vout: f64, load_resistance: f64) -> f64 {
    if load_resistance <= 0.0 {
        return 0.0;
    }
    vout / load_resistance
}

/// Calculate input current (average).
///
/// For an ideal boost: Pin = Pout, so Iin * Vin = Vout * Iout
/// Iin = Vout * Iout / Vin = Iout / (1 - D)
pub fn input_current(iout: f64, duty: f64) -> f64 {
    let d = valid_duty_cycle(duty);
    iout / (1.0 - d)
}

/// Estimate conduction losses for a boost converter.
///
/// P_cond = I² * R_switch * D + I² * R_L + I * Vf (diode conducts during off-time only)
/// For boost: diode conducts during (1-D) of the time, carrying the output/input current.
/// Simplified: I_diode ≈ Iout
pub fn conduction_losses(
    iin: f64,
    iout: f64,
    duty: f64,
    r_switch: f64,
    r_inductor: f64,
    v_f: f64,
) -> f64 {
    let d = valid_duty_cycle(duty);
    let iin_abs = iin.abs();
    let iout_abs = iout.abs();
    let p_switch = iin_abs * iin_abs * r_switch * d;
    let p_inductor = iin_abs * iin_abs * r_inductor;
    let p_diode = iout_abs * v_f;
    p_switch + p_inductor + p_diode
}

/// Estimate switching losses for a boost converter.
///
/// P_sw = Vin * Iin * (t_rise + t_fall) * f / 2
/// Note: during switching, switch voltage is Vout (Vin + induced voltage),
/// but simplified to Vin for conservative estimate.
pub fn switching_losses(vin: f64, iin: f64, frequency: f64, t_rise: f64, t_fall: f64) -> f64 {
    if frequency <= 0.0 {
        return 0.0;
    }
    vin * iin.abs() * (t_rise + t_fall) * frequency * 0.5
}

/// Full analytical calculation for boost converter.
pub fn calculate(params: &ConverterParams) -> ConverterResults {
    let duty = required_duty_cycle(params.vin, params.vout_target);
    let vout = output_voltage(params.vin, duty);
    let iout = output_current(vout, params.load_resistance);
    let iin = input_current(iout, duty);
    let il_ripple = inductor_current_ripple(params.vin, duty, params.frequency, params.inductance);
    let vout_ripple = output_voltage_ripple(iout, duty, params.frequency, params.capacitance);

    // Typical values for loss estimation
    let r_switch = 0.1;    // 100 mOhm
    let r_inductor = 0.05; // 50 mOhm
    let v_f = 0.7;          // Schottky
    let t_rise = 20e-9;
    let t_fall = 20e-9;

    let conduction_losses = conduction_losses(iin, iout, duty, r_switch, r_inductor, v_f);
    let switching_losses = switching_losses(params.vin, iin, params.frequency, t_rise, t_fall);

    let total_losses = conduction_losses + switching_losses;
    let p_out = vout * iout;
    let efficiency = if (p_out + total_losses) > 0.0 {
        p_out / (p_out + total_losses)
    } else {
        1.0
    };

    ConverterResults {
        vout,
        iout,
        iin,
        vout_ripple,
        il_ripple,
        conduction_losses,
        switching_losses,
        efficiency,
        thd: None,
        rms_output: None,
        fundamental_amplitude: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_voltage() {
        // Boost: Vin=12V, D=0.5 → Vout=12/(1-0.5)=24V
        let vout = output_voltage(12.0, 0.5);
        assert!((vout - 24.0).abs() < 1e-6);
    }

    #[test]
    fn test_output_voltage_higher_duty() {
        // Boost: Vin=12V, D=0.75 → Vout=12/(1-0.75)=48V
        let vout = output_voltage(12.0, 0.75);
        assert!((vout - 48.0).abs() < 1e-6);
    }

    #[test]
    fn test_required_duty_cycle() {
        // Vin=12V, Vout=24V → D=1-12/24=0.5
        let d = required_duty_cycle(12.0, 24.0);
        assert!((d - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_required_duty_cycle_vout_less_than_vin() {
        // Vout <= Vin → minimum duty
        let d = required_duty_cycle(24.0, 12.0);
        assert!((d - 0.01).abs() < 1e-6);
    }

    #[test]
    fn test_inductor_current_ripple() {
        // Boost: Vin=12V, D=0.5, f=100kHz, L=100µH
        // ΔiL = 12 * 0.5 / (100e3 * 100e-6) = 6 / 10 = 0.6 A
        let ripple = inductor_current_ripple(12.0, 0.5, 100_000.0, 100e-6);
        assert!((ripple - 0.6).abs() < 1e-6);
    }

    #[test]
    fn test_output_voltage_ripple() {
        // ΔVout = Iout * D / (f * C)
        // Iout = 24/10 = 2.4A, D=0.5, f=100kHz, C=100µF
        // ΔVout = 2.4 * 0.5 / (100e3 * 100e-6) = 1.2 / 10 = 0.12 V
        let ripple = output_voltage_ripple(2.4, 0.5, 100_000.0, 100e-6);
        assert!((ripple - 0.12).abs() < 1e-6);
    }

    #[test]
    fn test_full_calculation() {
        let params = ConverterParams {
            vin: 12.0,
            vout_target: 24.0,
            frequency: 100_000.0,
            duty_cycle: 0.5,
            inductance: 100e-6,
            capacitance: 100e-6,
            load_resistance: 10.0,
            ..Default::default()
        };

        let result = calculate(&params);

        // Vout should be about 24V
        assert!((result.vout - 24.0).abs() < 0.01);

        // Iout = 24V / 10Ω = 2.4A
        assert!((result.iout - 2.4).abs() < 0.01);

        // Iin from power balance: 24*2.4/12 = 4.8A
        assert!((result.iin - 4.8).abs() < 0.01);

        // Efficiency > 80%
        assert!(result.efficiency > 0.8);
    }

    #[test]
    fn test_input_current() {
        // Boost: Iout=2.4A, D=0.5 → Iin=2.4/(1-0.5)=4.8A
        let iin = input_current(2.4, 0.5);
        assert!((iin - 4.8).abs() < 1e-6);
    }
}
