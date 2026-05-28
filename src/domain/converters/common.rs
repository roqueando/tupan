/// Shared utilities for converter calculations.

/// Calculate the angular frequency from frequency in Hz.
pub fn angular_frequency(f: f64) -> f64 {
    2.0 * std::f64::consts::PI * f
}

/// Calculate the switching period from frequency.
pub fn switching_period(f: f64) -> f64 {
    1.0 / f
}

/// Clamp a value between min and max.
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Duty cycle clamped to valid range (0..1).
pub fn valid_duty_cycle(d: f64) -> f64 {
    clamp(d, 0.01, 0.99)
}
