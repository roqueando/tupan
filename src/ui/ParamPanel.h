#pragma once

#include "app/AppState.h"
#include "domain/Types.h"
#include "utils/Theme.h"

#include <wx/panel.h>
#include <wx/slider.h>
#include <wx/spinctrl.h>
#include <wx/stattext.h>
#include <wx/choice.h>
#include <wx/checkbox.h>
#include <wx/button.h>
#include <wx/scrolwin.h>

class ParamPanel : public wxPanel {
public:
    ParamPanel(wxWindow* parent, AppState& state);
    virtual ~ParamPanel() = default;

    void UpdateControls();

private:
    AppState& m_state;
    ThemeColors m_colors;

    // Converter selector
    wxChoice* m_converter_choice = nullptr;

    // Parameter controls
    wxSpinCtrlDouble* m_vin_ctrl = nullptr;
    wxSpinCtrlDouble* m_vout_target_ctrl = nullptr;
    wxSpinCtrlDouble* m_freq_ctrl = nullptr;
    wxSpinCtrlDouble* m_duty_ctrl = nullptr;
    wxSpinCtrlDouble* m_inductance_ctrl = nullptr;
    wxSpinCtrlDouble* m_capacitance_ctrl = nullptr;
    wxSpinCtrlDouble* m_load_ctrl = nullptr;
    wxSpinCtrlDouble* m_mod_index_ctrl = nullptr;
    wxSpinCtrlDouble* m_out_freq_ctrl = nullptr;
    wxCheckBox* m_sim_checkbox = nullptr;

    // Static text displays
    wxStaticText* m_vin_label = nullptr;
    wxStaticText* m_vout_label = nullptr;
    wxStaticText* m_freq_label = nullptr;
    wxStaticText* m_duty_label = nullptr;
    wxStaticText* m_inductance_label = nullptr;
    wxStaticText* m_capacitance_label = nullptr;
    wxStaticText* m_load_label = nullptr;
    wxStaticText* m_mod_index_label = nullptr;
    wxStaticText* m_out_freq_label = nullptr;

    // Computed results display
    wxStaticText* m_inductance_result = nullptr;
    wxStaticText* m_capacitance_result = nullptr;
    wxStaticText* m_delta_il_amps = nullptr;

    void OnConverterSelect(wxCommandEvent&);
    void OnAnyParamChanged(wxCommandEvent&);

    wxSpinCtrlDouble* AddParamRow(wxSizer* sizer, const wxString& label,
                                  double value, double min, double max, double inc,
                                  const wxString& suffix, wxStaticText** label_out = nullptr);
};
