/// Efficiency calculation utilities.

/// Calculate efficiency from output power and total losses.
///
/// η = Pout / (Pout + Ploss)
pub fn efficiency(p_out: f64, total_losses: f64) -> f64 {
    if (p_out + total_losses) <= 0.0 {
        return 1.0;
    }
    p_out / (p_out + total_losses)
}

/// Format efficiency as percentage string.
pub fn efficiency_percent(eff: f64) -> String {
    format!("{:.1}%", eff * 100.0)
}

/// Calculate power dissipation in a MOSFET as a ratio of switching + conduction losses.
///
/// P_mosfet = I² * R_ds_on * D + 0.5 * Vin * I * (t_rise + t_fall) * f
pub fn mosfet_power_loss(
    i_drain: f64,
    r_ds_on: f64,
    duty: f64,
    v_ds: f64,
    t_rise: f64,
    t_fall: f64,
    frequency: f64,
) -> f64 {
    let d = duty.clamp(0.0, 1.0);
    let i = i_drain.abs();
    let p_conduction = i * i * r_ds_on * d;
    let p_switching = if frequency > 0.0 {
        0.5 * v_ds * i * (t_rise + t_fall) * frequency
    } else {
        0.0
    };
    p_conduction + p_switching
}

/// Calculate power dissipation in a diode (simplified — just forward voltage drop).
///
/// P_diode = I_f * V_f * conduction_fraction
pub fn diode_power_loss(i_forward: f64, v_f: f64, conduction_fraction: f64) -> f64 {
    i_forward.abs() * v_f * conduction_fraction.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efficiency() {
        let eff = efficiency(100.0, 10.0);
        assert!((eff - 100.0 / 110.0).abs() < 1e-6);
    }

    #[test]
    fn test_efficiency_format() {
        let s = efficiency_percent(0.912);
        assert_eq!(s, "91.2%");
    }

    #[test]
    fn test_mosfet_power_loss() {
        // I=1.2A, Rds=0.1Ω, D=0.25, Vds=48V, tr=20ns, tf=20ns, f=100kHz
        // Pcond = 1.44 * 0.1 * 0.25 = 0.036
        // Psw = 0.5 * 48 * 1.2 * 40e-9 * 100e3 = 0.5 * 48 * 1.2 * 0.004 = 0.1152
        // Total = 0.1512
        let loss = mosfet_power_loss(1.2, 0.1, 0.25, 48.0, 20e-9, 20e-9, 100_000.0);
        assert!((loss - 0.1512).abs() < 1e-6);
    }

    #[test]
    fn test_diode_power_loss() {
        // If=1.2A, Vf=0.7V, conducts 75% of time
        let loss = diode_power_loss(1.2, 0.7, 0.75);
        assert!((loss - 0.63).abs() < 1e-6);
    }
}
