#include "Persistence.h"
#include "schematic/ExportSvg.h"

#include <nlohmann/json.hpp>
#include <fstream>
#include <iostream>

using json = nlohmann::json;

namespace persistence {

static json to_json(const ConverterParams& p) {
    return {
        {"vin", p.vin},
        {"vout_target", p.vout_target},
        {"frequency", p.frequency},
        {"duty_cycle", p.duty_cycle},
        {"inductance", p.inductance},
        {"capacitance", p.capacitance},
        {"load_resistance", p.load_resistance},
        {"modulation_index", p.modulation_index},
        {"output_frequency", p.output_frequency}
    };
}

static void from_json(const json& j, ConverterParams& p) {
    if (j.contains("vin"))                p.vin = j["vin"].get<double>();
    if (j.contains("vout_target"))        p.vout_target = j["vout_target"].get<double>();
    if (j.contains("frequency"))           p.frequency = j["frequency"].get<double>();
    if (j.contains("duty_cycle"))          p.duty_cycle = j["duty_cycle"].get<double>();
    if (j.contains("inductance"))          p.inductance = j["inductance"].get<double>();
    if (j.contains("capacitance"))         p.capacitance = j["capacitance"].get<double>();
    if (j.contains("load_resistance"))     p.load_resistance = j["load_resistance"].get<double>();
    if (j.contains("modulation_index"))    p.modulation_index = j["modulation_index"].get<double>();
    if (j.contains("output_frequency"))    p.output_frequency = j["output_frequency"].get<double>();
}

static json to_json(const ConverterResults& r) {
    json j = {
        {"vout", r.vout},
        {"iout", r.iout},
        {"iin", r.iin},
        {"vout_ripple", r.vout_ripple},
        {"il_ripple", r.il_ripple},
        {"conduction_losses", r.conduction_losses},
        {"switching_losses", r.switching_losses},
        {"efficiency", r.efficiency}
    };
    if (r.thd)                     j["thd"] = *r.thd;
    if (r.rms_output)              j["rms_output"] = *r.rms_output;
    if (r.fundamental_amplitude)   j["fundamental_amplitude"] = *r.fundamental_amplitude;
    return j;
}

static void from_json(const json& j, ConverterResults& r) {
    if (j.contains("vout"))              r.vout = j["vout"].get<double>();
    if (j.contains("iout"))              r.iout = j["iout"].get<double>();
    if (j.contains("iin"))               r.iin = j["iin"].get<double>();
    if (j.contains("vout_ripple"))       r.vout_ripple = j["vout_ripple"].get<double>();
    if (j.contains("il_ripple"))         r.il_ripple = j["il_ripple"].get<double>();
    if (j.contains("conduction_losses")) r.conduction_losses = j["conduction_losses"].get<double>();
    if (j.contains("switching_losses"))  r.switching_losses = j["switching_losses"].get<double>();
    if (j.contains("efficiency"))        r.efficiency = j["efficiency"].get<double>();
    if (j.contains("thd") && !j["thd"].is_null()) r.thd = j["thd"].get<double>();
    if (j.contains("rms_output") && !j["rms_output"].is_null()) r.rms_output = j["rms_output"].get<double>();
    if (j.contains("fundamental_amplitude") && !j["fundamental_amplitude"].is_null())
        r.fundamental_amplitude = j["fundamental_amplitude"].get<double>();
}

static json to_json(const AppState& state) {
    return {
        {"active_converter", static_cast<int>(state.active_converter)},
        {"params", to_json(state.params)},
        {"results", to_json(state.results)},
        {"show_numerical_sim", state.show_numerical_sim},
        {"show_schematic", state.show_schematic},
        {"theme", (state.theme == Theme::Dark) ? "Dark" : "Light"},
        {"status_message", state.status_message}
    };
}

static void from_json(const json& j, AppState& state) {
    if (j.contains("active_converter"))
        state.active_converter = static_cast<ConverterType>(j["active_converter"].get<int>());
    if (j.contains("params"))     from_json(j["params"], state.params);
    if (j.contains("results"))    from_json(j["results"], state.results);
    if (j.contains("show_numerical_sim")) state.show_numerical_sim = j["show_numerical_sim"].get<bool>();
    if (j.contains("show_schematic"))     state.show_schematic = j["show_schematic"].get<bool>();
    if (j.contains("theme"))              state.theme = (j["theme"].get<std::string>() == "Dark") ? Theme::Dark : Theme::Light;
    if (j.contains("status_message"))     state.status_message = j["status_message"].get<std::string>();
}

bool save_project(const std::string& path, const AppState& state) {
    try {
        json j = to_json(state);
        std::ofstream file(path);
        if (!file.is_open()) {
            std::cerr << "Failed to open " << path << " for writing\n";
            return false;
        }
        file << j.dump(2);
        return true;
    } catch (const std::exception& e) {
        std::cerr << "Error saving project: " << e.what() << "\n";
        return false;
    }
}

bool load_project(const std::string& path, AppState& state) {
    try {
        std::ifstream file(path);
        if (!file.is_open()) {
            std::cerr << "Failed to open " << path << " for reading\n";
            return false;
        }
        json j;
        file >> j;
        from_json(j, state);
        return true;
    } catch (const std::exception& e) {
        std::cerr << "Error loading project: " << e.what() << "\n";
        return false;
    }
}

bool export_schematic_svg(const std::string& path,
                          const std::vector<SchematicElement>& elements,
                          float width, float height)
{
    try {
        std::string svg = export_svg::export_svg(elements, width, height);
        std::ofstream file(path);
        if (!file.is_open()) {
            std::cerr << "Failed to open " << path << " for SVG writing\n";
            return false;
        }
        file << svg;
        return true;
    } catch (const std::exception& e) {
        std::cerr << "Error exporting SVG: " << e.what() << "\n";
        return false;
    }
}

} // namespace persistence
