#![allow(dead_code)]

pub mod components;
pub mod converters;
pub mod inverter;
pub mod metrics;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConverterType {
    Buck,
    Boost,
    VsiSinglePhase,
}

impl ConverterType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Buck => "Buck Converter",
            Self::Boost => "Boost Converter",
            Self::VsiSinglePhase => "VSI Single-Phase",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConverterParams {
    // Entrada
    pub vin: f64,          // V
    pub vout_target: f64,  // V (target)

    // Comutação
    pub frequency: f64,    // Hz
    pub duty_cycle: f64,   // 0..1

    // Componentes
    pub inductance: f64,   // H
    pub capacitance: f64,  // F
    pub load_resistance: f64, // Ohm

    // Inversor-specific
    pub modulation_index: f64,  // 0..1
    pub output_frequency: f64,  // Hz
}

impl Default for ConverterParams {
    fn default() -> Self {
        Self {
            vin: 48.0,
            vout_target: 12.0,
            frequency: 100_000.0,
            duty_cycle: 0.5,
            inductance: 100e-6,
            capacitance: 100e-6,
            load_resistance: 10.0,
            modulation_index: 0.8,
            output_frequency: 60.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConverterResults {
    // Tensões e correntes
    pub vout: f64,           // Tensão de saída média
    pub iout: f64,           // Corrente de saída média
    pub iin: f64,            // Corrente de entrada média

    // Ripple
    pub vout_ripple: f64,    // Ripple de tensão de saída (Vpp)
    pub il_ripple: f64,      // Ripple de corrente no indutor (App)

    // Perdas e eficiência
    pub conduction_losses: f64,
    pub switching_losses: f64,
    pub efficiency: f64,     // 0..1

    // Inversor
    pub thd: Option<f64>,
    pub rms_output: Option<f64>,
    pub fundamental_amplitude: Option<f64>,
}

impl ConverterResults {
    pub fn zero() -> Self {
        Self {
            vout: 0.0,
            iout: 0.0,
            iin: 0.0,
            vout_ripple: 0.0,
            il_ripple: 0.0,
            conduction_losses: 0.0,
            switching_losses: 0.0,
            efficiency: 0.0,
            thd: None,
            rms_output: None,
            fundamental_amplitude: None,
        }
    }
}
