#pragma once

#include "app/AppState.h"
#include "schematic/Layout.h"
#include "schematic/Renderer.h"
#include "utils/Theme.h"

#include <wx/panel.h>
#include <wx/dc.h>

class SchematicPanel : public wxPanel {
public:
    SchematicPanel(wxWindow* parent, AppState& state);
    virtual ~SchematicPanel() = default;

    void UpdateSchematic();

private:
    AppState& m_state;
    std::vector<SchematicElement> m_elements;

    void OnPaint(wxPaintEvent&);
    void OnSize(wxSizeEvent&);

    wxDECLARE_EVENT_TABLE();
};
