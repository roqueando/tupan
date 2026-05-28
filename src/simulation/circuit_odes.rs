/// ODE systems for power converter circuits.
///
/// Each function defines the state-space model of a converter.
/// State variables are typically inductor current(s) and capacitor voltage(s).

use crate::domain::ConverterParams;
use crate::simulation::integrator::StateVec;

/// Buck converter ODE system.
///
/// States:
///   y[0] = iL  (inductor current)
///   y[1] = vC  (capacitor voltage = Vout)
///
/// During switch ON (duty cycle):
///   diL/dt = (Vin - vC) / L
///   dvC/dt = (iL - vC/R) / C
///
/// During switch OFF:
///   diL/dt = -vC / L
///   dvC/dt = (iL - vC/R) / C
///
/// The switching function s(t) is 1 during ON-time, 0 during OFF-time.
pub struct BuckOde {
    pub vin: f64,
    pub l: f64,
    pub c: f64,
    pub r: f64,
    pub frequency: f64,
    pub duty: f64,
}

impl BuckOde {
    pub fn from_params(params: &ConverterParams) -> Self {
        Self {
            vin: params.vin,
            l: params.inductance,
            c: params.capacitance,
            r: params.load_resistance,
            frequency: params.frequency,
            duty: params.duty_cycle,
        }
    }

    /// Compute the switching function at time t.
    fn switching(&self, t: f64) -> f64 {
        let period = 1.0 / self.frequency;
        let phase = (t % period) / period;
        if phase < self.duty { 1.0 } else { 0.0 }
    }

    pub fn derivatives(&self, t: f64, y: &[f64]) -> StateVec {
        let il = y[0];
        let vc = y[1];
        let s = self.switching(t);

        // During ON: Vin applied, OFF: diode freewheels
        let dil_dt = (s * self.vin - vc) / self.l;
        let dvc_dt = (il - vc / self.r) / self.c;

        vec![dil_dt, dvc_dt]
    }
}

/// Boost converter ODE system.
///
/// States:
///   y[0] = iL  (inductor current)
///   y[1] = vC  (capacitor voltage = Vout)
///
/// During switch ON:
///   diL/dt = Vin / L
///   dvC/dt = -vC / (R*C)   (capacitor discharges into load)
///
/// During switch OFF:
///   diL/dt = (Vin - vC) / L
///   dvC/dt = (iL - vC/R) / C
pub struct BoostOde {
    pub vin: f64,
    pub l: f64,
    pub c: f64,
    pub r: f64,
    pub frequency: f64,
    pub duty: f64,
}

impl BoostOde {
    pub fn from_params(params: &ConverterParams) -> Self {
        Self {
            vin: params.vin,
            l: params.inductance,
            c: params.capacitance,
            r: params.load_resistance,
            frequency: params.frequency,
            duty: params.duty_cycle,
        }
    }

    fn switching(&self, t: f64) -> f64 {
        let period = 1.0 / self.frequency;
        let phase = (t % period) / period;
        if phase < self.duty { 1.0 } else { 0.0 }
    }

    pub fn derivatives(&self, t: f64, y: &[f64]) -> StateVec {
        let il = y[0];
        let vc = y[1];
        let s = self.switching(t);

        let dil_dt = (self.vin - (1.0 - s) * vc) / self.l;
        let dvc_dt = ((1.0 - s) * il - vc / self.r) / self.c;

        vec![dil_dt, dvc_dt]
    }
}

/// Single-phase VSI with RL load ODE system.
///
/// States:
///   y[0] = i_out  (output current)
///
/// For a simple RL load with PWM voltage input:
///   di/dt = (V_pwm(t) - R*i) / L_load
///
/// Where V_pwm(t) = ±Vdc/2 depending on PWM state.
pub struct VsiOde {
    pub vdc: f64,
    pub r_load: f64,
    pub l_load: f64,
    pub carrier_freq: f64,
    pub mod_freq: f64,
    pub ma: f64,  // modulation index
}

impl VsiOde {
    pub fn from_params(params: &ConverterParams) -> Self {
        Self {
            vdc: params.vin,
            r_load: params.load_resistance,
            l_load: params.inductance, // using inductance as load inductance for RL load
            carrier_freq: params.frequency,
            mod_freq: params.output_frequency,
            ma: params.modulation_index,
        }
    }

    /// Generate the PWM voltage at time t.
    fn pwm_voltage(&self, t: f64) -> f64 {
        let omega_m = 2.0 * std::f64::consts::PI * self.mod_freq;
        let omega_c = 2.0 * std::f64::consts::PI * self.carrier_freq;

        // Reference sine
        let v_ref = self.ma * (omega_m * t).sin();

        // Triangle carrier
        let phase_c = (omega_c * t) % (2.0 * std::f64::consts::PI);
        let triangle = if phase_c < std::f64::consts::PI {
            phase_c / std::f64::consts::PI * 2.0 - 1.0
        } else {
            1.0 - (phase_c - std::f64::consts::PI) / std::f64::consts::PI * 2.0
        };

        // Bipolar PWM: +Vdc/2 when v_ref >= triangle, -Vdc/2 otherwise
        if v_ref >= triangle {
            self.vdc / 2.0
        } else {
            -self.vdc / 2.0
        }
    }

    pub fn derivatives(&self, t: f64, y: &[f64]) -> StateVec {
        let i = y[0];
        let v_pwm = self.pwm_voltage(t);

        let di_dt = (v_pwm - self.r_load * i) / self.l_load;

        vec![di_dt]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::integrator::integrate_fixed;

    #[test]
    fn test_buck_ode_steady_state() {
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

        let ode = BuckOde::from_params(&params);

        // Initial guess: iL ≈ Vout/R, vC ≈ Vout
        let vout_est = params.vin * params.duty_cycle;
        let iout_est = vout_est / params.load_resistance;
        let y0 = vec![iout_est, vout_est];

        // Simulate for 5 ms (500 switching cycles at 100kHz)
        let f: Box<dyn Fn(f64, &[f64]) -> Vec<f64>> = Box::new(move |t, y| ode.derivatives(t, y));
        let result = integrate_fixed(
            &f,
            &y0,
            (0.0, 0.005),
            1e-8,
            5000,
        );

        let final_y = result.y.last().unwrap();

        // Should be close to steady-state values
        assert!((final_y[1] - vout_est).abs() < 1.0, "Vout should be near {} V, got {} V", vout_est, final_y[1]);
    }

    #[test]
    fn test_boost_ode_steady_state() {
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

        let ode = BoostOde::from_params(&params);

        let vout_est = params.vin / (1.0 - params.duty_cycle);
        let iout_est = vout_est / params.load_resistance;
        let iin_est = iout_est / (1.0 - params.duty_cycle);
        let y0 = vec![iin_est, vout_est];

        let f: Box<dyn Fn(f64, &[f64]) -> Vec<f64>> = Box::new(move |t, y| ode.derivatives(t, y));
        let result = integrate_fixed(
            &f,
            &y0,
            (0.0, 0.005),
            1e-8,
            5000,
        );

        let final_y = result.y.last().unwrap();

        // Boost should regulate at Vout = Vin/(1-D) = 24V
        assert!((final_y[1] - vout_est).abs() < 2.0, "Vout should be near {} V, got {} V", vout_est, final_y[1]);
    }

    #[test]
    fn test_vsi_ode_basic() {
        let params = ConverterParams {
            vin: 300.0,
            modulation_index: 0.8,
            frequency: 10_000.0,  // carrier freq
            output_frequency: 60.0,
            inductance: 1e-3,     // load inductance
            load_resistance: 10.0,
            ..Default::default()
        };

        let ode = VsiOde::from_params(&params);

        let y0 = vec![0.0]; // start with zero current

        let f: Box<dyn Fn(f64, &[f64]) -> Vec<f64>> = Box::new(move |t, y| ode.derivatives(t, y));
        let result = integrate_fixed(
            &f,
            &y0,
            (0.0, 0.05), // 50 ms = ~3 cycles at 60 Hz
            1e-6,
            10000,
        );

        let final_y = result.y.last().unwrap();

        // Current should be non-zero and within reasonable range
        // Vrms ≈ ma * Vdc / sqrt(2) = 0.8 * 300 / 1.414 =~ 170V
        // Irms ≈ 170 / 10 = 17A, peak ≈ 24A
        assert!(final_y[0].abs() < 50.0, "Current should be reasonable, got {} A", final_y[0]);
        assert!(final_y[0].abs() > 0.1, "Current should be non-zero");
    }
}
