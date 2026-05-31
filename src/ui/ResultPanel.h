#pragma once

#include "app/AppState.h"
#include "utils/Theme.h"

#include <wx/panel.h>
#include <wx/stattext.h>
#include <wx/sizer.h>
#include <wx/scrolwin.h>

class ResultPanel : public wxPanel {
public:
    ResultPanel(wxWindow* parent, AppState& state);
    virtual ~ResultPanel() = default;

    void UpdateDisplay();

private:
    AppState& m_state;
    ThemeColors m_colors;

    // Result display fields
    wxStaticText* m_vout_text = nullptr;
    wxStaticText* m_iout_text = nullptr;
    wxStaticText* m_iin_text = nullptr;
    wxStaticText* m_vout_ripple_text = nullptr;
    wxStaticText* m_il_ripple_text = nullptr;
    wxStaticText* m_cond_loss_text = nullptr;
    wxStaticText* m_sw_loss_text = nullptr;
    wxStaticText* m_total_loss_text = nullptr;
    wxStaticText* m_efficiency_text = nullptr;
    wxStaticText* m_thd_text = nullptr;
    wxStaticText* m_vrms_text = nullptr;
    wxStaticText* m_v1_text = nullptr;
    wxStaticText* m_status_text = nullptr;
};
