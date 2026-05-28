use crate::domain::converters::common::{valid_duty_cycle, switching_period};
use crate::domain::{ConverterParams, ConverterResults};

/// Calculate output voltage for a buck converter in CCM.
///
/// Vout = Vin * D
pub fn output_voltage(vin: f64, duty: f64) -> f64 {
    vin * valid_duty_cycle(duty)
}

/// Calculate the required duty cycle to achieve a target output voltage.
///
/// D = Vout / Vin
pub fn required_duty_cycle(vin: f64, vout_target: f64) -> f64 {
    if vin <= 0.0 {
        return 0.0;
    }
    valid_duty_cycle(vout_target / vin)
}

/// Calculate inductor current ripple (peak-to-peak) in CCM.
///
/// ΔiL = V_L * D * T / L
/// where V_L = Vin - Vout during on-time
///
/// Simplified: ΔiL = Vin * D * (1 - D) / (f * L)
/// (assuming Vout = Vin*D, which gives V_L = Vin - Vin*D = Vin*(1-D) during on-time)
pub fn inductor_current_ripple(vin: f64, duty: f64, frequency: f64, inductance: f64) -> f64 {
    let d = valid_duty_cycle(duty);
    let t = switching_period(frequency);
    let v_l = vin * (1.0 - d); // voltage across inductor during on-time
    if inductance <= 0.0 || frequency <= 0.0 {
        return 0.0;
    }
    (v_l * d * t) / inductance
}

/// Calculate output voltage ripple (peak-to-peak) in CCM.
///
/// ΔVout = ΔiL / (8 * f * C)
pub fn output_voltage_ripple(il_ripple: f64, frequency: f64, capacitance: f64) -> f64 {
    if frequency <= 0.0 || capacitance <= 0.0 {
        return 0.0;
    }
    il_ripple / (8.0 * frequency * capacitance)
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
/// Iin = Iout * D
pub fn input_current(iout: f64, duty: f64) -> f64 {
    iout * valid_duty_cycle(duty)
}

/// Estimate conduction losses for a buck converter.
///
/// Simplified model including switch R_ds(on), inductor DCR, diode forward drop.
///
/// P_cond = I² * R_switch * D + I² * R_L + I * Vf * (1-D)
pub fn conduction_losses(
    iout: f64,
    duty: f64,
    r_switch: f64,  // R_ds(on) of MOSFET
    r_inductor: f64, // inductor DCR
    v_f: f64,       // diode forward voltage
) -> f64 {
    let d = valid_duty_cycle(duty);
    let i = iout.abs();
    let p_switch = i * i * r_switch * d;
    let p_inductor = i * i * r_inductor;
    let p_diode = i * v_f * (1.0 - d);
    p_switch + p_inductor + p_diode
}

/// Estimate switching losses for a buck converter.
///
/// P_sw = Vin * Iout * (t_rise + t_fall) * f / 2
pub fn switching_losses(vin: f64, iout: f64, frequency: f64, t_rise: f64, t_fall: f64) -> f64 {
    if frequency <= 0.0 {
        return 0.0;
    }
    vin * iout.abs() * (t_rise + t_fall) * frequency * 0.5
}

/// Full analytical calculation for buck converter.
///
/// Returns a `ConverterResults` with all metrics computed.
pub fn calculate(params: &ConverterParams) -> ConverterResults {
    let duty = required_duty_cycle(params.vin, params.vout_target);
    let vout = output_voltage(params.vin, duty);
    let iout = output_current(vout, params.load_resistance);
    let iin = input_current(iout, duty);
    let il_ripple = inductor_current_ripple(params.vin, duty, params.frequency, params.inductance);
    let vout_ripple = output_voltage_ripple(il_ripple, params.frequency, params.capacitance);

    // Typical values for loss estimation
    let r_switch = 0.1;   // 100 mOhm typical
    let r_inductor = 0.05; // 50 mOhm typical
    let v_f = 0.7;         // typical Schottky
    let t_rise = 20e-9;    // 20 ns typical
    let t_fall = 20e-9;    // 20 ns typical

    let conduction_losses = conduction_losses(iout, duty, r_switch, r_inductor, v_f);
    let switching_losses = switching_losses(params.vin, iout, params.frequency, t_rise, t_fall);

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
        // Buck: Vin=48V, D=0.25 → Vout=12V
        let vout = output_voltage(48.0, 0.25);
        assert!((vout - 12.0).abs() < 1e-6);
    }

    #[test]
    fn test_output_voltage_duty_100() {
        let vout = output_voltage(48.0, 1.0);
        // Clamped to 0.99
        assert!((vout - 47.52).abs() < 1e-6);
    }

    #[test]
    fn test_required_duty_cycle() {
        // Vin=48V, Vout=12V → D=0.25
        let d = required_duty_cycle(48.0, 12.0);
        assert!((d - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_required_duty_cycle_zero_vin() {
        let d = required_duty_cycle(0.0, 12.0);
        assert!((d - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_inductor_current_ripple() {
        // Buck: Vin=48V, D=0.25, f=100kHz, L=100µH
        // ΔiL = 48 * 0.25 * (1-0.25) / (100e3 * 100e-6) = 48 * 0.25 * 0.75 / 10 = 0.9 A
        let ripple = inductor_current_ripple(48.0, 0.25, 100_000.0, 100e-6);
        assert!((ripple - 0.9).abs() < 1e-6);
    }

    #[test]
    fn test_output_voltage_ripple() {
        // ΔVout = ΔiL / (8 * f * C)
        // ΔiL = 0.9, f = 100kHz, C = 100µF
        // ΔVout = 0.9 / (8 * 100e3 * 100e-6) = 0.9 / 80 = 0.01125 V
        let ripple = output_voltage_ripple(0.9, 100_000.0, 100e-6);
        assert!((ripple - 0.01125).abs() < 1e-6);
    }

    #[test]
    fn test_output_current() {
        let i = output_current(12.0, 10.0);
        assert!((i - 1.2).abs() < 1e-6);
    }

    #[test]
    fn test_input_current() {
        let i = input_current(1.2, 0.25);
        assert!((i - 0.3).abs() < 1e-6);
    }

    #[test]
    fn test_full_calculation() {
        let params = ConverterParams {
            vin: 48.0,
            vout_target: 12.0,
            frequency: 100_000.0,
            duty_cycle: 0.25,
            inductance: 100e-6,
            capacitance: 100e-6,
            load_resistance: 10.0,
            ..Default::default()
        };

        let result = calculate(&params);

        // Vout should be close to target
        assert!((result.vout - 12.0).abs() < 0.01);

        // Iout = 12V / 10Ω = 1.2A
        assert!((result.iout - 1.2).abs() < 0.01);

        // Efficiency should be reasonable (> 80%)
        assert!(result.efficiency > 0.8);

        // Ripple values should be positive
        assert!(result.vout_ripple > 0.0);
        assert!(result.il_ripple > 0.0);
    }

    #[test]
    fn test_conduction_losses() {
        // I=1.2A, D=0.25, Rsw=0.1, RL=0.05, Vf=0.7
        // P = 1.44*0.1*0.25 + 1.44*0.05 + 1.2*0.7*0.75
        //   = 0.036 + 0.072 + 0.63 = 0.738 W
        let losses = conduction_losses(1.2, 0.25, 0.1, 0.05, 0.7);
        assert!((losses - 0.738).abs() < 1e-6);
    }

    #[test]
    fn test_switching_losses() {
        // Vin=48V, I=1.2A, f=100kHz, tr=20ns, tf=20ns
        // P = 48 * 1.2 * 40e-9 * 100e3 / 2 = 48 * 1.2 * 0.004 / 2 = 0.1152 W
        let losses = switching_losses(48.0, 1.2, 100_000.0, 20e-9, 20e-9);
        assert!((losses - 0.1152).abs() < 1e-6);
    }
}
