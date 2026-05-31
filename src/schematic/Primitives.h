#pragma once

#include "domain/Types.h"
#include <string>
#include <vector>

// ── Schematic Element Types ───────────────────────────────────────────

struct SchematicSource {
    Pos pos;
    std::string label;
    std::string value;
};

struct SchematicInductor {
    Pos pos;
    std::string label;
    std::string value;
};

struct SchematicCapacitor {
    Pos pos;
    std::string label;
    std::string value;
};

struct SchematicDiode {
    Pos pos;
    std::string label;
};

struct SchematicSwitch {
    Pos pos;
    std::string label;
};

struct SchematicResistor {
    Pos pos;
    std::string label;
    std::string value;
};

struct SchematicWire {
    Pos from;
    Pos to;
};

struct SchematicNode {
    Pos pos;
    std::string label;
};

struct SchematicGround {
    Pos pos;
};

struct SchematicLabel {
    Pos pos;
    std::string text;
};

// ── Variant wrapper ───────────────────────────────────────────────────

enum class SchematicElementType {
    Source, Inductor, Capacitor, Diode, Switch,
    Resistor, Wire, Node, Ground, Label
};

struct SchematicElement {
    SchematicElementType type;
    SchematicSource source;
    SchematicInductor inductor;
    SchematicCapacitor capacitor;
    SchematicDiode diode;
    SchematicSwitch switch_;
    SchematicResistor resistor;
    SchematicWire wire;
    SchematicNode node;
    SchematicGround ground;
    SchematicLabel label;
};

// ── Builder helpers ───────────────────────────────────────────────────

inline SchematicElement make_wire(Pos from, Pos to) {
    SchematicElement e;
    e.type = SchematicElementType::Wire;
    e.wire = {from, to};
    return e;
}

inline SchematicElement make_source(Pos pos, const std::string& label, const std::string& value) {
    SchematicElement e;
    e.type = SchematicElementType::Source;
    e.source = {pos, label, value};
    return e;
}

inline SchematicElement make_resistor(Pos pos, const std::string& label, const std::string& value) {
    SchematicElement e;
    e.type = SchematicElementType::Resistor;
    e.resistor = {pos, label, value};
    return e;
}

inline SchematicElement make_inductor(Pos pos, const std::string& label, const std::string& value) {
    SchematicElement e;
    e.type = SchematicElementType::Inductor;
    e.inductor = {pos, label, value};
    return e;
}

inline SchematicElement make_capacitor(Pos pos, const std::string& label, const std::string& value) {
    SchematicElement e;
    e.type = SchematicElementType::Capacitor;
    e.capacitor = {pos, label, value};
    return e;
}

inline SchematicElement make_diode(Pos pos, const std::string& label) {
    SchematicElement e;
    e.type = SchematicElementType::Diode;
    e.diode = {pos, label};
    return e;
}

inline SchematicElement make_switch(Pos pos, const std::string& label) {
    SchematicElement e;
    e.type = SchematicElementType::Switch;
    e.switch_ = {pos, label};
    return e;
}

inline SchematicElement make_ground(Pos pos) {
    SchematicElement e;
    e.type = SchematicElementType::Ground;
    e.ground = {pos};
    return e;
}

inline SchematicElement make_label(Pos pos, const std::string& text) {
    SchematicElement e;
    e.type = SchematicElementType::Label;
    e.label = {pos, text};
    return e;
}

inline SchematicElement make_node(Pos pos, const std::string& label) {
    SchematicElement e;
    e.type = SchematicElementType::Node;
    e.node = {pos, label};
    return e;
}
