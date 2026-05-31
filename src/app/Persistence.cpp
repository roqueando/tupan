#include "Persistence.h"
#include "schematic/ExportSvg.h"
#include <nlohmann/json.hpp>
#include <fstream>
#include <iostream>

using json = nlohmann::json;

namespace persistence {

static json to_json(const SharedParams& sp) {
    return {
        {"vin", sp.vin}, {"vout", sp.vout}, {"duty_cycle", sp.duty_cycle},
        {"frequency", sp.frequency}, {"delta_il", sp.delta_il},
        {"iout_max", sp.iout_max}, {"delta_vo", sp.delta_vo}
    };
}
static void from_json(const json& j, SharedParams& sp) {
    if (j.contains("vin"))         sp.vin = j["vin"].get<double>();
    if (j.contains("vout"))        sp.vout = j["vout"].get<double>();
    if (j.contains("duty_cycle"))  sp.duty_cycle = j["duty_cycle"].get<double>();
    if (j.contains("frequency"))   sp.frequency = j["frequency"].get<double>();
    if (j.contains("delta_il"))    sp.delta_il = j["delta_il"].get<double>();
    if (j.contains("iout_max"))    sp.iout_max = j["iout_max"].get<double>();
    if (j.contains("delta_vo"))    sp.delta_vo = j["delta_vo"].get<double>();
}

static json to_json(const CanvasState& cs) {
    json components = json::array();
    for (auto& c : cs.placed_components) {
        components.push_back({
            {"id", c.id},
            {"type", static_cast<int>(c.component_type)},
            {"x", c.pos.x}, {"y", c.pos.y}
        });
    }
    return {
        {"shared_params", to_json(cs.shared_params)},
        {"components", components},
        {"pan_x", cs.pan_x}, {"pan_y", cs.pan_y},
        {"zoom", cs.zoom}, {"next_id", cs.next_id}
    };
}
static void from_json(const json& j, CanvasState& cs) {
    if (j.contains("shared_params")) from_json(j["shared_params"], cs.shared_params);
    if (j.contains("components")) {
        cs.placed_components.clear();
        for (auto& cj : j["components"]) {
            PlacedComponent pc;
            pc.id = cj["id"].get<uint64_t>();
            pc.component_type = static_cast<CanvasComponentType>(cj["type"].get<int>());
            pc.pos.x = cj["x"].get<float>();
            pc.pos.y = cj["y"].get<float>();
            cs.placed_components.push_back(pc);
        }
    }
    if (j.contains("pan_x")) cs.pan_x = j["pan_x"].get<float>();
    if (j.contains("pan_y")) cs.pan_y = j["pan_y"].get<float>();
    if (j.contains("zoom"))  cs.zoom = j["zoom"].get<float>();
    if (j.contains("next_id")) cs.next_id = j["next_id"].get<uint64_t>();
}

static json to_json(const AppState& state) {
    return {
        {"theme", (state.theme == Theme::Dark) ? "Dark" : "Light"},
        {"status_message", state.status_message},
        {"canvas", to_json(state.canvas)}
    };
}
static void from_json(const json& j, AppState& state) {
    if (j.contains("theme")) state.theme = (j["theme"].get<std::string>() == "Dark") ? Theme::Dark : Theme::Light;
    if (j.contains("status_message")) state.status_message = j["status_message"].get<std::string>();
    if (j.contains("canvas")) from_json(j["canvas"], state.canvas);
}

bool save_project(const std::string& path, const AppState& state) {
    try {
        std::ofstream f(path);
        if (!f.is_open()) return false;
        f << to_json(state).dump(2);
        return true;
    } catch (...) { return false; }
}

bool load_project(const std::string& path, AppState& state) {
    try {
        std::ifstream f(path);
        if (!f.is_open()) return false;
        json j; f >> j;
        from_json(j, state);
        return true;
    } catch (...) { return false; }
}

bool export_schematic_svg(const std::string& path,
                          const std::vector<SchematicElement>& elements,
                          float width, float height)
{
    try {
        std::ofstream f(path);
        if (!f.is_open()) return false;
        f << export_svg::export_svg(elements, width, height);
        return true;
    } catch (...) { return false; }
}

} // namespace persistence
