#include "Layout.h"

namespace layout {

// ── Buck Layout ───────────────────────────────────────────────────────

static std::vector<SchematicElement> buck_layout(const ComponentValues& values) {
    float mid_y = 0.0f;
    float ground_y = 80.0f;
    std::vector<SchematicElement> elements;

    float x_start = 20.0f;
    float x_switch = 80.0f;
    float x_inductor_start = 140.0f;
    float x_inductor_end = 200.0f;
    float x_load = 260.0f;
    float x_end = 320.0f;

    // Top wire segments
    elements.push_back(make_wire({x_start, mid_y}, {x_switch, mid_y}));
    elements.push_back(make_wire({x_switch, mid_y}, {x_inductor_start, mid_y}));
    elements.push_back(make_wire({x_inductor_end, mid_y}, {x_load, mid_y}));
    elements.push_back(make_wire({x_load, mid_y}, {x_end, mid_y}));

    // Source
    elements.push_back(make_source({x_start - 10.0f, mid_y - 30.0f}, "Vin", values.vin));

    // Switch
    elements.push_back(make_switch({x_switch, mid_y}, "SW"));

    // Diode (vertical to ground)
    elements.push_back(make_wire({x_switch, mid_y}, {x_switch, mid_y + 5.0f}));
    elements.push_back(make_diode({x_switch, mid_y + 5.0f}, "D"));
    elements.push_back(make_wire({x_switch, mid_y + 25.0f}, {x_switch, ground_y}));

    // Inductor
    elements.push_back(make_inductor({x_inductor_start + 30.0f, mid_y}, "L", values.inductance));

    // Capacitor (vertical to ground)
    float cap_x = x_inductor_end - 10.0f;
    elements.push_back(make_wire({cap_x, mid_y}, {cap_x, mid_y + 15.0f}));
    elements.push_back(make_capacitor({cap_x, mid_y + 15.0f}, "C", values.capacitance));
    elements.push_back(make_wire({cap_x, mid_y + 35.0f}, {cap_x, ground_y}));

    // Load
    elements.push_back(make_resistor({x_load, mid_y}, "R", values.load));

    // Ground symbols
    elements.push_back(make_ground({x_switch, ground_y}));
    elements.push_back(make_ground({cap_x, ground_y}));
    elements.push_back(make_ground({x_end, ground_y}));

    // Bottom wire
    elements.push_back(make_wire({x_start, ground_y}, {x_switch, ground_y}));
    elements.push_back(make_wire({x_switch, ground_y}, {cap_x, ground_y}));
    elements.push_back(make_wire({cap_x, ground_y}, {x_end, ground_y}));

    // Labels
    elements.push_back(make_label({x_end + 20.0f, mid_y - 5.0f}, "Vout = " + values.vout));
    elements.push_back(make_label({x_end + 20.0f, mid_y + 10.0f}, "f = " + values.frequency));

    return elements;
}

// ── Boost Layout ──────────────────────────────────────────────────────

static std::vector<SchematicElement> boost_layout(const ComponentValues& values) {
    float mid_y = 0.0f;
    float ground_y = 80.0f;
    std::vector<SchematicElement> elements;

    float x_start = 20.0f;
    float x_inductor = 80.0f;
    float x_switch = 150.0f;
    float x_diode = 200.0f;
    float x_cap = 230.0f;
    float x_load = 280.0f;
    float x_end = 330.0f;

    // Top wire
    elements.push_back(make_wire({x_start, mid_y}, {x_inductor, mid_y}));
    elements.push_back(make_wire({x_inductor, mid_y}, {x_switch, mid_y}));
    elements.push_back(make_wire({x_switch, mid_y}, {x_diode, mid_y}));
    elements.push_back(make_wire({x_diode, mid_y}, {x_cap, mid_y}));
    elements.push_back(make_wire({x_cap, mid_y}, {x_load, mid_y}));
    elements.push_back(make_wire({x_load, mid_y}, {x_end, mid_y}));

    // Source
    elements.push_back(make_source({x_start - 10.0f, mid_y - 30.0f}, "Vin", values.vin));

    // Inductor
    elements.push_back(make_inductor({x_inductor, mid_y}, "L", values.inductance));

    // Switch (to ground)
    elements.push_back(make_wire({x_switch, mid_y}, {x_switch, mid_y + 5.0f}));
    elements.push_back(make_switch({x_switch, mid_y + 5.0f}, "SW"));
    elements.push_back(make_wire({x_switch, mid_y + 25.0f}, {x_switch, ground_y}));

    // Diode
    elements.push_back(make_diode({x_diode, mid_y}, "D"));

    // Output capacitor
    elements.push_back(make_wire({x_cap, mid_y}, {x_cap, mid_y + 15.0f}));
    elements.push_back(make_capacitor({x_cap, mid_y + 15.0f}, "C", values.capacitance));
    elements.push_back(make_wire({x_cap, mid_y + 35.0f}, {x_cap, ground_y}));

    // Load
    elements.push_back(make_resistor({x_load, mid_y}, "R", values.load));

    // Grounds
    elements.push_back(make_ground({x_start, ground_y}));
    elements.push_back(make_ground({x_switch, ground_y}));
    elements.push_back(make_ground({x_cap, ground_y}));

    // Bottom wire
    elements.push_back(make_wire({x_start, ground_y}, {x_switch, ground_y}));
    elements.push_back(make_wire({x_switch, ground_y}, {x_cap, ground_y}));
    elements.push_back(make_wire({x_cap, ground_y}, {x_end, ground_y}));

    // Label
    elements.push_back(make_label({x_end + 20.0f, mid_y - 5.0f}, "Vout = " + values.vout));

    return elements;
}

// ── VSI Layout ────────────────────────────────────────────────────────

static std::vector<SchematicElement> vsi_layout(const ComponentValues& values) {
    float mid_y = 0.0f;
    float top_y = -80.0f;
    float bot_y = 80.0f;
    std::vector<SchematicElement> elements;

    elements.push_back(make_source({30.0f, top_y - 20.0f}, "Vdc", values.vin));
    elements.push_back(make_label({100.0f, mid_y - 30.0f}, "H-Bridge"));
    elements.push_back(make_label({100.0f, mid_y - 10.0f}, "PWM Inverter"));
    elements.push_back(make_resistor({200.0f, mid_y}, "R", values.load));
    elements.push_back(make_ground({30.0f, bot_y}));
    elements.push_back(make_ground({200.0f, bot_y}));
    elements.push_back(make_label({250.0f, mid_y - 5.0f}, "Vrms = " + values.vout));

    return elements;
}

// ── Dispatcher ────────────────────────────────────────────────────────

std::vector<SchematicElement> generate_schematic(
    ConverterType converter_type,
    const ComponentValues& values)
{
    switch (converter_type) {
        case ConverterType::Buck:           return buck_layout(values);
        case ConverterType::Boost:          return boost_layout(values);
        case ConverterType::VsiSinglePhase: return vsi_layout(values);
    }
    return {};
}

} // namespace layout
