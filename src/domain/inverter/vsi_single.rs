/// Single-phase Voltage Source Inverter (VSI) analytical model.

use crate::domain::metrics::thd;
use crate::domain::{ConverterParams, ConverterResults};

/// Calculate the fundamental output voltage amplitude.
///
/// For a full-bridge VSI with sine-triangle PWM:
/// V1 = ma * Vdc  (full-bridge)
/// V1 = ma * Vdc / 2 (half-bridge)
pub fn fundamental_output(ma: f64, vdc: f64, full_bridge: bool) -> f64 {
    thd::fundamental_amplitude(ma, vdc, full_bridge)
}

/// Calculate RMS output voltage.
pub fn rms_output(ma: f64, vdc: f64, full_bridge: bool) -> f64 {
    thd::rms_output_voltage(ma, vdc, full_bridge)
}

/// Calculate output current (resistive load, fundamental component).
pub fn output_current(vout_rms: f64, load_resistance: f64) -> f64 {
    if load_resistance <= 0.0 {
        return 0.0;
    }
    vout_rms / load_resistance
}

/// Calculate input current (average DC side).
///
/// Assuming ideal inverter: Pdc = Pac → Idc * Vdc = Vrms * Irms
/// Idc = (Vrms * Irms) / Vdc
pub fn input_current(vout_rms: f64, iout_rms: f64, vdc: f64) -> f64 {
    if vdc <= 0.0 {
        return 0.0;
    }
    (vout_rms * iout_rms) / vdc
}

/// Estimate conduction losses for a VSI (simplified).
///
/// Includes IGBT/MOSFET and diode losses for all switches.
/// Simplified: 2 switches conducting at any time.
pub fn conduction_losses(
    iout_rms: f64,
    r_switch: f64,
    v_f: f64,
    num_switches_conduction: f64,
) -> f64 {
    let i = iout_rms.abs();
    // Switch conduction loss
    let p_sw = i * i * r_switch * num_switches_conduction;
    // Diode loss (freewheeling)
    // Assuming diode conducts roughly half the time per switch
    let p_diode = i * v_f * num_switches_conduction * 0.3;
    p_sw + p_diode
}

/// Estimate switching losses for a VSI.
///
/// Simplified: P_sw_total = 6 * P_sw_per_switch (for full-bridge, 4 switches)
pub fn switching_losses(
    vdc: f64,
    iout: f64,
    switching_freq: f64,
    t_rise: f64,
    t_fall: f64,
    num_switches: f64,
) -> f64 {
    if switching_freq <= 0.0 {
        return 0.0;
    }
    let p_per_switch = 0.5 * vdc * iout.abs() * (t_rise + t_fall) * switching_freq;
    p_per_switch * num_switches
}

/// Full analytical calculation for single-phase VSI.
pub fn calculate(params: &ConverterParams, full_bridge: bool) -> ConverterResults {
    let v1 = fundamental_output(params.modulation_index, params.vin, full_bridge);
    let vrms = rms_output(params.modulation_index, params.vin, full_bridge);
    let iout = output_current(vrms, params.load_resistance);
    let iin = input_current(vrms, iout, params.vin);

    // THD approximation for bipolar PWM
    let thd = thd::pwm_thd_approximate(params.modulation_index, true);

    // Losses estimation
    let r_switch = 0.1; // IGBT/MOSFET typical Rds(on)
    let v_f = 1.0; // typical diode Vf for high voltage
    let t_rise = 50e-9; // typical IGBT rise time
    let t_fall = 50e-9;

    let num_switches = if full_bridge { 4.0 } else { 2.0 };
    let num_conducting = 2.0; // 2 switches always conducting in full bridge

    let conduction_losses = conduction_losses(iout, r_switch, v_f, num_conducting);
    let switching_losses = switching_losses(
        params.vin,
        iout,
        params.frequency,
        t_rise,
        t_fall,
        num_switches,
    );

    let total_losses = conduction_losses + switching_losses;
    let p_out = vrms * iout;
    let efficiency = if (p_out + total_losses) > 0.0 {
        p_out / (p_out + total_losses)
    } else {
        1.0
    };

    ConverterResults {
        vout: v1, // peak fundamental
        iout,
        iin,
        vout_ripple: 0.0, // Not applicable for VSI (AC output)
        il_ripple: 0.0,   // Not applicable directly
        conduction_losses,
        switching_losses,
        efficiency,
        thd: Some(thd),
        rms_output: Some(vrms),
        fundamental_amplitude: Some(v1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fundamental_output_full_bridge() {
        // ma=0.8, Vdc=300V → V1=0.8*300=240V
        let v1 = fundamental_output(0.8, 300.0, true);
        assert!((v1 - 240.0).abs() < 1e-6);
    }

    #[test]
    fn test_fundamental_output_half_bridge() {
        // ma=0.8, Vdc=300V → V1=0.8*300/2=120V
        let v1 = fundamental_output(0.8, 300.0, false);
        assert!((v1 - 120.0).abs() < 1e-6);
    }

    #[test]
    fn test_full_calculation_full_bridge() {
        let params = ConverterParams {
            vin: 300.0,
            modulation_index: 0.8,
            frequency: 10000.0, // switching frequency
            output_frequency: 60.0,
            load_resistance: 10.0,
            ..Default::default()
        };

        let result = calculate(&params, true);

        // Fundamental should be ~240V peak
        assert!((result.vout - 240.0).abs() < 1.0);

        // THD should be present
        assert!(result.thd.is_some());
        assert!(result.thd.unwrap() > 0.0);

        // Efficiency reasonable
        assert!(result.efficiency > 0.7);
    }

    #[test]
    fn test_output_current() {
        let i = output_current(169.7, 10.0); // 169.7Vrms / 10Ω = 16.97A
        assert!((i - 16.97).abs() < 0.01);
    }

    #[test]
    fn test_conduction_losses_vsi() {
        // I=10A, Rsw=0.1Ω, Vf=1.0V, 2 switches conducting
        // P = 100*0.1*2 + 10*1.0*2*0.3 = 20 + 6 = 26W
        let losses = conduction_losses(10.0, 0.1, 1.0, 2.0);
        assert!((losses - 26.0).abs() < 0.01);
    }
}
