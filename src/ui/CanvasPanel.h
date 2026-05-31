#pragma once

#include "app/AppState.h"
#include "domain/Types.h"
#include "utils/Theme.h"
#include <wx/panel.h>
#include <wx/dc.h>
#include <wx/textctrl.h>
#include <vector>
#include <optional>

class CanvasPanel : public wxPanel {
public:
    CanvasPanel(wxWindow* parent, AppState& state);
    virtual ~CanvasPanel() = default;

    void RefreshCanvas();

private:
    AppState& m_state;
    CanvasState& m_cs;  // alias to m_state.canvas
    ThemeColors m_colors;

    // Canvas geometry constants
    static constexpr float BLOCK_W = 180.0f;
    static constexpr float BLOCK_H = 90.0f;
    static constexpr float PLOT_BLOCK_W = 280.0f;
    static constexpr float PLOT_BLOCK_H = 200.0f;
    static constexpr float GRID_SPACING = 40.0f;

    // Sidebar constants
    static constexpr int SIDEBAR_W = 220;

    // Drag/pan state
    bool m_dragging = false;
    float m_drag_start_x = 0, m_drag_start_y = 0;
    float m_drag_pan_x = 0, m_drag_pan_y = 0;

    // Inline editor state
    std::optional<size_t> m_editing_idx;
    wxTextCtrl* m_edit_ctrl = nullptr;
    CanvasComponentType m_edit_type = CanvasComponentType::Vin;

    // Sidebar palette hover
    std::optional<CanvasComponentType> m_hovered_palette;

    void OnPaint(wxPaintEvent&);
    void OnSize(wxSizeEvent&);
    void OnMouse(wxMouseEvent&);
    void OnKeyDown(wxKeyEvent&);

    // Drawing helpers
    void DrawGrid(wxDC& dc, float ox, float oy, int w, int h);
    void DrawSidebar(wxDC& dc, int w, int h);
    void DrawCanvasContent(wxDC& dc, float ox, float oy, int w, int h);
    void DrawComponentBlock(wxDC& dc, const PlacedComponent& comp, float ox, float oy,
                            float zoom, bool selected, double value);
    void DrawPlotBlock(wxDC& dc, const PlacedComponent& comp, float ox, float oy,
                       float zoom, bool selected);
    void DrawPlacementPreview(wxDC& dc, const PlacedComponent& comp, float ox, float oy, float zoom);

    // Geometry
    wxRect BlockRect(const Pos& pos, float ox, float oy, float zoom) const;
    wxRect PlotRect(const Pos& pos, float ox, float oy, float zoom) const;
    Pos ScreenToCanvas(int sx, int sy, float ox, float oy, float zoom) const;
    Pos Snap(Pos p) const;

    // Hit testing
    std::optional<size_t> FindComponentAt(const Pos& point) const;

    wxDECLARE_EVENT_TABLE();
};
