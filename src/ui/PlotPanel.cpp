#include "PlotPanel.h"
#include <wx/stattext.h>
#include <wx/sizer.h>
#include <wx/statline.h>
#include <wx/dcclient.h>
#include <wx/colour.h>
#include <wx/pen.h>
#include <wx/font.h>
#include <algorithm>
#include <cmath>

// ── SimplePlotWidget ── Custom paint line plot ────────────────────────
// Paints directly on wxDC without an event table (avoids vtable issues on macOS).

class SimplePlotWidget : public wxPanel {
public:
    SimplePlotWidget(wxWindow* parent)
        : wxPanel(parent, wxID_ANY)
    {
        SetMinSize(wxSize(200, 120));
        Bind(wxEVT_PAINT, &SimplePlotWidget::OnPaint, this);
    }

    void AddSeries(const std::vector<std::pair<double, double>>& data,
                   const wxColour& color, const wxString& label)
    {
        m_series.push_back({data, color, label});
    }

    void Clear() { m_series.clear(); }

private:
    struct Series {
        std::vector<std::pair<double, double>> data;
        wxColour color;
        wxString label;
    };
    std::vector<Series> m_series;

    void OnPaint(wxPaintEvent&) {
        wxPaintDC dc(this);
        wxSize sz = GetClientSize();
        int w = sz.x, h = sz.y;

        // Background
        dc.SetBrush(wxBrush(wxColour(20, 22, 32)));
        dc.SetPen(*wxTRANSPARENT_PEN);
        dc.DrawRectangle(0, 0, w, h);

        int ml = 55, mr = 15, mt = 20, mb = 30;
        int pw = w - ml - mr, ph = h - mt - mb;
        if (pw < 20 || ph < 20) return;

        // Border
        dc.SetPen(wxPen(wxColour(60, 60, 80), 1));
        dc.SetBrush(*wxTRANSPARENT_BRUSH);
        dc.DrawRectangle(ml, mt, pw, ph);

        // Grid
        dc.SetPen(wxPen(wxColour(60, 60, 80, 40), 1, wxPENSTYLE_DOT));
        for (int i = 1; i < 4; ++i) {
            int x = ml + pw * i / 4;
            dc.DrawLine(x, mt, x, mt + ph);
        }
        for (int i = 1; i < 4; ++i) {
            int y = mt + ph * i / 4;
            dc.DrawLine(ml, y, ml + pw, y);
        }

        // Find bounds
        double x_min = INFINITY, x_max = -INFINITY;
        double y_min = INFINITY, y_max = -INFINITY;
        for (auto& s : m_series)
            for (auto& p : s.data) {
                x_min = std::min(x_min, p.first);
                x_max = std::max(x_max, p.first);
                y_min = std::min(y_min, p.second);
                y_max = std::max(y_max, p.second);
            }
        if (m_series.empty()) return;

        double xr = (x_max - x_min) * 0.05;
        double yr = (y_max - y_min) * 0.1;
        if (xr < 1e-12) xr = 0.5;
        if (yr < 1e-12) yr = 0.5;
        x_min -= xr; x_max += xr;
        y_min -= yr; y_max += yr;

        // Data series
        int idx = 0;
        for (auto& s : m_series) {
            if (s.data.size() < 2) continue;
            dc.SetPen(wxPen(s.color, idx == 0 ? 2 : 1));
            for (size_t i = 1; i < s.data.size(); ++i) {
                auto to_screen = [&](double vx, double vy) -> wxPoint {
                    int sx = ml + int((vx - x_min) / (x_max - x_min) * pw);
                    int sy = mt + ph - int((vy - y_min) / (y_max - y_min) * ph);
                    return {sx, sy};
                };
                auto p1 = to_screen(s.data[i-1].first, s.data[i-1].second);
                auto p2 = to_screen(s.data[i].first, s.data[i].second);
                dc.DrawLine(p1, p2);
            }
            ++idx;
        }
    }
};

// ── PlotPanel ─────────────────────────────────────────────────────────

wxBEGIN_EVENT_TABLE(PlotPanel, wxPanel)
wxEND_EVENT_TABLE()

PlotPanel::PlotPanel(wxWindow* parent, AppState& state)
    : wxPanel(parent, wxID_ANY)
    , m_state(state)
    , m_colors(ThemeColors::resolve(state.theme))
{
    wxBoxSizer* sizer = new wxBoxSizer(wxVERTICAL);

    wxStaticText* title = new wxStaticText(this, wxID_ANY, "   Waveforms");
    wxFont tf = title->GetFont();
    tf.SetPointSize(tf.GetPointSize() + 2);
    tf.SetWeight(wxFONTWEIGHT_BOLD);
    title->SetFont(tf);
    sizer->Add(title, 0, wxEXPAND | wxALL, 8);

    m_plot_container = new wxPanel(this, wxID_ANY);
    m_plot_container->SetSizer(new wxBoxSizer(wxVERTICAL));
    sizer->Add(m_plot_container, 1, wxEXPAND | wxALL, 4);

    SetSizer(sizer);
}

void PlotPanel::UpdatePlots() {
    auto* ps = m_plot_container->GetSizer();
    ps->Clear(true);

    // Vout plot
    auto* vp = new SimplePlotWidget(m_plot_container);
    ps->Add(vp, 1, wxEXPAND | wxALL, 2);

    // IL plot
    auto* ip = new SimplePlotWidget(m_plot_container);
    ps->Add(ip, 1, wxEXPAND | wxALL, 2);

    // Generate data
    const auto& p = m_state.params;
    const auto& r = m_state.results;
    double f = p.frequency;
    double tp = (f > 0.0) ? 1.0 / f : 1e-5;
    int np = 200;
    double dt = tp * 3.0 / np;

    std::vector<std::pair<double, double>> vd, id;
    double duty = p.duty_cycle;
    double vout = r.vout;
    double iout = r.iout;
    double ilrip = r.il_ripple;

    for (int i = 0; i < np; ++i) {
        double t = i * dt;
        double ph = std::fmod(t / tp, 1.0);
        double vw = vout + ((ph < duty) ? 1.0 : -1.0) * r.vout_ripple * 0.5;
        vd.emplace_back(t * 1e6, vw);

        double il;
        if (ph < duty && duty > 0.0) {
            il = iout - ilrip / 2.0 + (ph / duty) * ilrip;
        } else {
            double off = (1.0 - duty);
            il = (off > 0.0) ? iout + ilrip / 2.0 - ((ph - duty) / off) * ilrip : iout;
        }
        id.emplace_back(t * 1e6, il);
    }

    vp->AddSeries(vd, wxColour(100, 200, 255), wxString::Format("Vout (%.2f V)", vout));
    ip->AddSeries(id, wxColour(255, 200, 100), wxString::Format("IL (%.2f A)", iout));

    m_plot_container->Layout();
}

void PlotPanel::OnPaint(wxPaintEvent&) {}
void PlotPanel::OnSize(wxSizeEvent& e) { e.Skip(); }
