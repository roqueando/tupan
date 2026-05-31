#include "SchematicPanel.h"

#include <wx/dcclient.h>
#include <wx/colour.h>
#include <wx/pen.h>
#include <wx/font.h>

wxBEGIN_EVENT_TABLE(SchematicPanel, wxPanel)
    EVT_PAINT(SchematicPanel::OnPaint)
    EVT_SIZE(SchematicPanel::OnSize)
wxEND_EVENT_TABLE()

SchematicPanel::SchematicPanel(wxWindow* parent, AppState& state)
    : wxPanel(parent, wxID_ANY)
    , m_state(state)
{
    SetBackgroundColour(wxColour(30, 30, 40));
    UpdateSchematic();
}

void SchematicPanel::UpdateSchematic() {
    auto comp_values = m_state.get_component_values();
    m_elements = layout::generate_schematic(m_state.active_converter, comp_values);
    Refresh();
}

void SchematicPanel::OnPaint(wxPaintEvent&) {
    wxPaintDC dc(this);

    // Fill background
    wxSize size = GetClientSize();
    dc.SetBrush(wxBrush(wxColour(18, 18, 26)));
    dc.SetPen(*wxTRANSPARENT_PEN);
    dc.DrawRectangle(0, 0, size.x, size.y);

    // Set pen and font for drawing
    dc.SetPen(wxPen(wxColour(220, 225, 240), 2));
    dc.SetTextForeground(wxColour(220, 225, 240));
    wxFont font(9, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL);
    dc.SetFont(font);

    // Title
    const char* title = "";
    switch (m_state.active_converter) {
        case ConverterType::Buck:           title = "Buck Converter"; break;
        case ConverterType::Boost:          title = "Boost Converter"; break;
        case ConverterType::VsiSinglePhase: title = "Single-Phase VSI"; break;
    }
    dc.DrawText(title, 5, 5);

    // Draw schematic elements (centered, offset 30px from top)
    float ox = 30.0f;
    float oy = 30.0f;
    renderer::draw_all(dc, m_elements, ox, oy);
}

void SchematicPanel::OnSize(wxSizeEvent& event) {
    event.Skip();
}
