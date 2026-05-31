#include "ResultPanel.h"
#include "utils/Formatting.h"

#include <wx/statbox.h>
#include <wx/statline.h>
#include <wx/colour.h>

ResultPanel::ResultPanel(wxWindow* parent, AppState& state)
    : wxPanel(parent, wxID_ANY)
    , m_state(state)
    , m_colors(ThemeColors::resolve(state.theme))
{
    SetBackgroundColour(m_colors.sidebar_bg);

    wxBoxSizer* mainSizer = new wxBoxSizer(wxVERTICAL);

    // ── Title ──
    wxStaticText* title = new wxStaticText(this, wxID_ANY, "   📊 Results");
    wxFont titleFont = title->GetFont();
    titleFont.SetPointSize(titleFont.GetPointSize() + 2);
    titleFont.SetWeight(wxFONTWEIGHT_BOLD);
    title->SetFont(titleFont);
    title->SetForegroundColour(m_colors.text_primary);
    mainSizer->Add(title, 0, wxEXPAND | wxALL, 8);
    mainSizer->Add(new wxStaticLine(this), 0, wxEXPAND | wxLEFT | wxRIGHT, 8);

    // ── Voltage / Current ──
    {
        wxStaticBoxSizer* box = new wxStaticBoxSizer(wxVERTICAL, this, "Voltage / Current");
        m_vout_text = new wxStaticText(this, wxID_ANY, "Vout = 0 V");
        m_iout_text = new wxStaticText(this, wxID_ANY, "Iout = 0 A");
        m_iin_text = new wxStaticText(this, wxID_ANY, "Iin = 0 A");
        m_vrms_text = new wxStaticText(this, wxID_ANY, "");
        m_v1_text = new wxStaticText(this, wxID_ANY, "");

        for (auto* t : {m_vout_text, m_iout_text, m_iin_text, m_vrms_text, m_v1_text})
            t->SetForegroundColour(m_colors.text_value);
        for (auto* t : {m_vout_text, m_iout_text, m_iin_text, m_vrms_text, m_v1_text})
            box->Add(t, 0, wxEXPAND | wxALL, 4);

        mainSizer->Add(box, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(4);

    // ── Ripple ──
    {
        wxStaticBoxSizer* box = new wxStaticBoxSizer(wxVERTICAL, this, "Ripple");
        m_vout_ripple_text = new wxStaticText(this, wxID_ANY, "Vout ripple = 0 V");
        m_il_ripple_text = new wxStaticText(this, wxID_ANY, "iL ripple = 0 A");
        m_vout_ripple_text->SetForegroundColour(m_colors.text_value);
        m_il_ripple_text->SetForegroundColour(m_colors.text_value);
        box->Add(m_vout_ripple_text, 0, wxEXPAND | wxALL, 4);
        box->Add(m_il_ripple_text, 0, wxEXPAND | wxALL, 4);
        mainSizer->Add(box, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(4);

    // ── Losses & Efficiency ──
    {
        wxStaticBoxSizer* box = new wxStaticBoxSizer(wxVERTICAL, this, "Losses & Efficiency");
        m_cond_loss_text = new wxStaticText(this, wxID_ANY, "Conduction = 0 W");
        m_sw_loss_text = new wxStaticText(this, wxID_ANY, "Switching = 0 W");
        m_total_loss_text = new wxStaticText(this, wxID_ANY, "Total = 0 W");
        m_efficiency_text = new wxStaticText(this, wxID_ANY, "Efficiency = 0%");

        for (auto* t : {m_cond_loss_text, m_sw_loss_text, m_total_loss_text, m_efficiency_text})
            t->SetForegroundColour(m_colors.text_value);
        for (auto* t : {m_cond_loss_text, m_sw_loss_text, m_total_loss_text, m_efficiency_text})
            box->Add(t, 0, wxEXPAND | wxALL, 4);

        mainSizer->Add(box, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(4);

    // ── Harmonics (VSI) ──
    {
        wxStaticBoxSizer* box = new wxStaticBoxSizer(wxVERTICAL, this, "Harmonics");
        m_thd_text = new wxStaticText(this, wxID_ANY, "");
        m_thd_text->SetForegroundColour(m_colors.text_value);
        box->Add(m_thd_text, 0, wxEXPAND | wxALL, 4);
        mainSizer->Add(box, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(8);
    mainSizer->Add(new wxStaticLine(this), 0, wxEXPAND | wxLEFT | wxRIGHT, 8);

    // ── Status ──
    m_status_text = new wxStaticText(this, wxID_ANY, "Status: ready");
    m_status_text->SetForegroundColour(m_colors.status);
    mainSizer->Add(m_status_text, 0, wxEXPAND | wxALL, 8);

    mainSizer->AddStretchSpacer();

    SetSizer(mainSizer);
    UpdateDisplay();
}

void ResultPanel::UpdateDisplay() {
    m_colors = ThemeColors::resolve(m_state.theme);
    SetBackgroundColour(m_colors.sidebar_bg);

    const auto& r = m_state.results;

    m_vout_text->SetLabel("Vout = " + formatting::format_value(r.vout, "V"));
    m_iout_text->SetLabel("Iout = " + formatting::format_value(r.iout, "A"));
    m_iin_text->SetLabel("Iin = " + formatting::format_value(r.iin, "A"));

    if (r.rms_output) {
        m_vrms_text->SetLabel("Vrms = " + formatting::format_value(*r.rms_output, "V"));
        m_vrms_text->Show();
    } else {
        m_vrms_text->Hide();
    }

    if (r.fundamental_amplitude) {
        m_v1_text->SetLabel("V1 (fund) = " + formatting::format_value(*r.fundamental_amplitude, "V"));
        m_v1_text->Show();
    } else {
        m_v1_text->Hide();
    }

    m_vout_ripple_text->SetLabel("Vout ripple = " + formatting::format_value(r.vout_ripple, "V"));
    m_il_ripple_text->SetLabel("iL ripple = " + formatting::format_value(r.il_ripple, "A"));

    m_cond_loss_text->SetLabel("Conduction = " + formatting::format_value(r.conduction_losses, "W"));
    m_sw_loss_text->SetLabel("Switching = " + formatting::format_value(r.switching_losses, "W"));
    m_total_loss_text->SetLabel("Total = " + formatting::format_value(r.conduction_losses + r.switching_losses, "W"));

    // Efficiency with color coding
    double eff = r.efficiency;
    wxColour eff_color;
    if (eff > 0.95)       eff_color = wxColour(0, 200, 0);    // Green
    else if (eff > 0.85) eff_color = wxColour(200, 200, 0);  // Yellow
    else                  eff_color = wxColour(200, 0, 0);    // Red

    m_efficiency_text->SetLabel(wxString::Format("Efficiency = %.1f%%", eff * 100.0));
    m_efficiency_text->SetForegroundColour(eff_color);

    // THD
    if (r.thd) {
        double thd_val = *r.thd;
        wxColour thd_color;
        if (thd_val < 0.5)       thd_color = wxColour(0, 200, 0);
        else if (thd_val < 1.0) thd_color = wxColour(200, 200, 0);
        else                     thd_color = wxColour(200, 0, 0);
        m_thd_text->SetLabel(wxString::Format("THD = %.1f%%", thd_val * 100.0));
        m_thd_text->SetForegroundColour(thd_color);
        m_thd_text->Show();
    } else {
        m_thd_text->Hide();
    }

    m_status_text->SetLabel("Status: " + m_state.status_message);
    m_status_text->SetForegroundColour(m_colors.status);

    // Refresh text colors for all visible labels
    for (wxWindow* child : GetChildren()) {
        wxStaticText* st = dynamic_cast<wxStaticText*>(child);
        if (st && st != m_efficiency_text && st != m_thd_text && st != m_status_text) {
            st->SetForegroundColour(m_colors.text_value);
        }
    }
}
