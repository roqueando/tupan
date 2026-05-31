#pragma once

#include "app/AppState.h"
#include "utils/Theme.h"

#include <wx/panel.h>

class PlotPanel : public wxPanel {
public:
    PlotPanel(wxWindow* parent, AppState& state);
    virtual ~PlotPanel() = default;

    void UpdatePlots();

private:
    AppState& m_state;
    ThemeColors m_colors;
    wxPanel* m_plot_container = nullptr;

    void OnPaint(wxPaintEvent&);
    void OnSize(wxSizeEvent&);

    wxDECLARE_EVENT_TABLE();
};
