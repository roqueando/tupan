#include "Renderer.h"

#include <wx/dc.h>
#include <wx/colour.h>
#include <wx/pen.h>
#include <wx/font.h>

namespace renderer {

static void draw_wire(wxDC& dc, const SchematicWire& wire, float ox, float oy) {
    dc.DrawLine(ox + wire.from.x, oy + wire.from.y,
                ox + wire.to.x,   oy + wire.to.y);
}

static void draw_source(wxDC& dc, const SchematicSource& src, float ox, float oy) {
    float cx = ox + src.pos.x;
    float cy = oy + src.pos.y;
    float r = 14.0f;

    dc.DrawCircle(cx, cy, r);
    dc.DrawText("+", cx - 6.0f, cy - 6.0f);
    dc.DrawText("-", cx - 5.0f, cy + 8.0f);

    wxString label = wxString::Format("%s: %s", src.label, src.value);
    dc.DrawText(label, cx + r + 5.0f, cy - 4.0f);
}

static void draw_resistor(wxDC& dc, const SchematicResistor& res, float ox, float oy) {
    float cx = ox + res.pos.x;
    float cy = oy + res.pos.y;
    float w = 30.0f;
    float h = 14.0f;

    dc.DrawRectangle(cx - w / 2.0f, cy - h / 2.0f, w, h);
    dc.DrawLine(cx - w / 2.0f - 10.0f, cy, cx - w / 2.0f, cy);
    dc.DrawLine(cx + w / 2.0f, cy, cx + w / 2.0f + 10.0f, cy);

    wxString label = wxString::Format("%s: %s", res.label, res.value);
    dc.DrawText(label, cx + w / 2.0f + 15.0f, cy - 4.0f);
}

static void draw_inductor(wxDC& dc, const SchematicInductor& ind, float ox, float oy) {
    float cx = ox + ind.pos.x;
    float cy = oy + ind.pos.y;
    int segments = 4;
    float seg_w = 8.0f;
    float seg_h = 10.0f;
    float total_w = segments * seg_w;
    float start_x = cx - total_w / 2.0f;

    float prev_x = start_x - 10.0f;
    float prev_y = cy;
    dc.DrawLine(prev_x, prev_y, start_x, cy);

    for (int i = 0; i < segments; ++i) {
        float x1 = start_x + (i + 0.5f) * seg_w;
        float y1 = (i % 2 == 0) ? (cy - seg_h) : (cy + seg_h);
        dc.DrawLine(prev_x, prev_y, x1, y1);
        prev_x = x1;
        prev_y = y1;
    }
    dc.DrawLine(prev_x, prev_y, start_x + total_w + 10.0f, cy);

    wxString label = wxString::Format("%s: %s", ind.label, ind.value);
    dc.DrawText(label, cx - 20.0f, cy - seg_h - 14.0f);
}

static void draw_capacitor(wxDC& dc, const SchematicCapacitor& cap, float ox, float oy) {
    float cx = ox + cap.pos.x;
    float cy = oy + cap.pos.y;
    float half_plate = 10.0f;

    dc.DrawLine(cx - half_plate, cy, cx + half_plate, cy);
    dc.DrawLine(cx - half_plate, cy + 12.0f, cx + half_plate, cy + 12.0f);
    dc.DrawLine(cx, cy - 8.0f, cx, cy);
    dc.DrawLine(cx, cy + 12.0f, cx, cy + 20.0f);

    wxString label = wxString::Format("%s: %s", cap.label, cap.value);
    dc.DrawText(label, cx + half_plate + 5.0f, cy + 3.0f);
}

static void draw_diode(wxDC& dc, const SchematicDiode& diode, float ox, float oy) {
    float cx = ox + diode.pos.x;
    float cy = oy + diode.pos.y;

    // Triangle
    wxPoint tri[3] = {
        {static_cast<int>(cx), static_cast<int>(cy - 8)},
        {static_cast<int>(cx), static_cast<int>(cy + 8)},
        {static_cast<int>(cx + 12), static_cast<int>(cy)}
    };
    dc.DrawPolygon(3, tri);

    // Bar
    dc.DrawLine(cx + 14.0f, cy - 10.0f, cx + 14.0f, cy + 10.0f);

    // Leads
    dc.DrawLine(cx - 8.0f, cy, cx, cy);
    dc.DrawLine(cx + 14.0f, cy, cx + 22.0f, cy);

    dc.DrawText(diode.label, cx + 22.0f + 5.0f, cy - 2.0f);
}

static void draw_switch(wxDC& dc, const SchematicSwitch& sw, float ox, float oy) {
    float cx = ox + sw.pos.x;
    float cy = oy + sw.pos.y;

    dc.DrawLine(cx - 12.0f, cy, cx, cy);
    dc.DrawLine(cx, cy, cx + 6.0f, cy - 8.0f);
    dc.DrawLine(cx + 12.0f, cy, cx + 16.0f, cy);

    dc.DrawText(sw.label, cx + 6.0f - 10.0f, cy - 14.0f);
}

static void draw_ground(wxDC& dc, const SchematicGround& gnd, float ox, float oy) {
    float cx = ox + gnd.pos.x;
    float cy = oy + gnd.pos.y;

    dc.DrawLine(cx, cy, cx, cy + 6.0f);
    dc.DrawLine(cx - 10.0f, cy + 6.0f, cx + 10.0f, cy + 6.0f);
    dc.DrawLine(cx - 6.0f, cy + 10.0f, cx + 6.0f, cy + 10.0f);
    dc.DrawLine(cx - 3.0f, cy + 14.0f, cx + 3.0f, cy + 14.0f);
}

static void draw_label(wxDC& dc, const SchematicLabel& lbl, float ox, float oy) {
    dc.DrawText(lbl.text, ox + lbl.pos.x, oy + lbl.pos.y);
}

static void draw_node(wxDC& dc, const SchematicNode& node, float ox, float oy) {
    float cx = ox + node.pos.x;
    float cy = oy + node.pos.y;
    dc.DrawCircle(cx, cy, 3.0f);
    dc.DrawText(node.label, cx + 6.0f, cy - 4.0f);
}

void draw_element(wxDC& dc, const SchematicElement& element, float origin_x, float origin_y, bool /*highlight*/) {
    // Save dc state
    dc.SetBrush(*wxTRANSPARENT_BRUSH);
    dc.SetTextForeground(*wxWHITE);

    switch (element.type) {
        case SchematicElementType::Wire:
            draw_wire(dc, element.wire, origin_x, origin_y);
            break;
        case SchematicElementType::Source:
            draw_source(dc, element.source, origin_x, origin_y);
            break;
        case SchematicElementType::Resistor:
            draw_resistor(dc, element.resistor, origin_x, origin_y);
            break;
        case SchematicElementType::Inductor:
            draw_inductor(dc, element.inductor, origin_x, origin_y);
            break;
        case SchematicElementType::Capacitor:
            draw_capacitor(dc, element.capacitor, origin_x, origin_y);
            break;
        case SchematicElementType::Diode:
            draw_diode(dc, element.diode, origin_x, origin_y);
            break;
        case SchematicElementType::Switch:
            draw_switch(dc, element.switch_, origin_x, origin_y);
            break;
        case SchematicElementType::Ground:
            draw_ground(dc, element.ground, origin_x, origin_y);
            break;
        case SchematicElementType::Label:
            draw_label(dc, element.label, origin_x, origin_y);
            break;
        case SchematicElementType::Node:
            draw_node(dc, element.node, origin_x, origin_y);
            break;
    }
}

void draw_all(wxDC& dc, const std::vector<SchematicElement>& elements, float origin_x, float origin_y) {
    for (const auto& elem : elements) {
        draw_element(dc, elem, origin_x, origin_y);
    }
}

} // namespace renderer
