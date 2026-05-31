#include "CanvasPanel.h"
#include "utils/Formatting.h"
#include <wx/dcclient.h>
#include <wx/colour.h>
#include <wx/pen.h>
#include <wx/font.h>
#include <cmath>
#include <algorithm>

wxBEGIN_EVENT_TABLE(CanvasPanel, wxPanel)
    EVT_PAINT(CanvasPanel::OnPaint)
    EVT_SIZE(CanvasPanel::OnSize)
    EVT_MOUSE_EVENTS(CanvasPanel::OnMouse)
wxEND_EVENT_TABLE()

CanvasPanel::CanvasPanel(wxWindow* parent, AppState& state)
    : wxPanel(parent, wxID_ANY, wxDefaultPosition, wxDefaultSize, wxWANTS_CHARS)
    , m_state(state), m_cs(state.canvas), m_colors(ThemeColors::resolve(state.theme))
{
    SetBackgroundColour(m_colors.canvas_bg);
    SetMinSize(wxSize(800, 500));
    Bind(wxEVT_KEY_DOWN, &CanvasPanel::OnKeyDown, this);
}

void CanvasPanel::RefreshCanvas() {
    m_colors = ThemeColors::resolve(m_state.theme);
    SetBackgroundColour(m_colors.canvas_bg);
    Refresh();
}
void CanvasPanel::OnSize(wxSizeEvent&) { Refresh(); }

// ── Drawing helpers ───────────────────────────────────────────────────

static void DrawSH(wxDC& dc, int x, int& y, const wxString& t, const ThemeColors& c) {
    dc.SetTextForeground(c.section_title);
    wxFont f(9, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_BOLD); dc.SetFont(f);
    dc.DrawText(t, x + 8, y + 2);
    y += dc.GetCharHeight() + 6;
    dc.SetPen(wxPen(c.section_title, 1)); dc.DrawLine(x + 8, y, x + 200, y);
    y += 4;
}
static void DrawPR(wxDC& dc, int x, int& y, const wxString& l, const wxString& v, const ThemeColors& c) {
    dc.SetTextForeground(c.text_secondary);
    wxFont f(10, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL); dc.SetFont(f);
    dc.DrawText(l, x + 8, y);
    dc.SetTextForeground(c.text_value);
    dc.DrawText(v, x + 200 - dc.GetTextExtent(v).x, y); y += 16;
}

void CanvasPanel::OnPaint(wxPaintEvent&) {
    wxPaintDC dc(this);
    wxSize sz = GetClientSize();
    int w = sz.x, h = sz.y;

    // Backgrounds
    dc.SetBrush(wxBrush(m_colors.sidebar_bg));
    dc.SetPen(*wxTRANSPARENT_PEN); dc.DrawRectangle(0, 0, SIDEBAR_W, h);
    dc.SetBrush(wxBrush(m_colors.canvas_bg));
    dc.DrawRectangle(SIDEBAR_W, 0, w - SIDEBAR_W, h);

    // Grid
    float ox = SIDEBAR_W + m_cs.pan_x, oy = m_cs.pan_y;
    float gs = GRID_SPACING * m_cs.zoom;
    dc.SetPen(wxPen(m_colors.grid, 1, wxPENSTYLE_DOT));
    float gx = std::fmod(ox, gs); if (gx > 0) gx -= gs;
    while (gx < w - SIDEBAR_W) { dc.DrawLine(int(gx+SIDEBAR_W),0,int(gx+SIDEBAR_W),h); gx += gs; }
    float gy = std::fmod(oy, gs); if (gy > 0) gy -= gs;
    while (gy < h) { dc.DrawLine(SIDEBAR_W,int(gy),SIDEBAR_W+w-SIDEBAR_W,int(gy)); gy += gs; }

    // Sidebar
    int sx = 0, sy = 4;
    auto& sp = m_cs.shared_params;
    auto fmt = [](double v, const char* u) { return formatting::format_value(v, u); };

    DrawSH(dc, sx, sy, "INPUTS", m_colors);
    const char* items[] = {"Vin","Vout","Duty Cycle","Frequency","DiL","Iout,max","DVo"};
    for (auto* n : items) {
        dc.SetTextForeground(m_colors.text_primary);
        wxFont f(10, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL); dc.SetFont(f);
        dc.DrawText(n, sx + 12, sy);
        sy += 24;
    }
    sy += 8;
    DrawSH(dc, sx, sy, "COMPUTED", m_colors);
    const char* items2[] = {"Inductor (L)","Capacitor (C)"};
    for (auto* n : items2) {
        dc.SetTextForeground(m_colors.text_primary);
        wxFont f2(10, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL); dc.SetFont(f2);
        dc.DrawText(n, sx + 12, sy);
        sy += 24;
    }
    sy += 8;
    DrawSH(dc, sx, sy, "VIZ", m_colors);
    dc.DrawText("Curve Plot", sx + 12, sy); sy += 24;
    sy += 12;
    DrawSH(dc, sx, sy, "PARAMETERS", m_colors);
    DrawPR(dc, sx, sy, "Vin",  fmt(sp.vin,"V"), m_colors);
    DrawPR(dc, sx, sy, "Vout", fmt(sp.vout,"V"), m_colors);
    DrawPR(dc, sx, sy, "D",    wxString::Format("%.1f%%",sp.duty_cycle*100), m_colors);
    DrawPR(dc, sx, sy, "Freq", fmt(sp.frequency,"Hz"), m_colors);
    DrawPR(dc, sx, sy, "DiL",  wxString::Format("%.1f%%",sp.delta_il*100), m_colors);
    DrawPR(dc, sx, sy, "Iout,max", fmt(sp.iout_max,"A"), m_colors);
    DrawPR(dc, sx, sy, "DVo",  wxString::Format("%.3f%%",sp.delta_vo*100), m_colors);
    sy += 12;
    DrawSH(dc, sx, sy, "RESULTS", m_colors);
    DrawPR(dc, sx, sy, "L",     fmt(sp.calc_inductance(),"H"), m_colors);
    DrawPR(dc, sx, sy, "C",     fmt(sp.calc_capacitance(),"F"), m_colors);
    DrawPR(dc, sx, sy, "DiL(A)", fmt(sp.calc_delta_il_amps(),"A"), m_colors);

    // Draw placed components
    for (auto& c : m_cs.placed_components) {
        if (!is_plot(c.component_type)) {
            auto r = wxRect(int(ox+c.pos.x*m_cs.zoom), int(oy+c.pos.y*m_cs.zoom), int(BLOCK_W*m_cs.zoom), int(BLOCK_H*m_cs.zoom));
            bool ed = is_editable(c.component_type), co = is_computed(c.component_type);
            dc.SetBrush(wxBrush(ed ? m_colors.input_bg : (co ? m_colors.computed_bg : m_colors.card_bg)));
            dc.SetPen(wxPen(ed ? m_colors.input_border : m_colors.computed_border, 1));
            dc.DrawRoundedRectangle(r.x, r.y, r.width, r.height, 8);
            dc.SetBrush(wxBrush(ed ? ThemeColors::accent() : ThemeColors::accent_dim()));
            dc.SetPen(*wxTRANSPARENT_PEN);
            dc.DrawRoundedRectangle(r.x, r.y, r.width, 3, 2);
            dc.SetTextForeground(m_colors.text_primary);
            wxFont nf(10, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL); dc.SetFont(nf);
            dc.DrawText(canvas_type_name(c.component_type), r.x+8, r.y+8);
            wxString vs = formatting::format_value(m_cs.get_value(c.component_type), canvas_type_unit(c.component_type));
            dc.SetTextForeground(m_colors.text_value);
            wxFont vf(10, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_BOLD); dc.SetFont(vf);
            wxSize ts = dc.GetTextExtent(vs);
            dc.DrawText(vs, r.x+(r.width-ts.x)/2, r.y+r.height-ts.y-6);
        } else {
            // Plot block
            auto r = wxRect(int(ox+c.pos.x*m_cs.zoom), int(oy+c.pos.y*m_cs.zoom), int(PLOT_BLOCK_W*m_cs.zoom), int(PLOT_BLOCK_H*m_cs.zoom));
            dc.SetBrush(wxBrush(m_colors.plot_bg));
            dc.SetPen(wxPen(m_colors.input_border, 1));
            dc.DrawRoundedRectangle(r.x, r.y, r.width, r.height, 8);
            dc.SetTextForeground(m_colors.text_secondary);
            wxFont pf(8, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL); dc.SetFont(pf);
            dc.DrawText("Plot Block", r.x+8, r.y+8);
        }
    }

    // Status
    dc.SetTextForeground(m_colors.status);
    wxFont sf(9, wxFONTFAMILY_TELETYPE, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL); dc.SetFont(sf);
    dc.DrawText(wxString::Format("%zu comps / %.0f%% zoom", m_cs.placed_components.size(), m_cs.zoom*100), SIDEBAR_W+12, h-20);
}

void CanvasPanel::OnMouse(wxMouseEvent& e) {
    bool ca = e.GetX() >= SIDEBAR_W;
    float ox = SIDEBAR_W+m_cs.pan_x, oy = m_cs.pan_y;

    if (e.GetWheelRotation() != 0 && ca) {
        float f = (e.GetWheelRotation()>0) ? 1.1f : 0.9f;
        float nz = std::clamp(m_cs.zoom*f, 0.2f, 5.0f);
        if (std::abs(nz-m_cs.zoom) > 0.001f) {
            float cx = e.GetX()-SIDEBAR_W, cy = e.GetY();
            float wbx = (cx-m_cs.pan_x)/m_cs.zoom, wby = (cy-m_cs.pan_y)/m_cs.zoom;
            m_cs.zoom = nz; m_cs.pan_x = cx-wbx*nz; m_cs.pan_y = cy-wby*nz;
            Refresh();
        }
        return;
    }

    if (e.LeftDown()) {
        if (!ca) {
            int sy = 4+22; m_cs.palette_selection.reset(); m_cs.selected_index.reset();
            for (int i=0;i<7;++i) { if(e.GetY()>=sy&&e.GetY()<sy+26){auto ct=CanvasComponentType(i);
                if(m_cs.palette_selection==ct)m_cs.palette_selection.reset(); else m_cs.palette_selection=ct; Refresh(); return; } sy+=28; }
            sy+=8+22; for(int i=7;i<9;++i){if(e.GetY()>=sy&&e.GetY()<sy+26){auto ct=CanvasComponentType(i);
                if(m_cs.palette_selection==ct)m_cs.palette_selection.reset();else m_cs.palette_selection=ct;Refresh();return;}sy+=28;}
            sy+=8+22; if(e.GetY()>=sy&&e.GetY()<sy+26){auto ct=CanvasComponentType::Plot;
                if(m_cs.palette_selection==ct)m_cs.palette_selection.reset();else m_cs.palette_selection=ct;Refresh();return;}
            Refresh(); return;
        }
        SetFocus();
        float cx = (e.GetX()-ox)/m_cs.zoom, cy = (e.GetY()-oy)/m_cs.zoom;
        auto hit = std::optional<size_t>();
        for (int i=int(m_cs.placed_components.size())-1; i>=0; --i) {
            auto& c = m_cs.placed_components[i];
            float hw = BLOCK_W/2, hh = BLOCK_H/2;
            if (cx>=c.pos.x-hw && cx<=c.pos.x+hw && cy>=c.pos.y-hh && cy<=c.pos.y+hh) { hit=size_t(i); break; }
        }
        if (hit) {
            m_cs.palette_selection.reset(); m_cs.selected_index=hit;
            // Show a simple wxTextCtrl-based inline editor for editable components
            auto& comp = m_cs.placed_components[*hit];
            if (is_editable(comp.component_type) && !is_plot(comp.component_type)) {
                auto r = wxRect(int(ox+comp.pos.x*m_cs.zoom), int(oy+comp.pos.y*m_cs.zoom), int(BLOCK_W*m_cs.zoom), int(BLOCK_H*m_cs.zoom));
                wxTextCtrl* tc = new wxTextCtrl(this, wxID_ANY,
                    wxString::Format("%.3f", m_cs.get_value(comp.component_type)),
                    wxPoint(r.x+10, r.y+40), wxSize(r.width-20, -1),
                    wxTE_PROCESS_ENTER);
                tc->SetFocus();
                tc->SelectAll();
                CanvasComponentType ct = comp.component_type;
                // Store so we can clean up later
                m_edit_ctrl = tc;
                m_edit_type = ct;
                tc->Bind(wxEVT_TEXT_ENTER, [this, ct, tc](wxCommandEvent&) {
                    double v;
                    if (tc->GetValue().ToDouble(&v)) {
                        m_cs.set_value(ct, v);
                    }
                    m_cs.selected_index.reset();
                    tc->Destroy();
                    m_edit_ctrl = nullptr;
                    Refresh();
                });
            }
            Refresh(); return;
        }
        if (m_cs.palette_selection) {
            Pos snp(std::round(cx/GRID_SPACING)*GRID_SPACING, std::round(cy/GRID_SPACING)*GRID_SPACING);
            m_cs.place_component(*m_cs.palette_selection, snp); m_cs.palette_selection.reset(); Refresh(); return;
        }
        m_cs.selected_index.reset(); m_dragging=true;
        m_drag_start_x=e.GetX(); m_drag_start_y=e.GetY();
        m_drag_pan_x=m_cs.pan_x; m_drag_pan_y=m_cs.pan_y;
        return;
    }

    if (e.Moving() || e.Dragging()) {
        if (!ca) {
            m_hovered_palette.reset(); int sy=4+22;
            for(int i=0;i<7;++i){if(e.GetY()>=sy&&e.GetY()<sy+26){m_hovered_palette=CanvasComponentType(i);Refresh();return;}sy+=28;}
            sy+=8+22;for(int i=7;i<9;++i){if(e.GetY()>=sy&&e.GetY()<sy+26){m_hovered_palette=CanvasComponentType(i);Refresh();return;}sy+=28;}
            sy+=8+22;if(e.GetY()>=sy&&e.GetY()<sy+26){m_hovered_palette=CanvasComponentType::Plot;Refresh();return;}Refresh();
        }
        if(e.Dragging()&&m_dragging&&ca){m_cs.pan_x=m_drag_pan_x+(e.GetX()-m_drag_start_x);m_cs.pan_y=m_drag_pan_y+(e.GetY()-m_drag_start_y);Refresh();}
        return;
    }
    if(e.LeftUp()){m_dragging=false;return;}
    if(e.RightDown()&&ca) {
        float cx=(e.GetX()-ox)/m_cs.zoom, cy=(e.GetY()-oy)/m_cs.zoom;
        for(int i=int(m_cs.placed_components.size())-1;i>=0;--i){
            auto&c=m_cs.placed_components[i]; float hw=BLOCK_W/2,hh=BLOCK_H/2;
            if(cx>=c.pos.x-hw&&cx<=c.pos.x+hw&&cy>=c.pos.y-hh&&cy<=c.pos.y+hh){m_cs.selected_index=size_t(i);m_cs.delete_selected();Refresh();return;}
        } return;
    }
    e.Skip();
}

void CanvasPanel::OnKeyDown(wxKeyEvent& e) {
    if((e.GetKeyCode()==WXK_DELETE||e.GetKeyCode()==WXK_BACK)&&m_cs.selected_index){m_cs.delete_selected();Refresh();return;}
    e.Skip();
}
