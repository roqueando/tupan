#include "VsiSingle.h"
#include "domain/metrics/Thd.h"
#include <cmath>

namespace vsi_single {

double fundamental_output(double ma, double vdc, bool full_bridge) {
    return thd::fundamental_amplitude(ma, vdc, full_bridge);
}

double rms_output(double ma, double vdc, bool full_bridge) {
    return thd::rms_output_voltage(ma, vdc, full_bridge);
}

double output_current(double vout_rms, double load_resistance) {
    if (load_resistance <= 0.0) return 0.0;
    return vout_rms / load_resistance;
}

double input_current(double vout_rms, double iout_rms, double vdc) {
    if (vdc <= 0.0) return 0.0;
    return (vout_rms * iout_rms) / vdc;
}

double conduction_losses(double iout_rms, double r_switch, double v_f, double num_switches_conduction) {
    double i = std::abs(iout_rms);
    double p_sw = i * i * r_switch * num_switches_conduction;
    double p_diode = i * v_f * num_switches_conduction * 0.3;
    return p_sw + p_diode;
}

double switching_losses(double vdc, double iout, double switching_freq, double t_rise, double t_fall, double num_switches) {
    if (switching_freq <= 0.0) return 0.0;
    double p_per_switch = 0.5 * vdc * std::abs(iout) * (t_rise + t_fall) * switching_freq;
    return p_per_switch * num_switches;
}

ConverterResults calculate(const ConverterParams& params, bool full_bridge) {
    double v1 = fundamental_output(params.modulation_index, params.vin, full_bridge);
    double vrms = rms_output(params.modulation_index, params.vin, full_bridge);
    double iout = output_current(vrms, params.load_resistance);
    double iin = input_current(vrms, iout, params.vin);

    double thd_val = thd::pwm_thd_approximate(params.modulation_index, true);

    const double r_switch = 0.1;
    const double v_f = 1.0;
    const double t_rise = 50e-9;
    const double t_fall = 50e-9;

    double num_switches = full_bridge ? 4.0 : 2.0;
    double num_conducting = 2.0;

    double cond_loss = conduction_losses(iout, r_switch, v_f, num_conducting);
    double sw_loss = switching_losses(params.vin, iout, params.frequency, t_rise, t_fall, num_switches);

    double total_losses = cond_loss + sw_loss;
    double p_out = vrms * iout;
    double efficiency = (p_out + total_losses) > 0.0 ? p_out / (p_out + total_losses) : 1.0;

    ConverterResults res;
    res.vout = v1;
    res.iout = iout;
    res.iin = iin;
    res.vout_ripple = 0.0;
    res.il_ripple = 0.0;
    res.conduction_losses = cond_loss;
    res.switching_losses = sw_loss;
    res.efficiency = efficiency;
    res.thd = thd_val;
    res.rms_output = vrms;
    res.fundamental_amplitude = v1;
    return res;
}

} // namespace vsi_single
