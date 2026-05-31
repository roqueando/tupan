#include "ExportSvg.h"
#include <sstream>
#include <string>
#include <vector>

namespace export_svg {

static std::string fmt(float v) {
    char buf[32];
    std::snprintf(buf, sizeof(buf), "%.1f", v);
    return buf;
}

static void write_wire(std::ostringstream& svg, const SchematicWire& wire) {
    svg << "  <line class=\"wire\" x1=\"" << fmt(wire.from.x)
        << "\" y1=\"" << fmt(wire.from.y)
        << "\" x2=\"" << fmt(wire.to.x)
        << "\" y2=\"" << fmt(wire.to.y) << "\"/>\n";
}

static void write_source(std::ostringstream& svg, const SchematicSource& src) {
    float cx = src.pos.x;
    float cy = src.pos.y;
    float r = 14.0f;

    svg << "  <circle class=\"comp\" cx=\"" << fmt(cx) << "\" cy=\"" << fmt(cy) << "\" r=\"" << fmt(r) << "\"/>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx - 4.0f) << "\" y=\"" << fmt(cy - 8.0f + 4.0f) << "\" text-anchor=\"middle\">+</text>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx - 4.0f) << "\" y=\"" << fmt(cy + 8.0f + 4.0f) << "\" text-anchor=\"middle\">-</text>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx + r + 5.0f) << "\" y=\"" << fmt(cy + 3.0f)
        << "\">" << src.label << ": " << src.value << "</text>\n";
}

static void write_resistor(std::ostringstream& svg, const SchematicResistor& res) {
    float cx = res.pos.x;
    float cy = res.pos.y;
    float w = 30.0f;
    float h = 14.0f;

    svg << "  <rect class=\"comp\" x=\"" << fmt(cx - w / 2.0f)
        << "\" y=\"" << fmt(cy - h / 2.0f)
        << "\" width=\"" << fmt(w) << "\" height=\"" << fmt(h) << "\"/>\n";
    svg << "  <line class=\"wire\" x1=\"" << fmt(cx - w / 2.0f - 10.0f)
        << "\" y1=\"" << fmt(cy) << "\" x2=\"" << fmt(cx - w / 2.0f)
        << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <line class=\"wire\" x1=\"" << fmt(cx + w / 2.0f)
        << "\" y1=\"" << fmt(cy) << "\" x2=\"" << fmt(cx + w / 2.0f + 10.0f)
        << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx + w / 2.0f + 15.0f)
        << "\" y=\"" << fmt(cy + 3.0f)
        << "\">" << res.label << ": " << res.value << "</text>\n";
}

static void write_inductor(std::ostringstream& svg, const SchematicInductor& ind) {
    float cx = ind.pos.x;
    float cy = ind.pos.y;
    int segments = 4;
    float seg_w = 8.0f;
    float seg_h = 10.0f;
    float total_w = segments * seg_w;
    float start_x = cx - total_w / 2.0f;

    std::string d = "M " + fmt(start_x - 10.0f) + " " + fmt(cy);
    for (int i = 0; i < segments; ++i) {
        float x1 = start_x + (i + 0.5f) * seg_w;
        float y1 = (i % 2 == 0) ? (cy - seg_h) : (cy + seg_h);
        d += " L " + fmt(x1) + " " + fmt(y1);
    }
    d += " L " + fmt(start_x + total_w + 10.0f) + " " + fmt(cy);

    svg << "  <path class=\"comp\" d=\"" << d << "\"/>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx)
        << "\" y=\"" << fmt(cy - seg_h - 5.0f)
        << "\" text-anchor=\"middle\">" << ind.label << ": " << ind.value << "</text>\n";
}

static void write_capacitor(std::ostringstream& svg, const SchematicCapacitor& cap) {
    float cx = cap.pos.x;
    float cy = cap.pos.y;
    float half_plate = 10.0f;

    svg << "  <line class=\"comp\" x1=\"" << fmt(cx - half_plate)
        << "\" y1=\"" << fmt(cy) << "\" x2=\"" << fmt(cx + half_plate)
        << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <line class=\"comp\" x1=\"" << fmt(cx - half_plate)
        << "\" y1=\"" << fmt(cy + 12.0f) << "\" x2=\"" << fmt(cx + half_plate)
        << "\" y2=\"" << fmt(cy + 12.0f) << "\"/>\n";
    svg << "  <line class=\"wire\" x1=\"" << fmt(cx) << "\" y1=\"" << fmt(cy - 8.0f)
        << "\" x2=\"" << fmt(cx) << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <line class=\"wire\" x1=\"" << fmt(cx) << "\" y1=\"" << fmt(cy + 12.0f)
        << "\" x2=\"" << fmt(cx) << "\" y2=\"" << fmt(cy + 20.0f) << "\"/>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx + half_plate + 5.0f)
        << "\" y=\"" << fmt(cy + 8.0f)
        << "\">" << cap.label << ": " << cap.value << "</text>\n";
}

static void write_diode(std::ostringstream& svg, const SchematicDiode& diode) {
    float cx = diode.pos.x;
    float cy = diode.pos.y;

    svg << "  <polygon class=\"comp\" points=\""
        << fmt(cx) << "," << fmt(cy - 8.0f) << " "
        << fmt(cx) << "," << fmt(cy + 8.0f) << " "
        << fmt(cx + 12.0f) << "," << fmt(cy) << "\"/>\n";
    svg << "  <line class=\"comp\" x1=\"" << fmt(cx + 14.0f)
        << "\" y1=\"" << fmt(cy - 10.0f)
        << "\" x2=\"" << fmt(cx + 14.0f)
        << "\" y2=\"" << fmt(cy + 10.0f) << "\"/>\n";
    svg << "  <line class=\"wire\" x1=\"" << fmt(cx - 8.0f)
        << "\" y1=\"" << fmt(cy) << "\" x2=\"" << fmt(cx)
        << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <line class=\"wire\" x1=\"" << fmt(cx + 14.0f)
        << "\" y1=\"" << fmt(cy) << "\" x2=\"" << fmt(cx + 22.0f)
        << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx + 22.0f + 5.0f)
        << "\" y=\"" << fmt(cy + 3.0f)
        << "\">" << diode.label << "</text>\n";
}

static void write_switch(std::ostringstream& svg, const SchematicSwitch& sw) {
    float cx = sw.pos.x;
    float cy = sw.pos.y;

    svg << "  <line class=\"wire\" x1=\"" << fmt(cx - 12.0f) << "\" y1=\"" << fmt(cy)
        << "\" x2=\"" << fmt(cx) << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <line class=\"comp\" x1=\"" << fmt(cx) << "\" y1=\"" << fmt(cy)
        << "\" x2=\"" << fmt(cx + 6.0f) << "\" y2=\"" << fmt(cy - 8.0f) << "\"/>\n";
    svg << "  <line class=\"wire\" x1=\"" << fmt(cx + 12.0f) << "\" y1=\"" << fmt(cy)
        << "\" x2=\"" << fmt(cx + 16.0f) << "\" y2=\"" << fmt(cy) << "\"/>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx + 6.0f) << "\" y=\"" << fmt(cy - 14.0f)
        << "\" text-anchor=\"middle\">" << sw.label << "</text>\n";
}

static void write_ground(std::ostringstream& svg, const SchematicGround& gnd) {
    float cx = gnd.pos.x;
    float cy = gnd.pos.y;
    svg << "  <line class=\"ground\" x1=\"" << fmt(cx) << "\" y1=\"" << fmt(cy)
        << "\" x2=\"" << fmt(cx) << "\" y2=\"" << fmt(cy + 6.0f) << "\"/>\n";
    svg << "  <line class=\"ground\" x1=\"" << fmt(cx - 10.0f) << "\" y1=\"" << fmt(cy + 6.0f)
        << "\" x2=\"" << fmt(cx + 10.0f) << "\" y2=\"" << fmt(cy + 6.0f) << "\"/>\n";
    svg << "  <line class=\"ground\" x1=\"" << fmt(cx - 6.0f) << "\" y1=\"" << fmt(cy + 10.0f)
        << "\" x2=\"" << fmt(cx + 6.0f) << "\" y2=\"" << fmt(cy + 10.0f) << "\"/>\n";
    svg << "  <line class=\"ground\" x1=\"" << fmt(cx - 3.0f) << "\" y1=\"" << fmt(cy + 14.0f)
        << "\" x2=\"" << fmt(cx + 3.0f) << "\" y2=\"" << fmt(cy + 14.0f) << "\"/>\n";
}

static void write_node(std::ostringstream& svg, const SchematicNode& node) {
    float cx = node.pos.x;
    float cy = node.pos.y;
    svg << "  <circle class=\"comp\" cx=\"" << fmt(cx) << "\" cy=\"" << fmt(cy)
        << "\" r=\"3\" fill=\"black\"/>\n";
    svg << "  <text class=\"label\" x=\"" << fmt(cx + 6.0f)
        << "\" y=\"" << fmt(cy + 3.0f)
        << "\">" << node.label << "</text>\n";
}

static void write_label(std::ostringstream& svg, const SchematicLabel& lbl) {
    svg << "  <text class=\"label\" x=\"" << fmt(lbl.pos.x)
        << "\" y=\"" << fmt(lbl.pos.y + 3.0f)
        << "\">" << lbl.text << "</text>\n";
}

std::string export_svg(const std::vector<SchematicElement>& elements, float width, float height) {
    std::ostringstream svg;

    svg << R"(<?xml version="1.0" encoding="UTF-8"?>)" << "\n";
    svg << "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 "
        << static_cast<int>(width) << " " << static_cast<int>(height)
        << "\" width=\"" << static_cast<int>(width) << "pt\" height=\""
        << static_cast<int>(height) << "pt\">\n";

    svg << R"(  <rect width="100%" height="100%" fill="white"/>)" << "\n";

    svg << R"(  <style>
    .wire { stroke: black; stroke-width: 1.5; fill: none; }
    .comp { stroke: black; stroke-width: 1.5; fill: none; }
    .label { font-family: monospace; font-size: 11px; fill: #333; }
    .value { font-family: monospace; font-size: 10px; fill: #666; }
    .ground { stroke: black; stroke-width: 1.5; fill: none; }
  </style>)" << "\n";

    for (const auto& elem : elements) {
        switch (elem.type) {
            case SchematicElementType::Wire:      write_wire(svg, elem.wire); break;
            case SchematicElementType::Source:    write_source(svg, elem.source); break;
            case SchematicElementType::Resistor:  write_resistor(svg, elem.resistor); break;
            case SchematicElementType::Inductor:  write_inductor(svg, elem.inductor); break;
            case SchematicElementType::Capacitor: write_capacitor(svg, elem.capacitor); break;
            case SchematicElementType::Diode:     write_diode(svg, elem.diode); break;
            case SchematicElementType::Switch:    write_switch(svg, elem.switch_); break;
            case SchematicElementType::Ground:    write_ground(svg, elem.ground); break;
            case SchematicElementType::Node:      write_node(svg, elem.node); break;
            case SchematicElementType::Label:     write_label(svg, elem.label); break;
        }
    }

    svg << "</svg>\n";
    return svg.str();
}

} // namespace export_svg
