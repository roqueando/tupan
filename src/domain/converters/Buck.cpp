#include "Buck.h"
#include "Common.h"
#include <cmath>

namespace buck {

double output_voltage(double vin, double duty) {
    return vin * converter_common::valid_duty_cycle(duty);
}

double required_duty_cycle(double vin, double vout_target) {
    if (vin <= 0.0) return 0.0;
    return converter_common::valid_duty_cycle(vout_target / vin);
}

double inductor_current_ripple(double vin, double duty, double frequency, double inductance) {
    double d = converter_common::valid_duty_cycle(duty);
    double t = converter_common::switching_period(frequency);
    double v_l = vin * (1.0 - d);
    if (inductance <= 0.0 || frequency <= 0.0) return 0.0;
    return (v_l * d * t) / inductance;
}

double output_voltage_ripple(double il_ripple, double frequency, double capacitance) {
    if (frequency <= 0.0 || capacitance <= 0.0) return 0.0;
    return il_ripple / (8.0 * frequency * capacitance);
}

double output_current(double vout, double load_resistance) {
    if (load_resistance <= 0.0) return 0.0;
    return vout / load_resistance;
}

double input_current(double iout, double duty) {
    return iout * converter_common::valid_duty_cycle(duty);
}

double conduction_losses(double iout, double duty, double r_switch, double r_inductor, double v_f) {
    double d = converter_common::valid_duty_cycle(duty);
    double i = std::abs(iout);
    double p_switch = i * i * r_switch * d;
    double p_inductor = i * i * r_inductor;
    double p_diode = i * v_f * (1.0 - d);
    return p_switch + p_inductor + p_diode;
}

double switching_losses(double vin, double iout, double frequency, double t_rise, double t_fall) {
    if (frequency <= 0.0) return 0.0;
    return vin * std::abs(iout) * (t_rise + t_fall) * frequency * 0.5;
}

ConverterResults calculate(const ConverterParams& params) {
    double duty = required_duty_cycle(params.vin, params.vout_target);
    double vout = output_voltage(params.vin, duty);
    double iout = output_current(vout, params.load_resistance);
    double iin = input_current(iout, duty);
    double il_ripple = inductor_current_ripple(params.vin, duty, params.frequency, params.inductance);
    double vout_ripple = output_voltage_ripple(il_ripple, params.frequency, params.capacitance);

    const double r_switch = 0.1;
    const double r_inductor = 0.05;
    const double v_f = 0.7;
    const double t_rise = 20e-9;
    const double t_fall = 20e-9;

    double cond_loss = conduction_losses(iout, duty, r_switch, r_inductor, v_f);
    double sw_loss = switching_losses(params.vin, iout, params.frequency, t_rise, t_fall);

    double total_losses = cond_loss + sw_loss;
    double p_out = vout * iout;
    double efficiency = (p_out + total_losses) > 0.0 ? p_out / (p_out + total_losses) : 1.0;

    ConverterResults res;
    res.vout = vout;
    res.iout = iout;
    res.iin = iin;
    res.vout_ripple = vout_ripple;
    res.il_ripple = il_ripple;
    res.conduction_losses = cond_loss;
    res.switching_losses = sw_loss;
    res.efficiency = efficiency;
    res.thd = std::nullopt;
    res.rms_output = std::nullopt;
    res.fundamental_amplitude = std::nullopt;
    return res;
}

} // namespace buck
