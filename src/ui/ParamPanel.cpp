#include "ParamPanel.h"

#include <wx/sizer.h>
#include <wx/statbox.h>
#include <wx/statline.h>

ParamPanel::ParamPanel(wxWindow* parent, AppState& state)
    : wxPanel(parent, wxID_ANY)
    , m_state(state)
    , m_colors(ThemeColors::resolve(state.theme))
{
    SetBackgroundColour(m_colors.sidebar_bg);

    wxBoxSizer* mainSizer = new wxBoxSizer(wxVERTICAL);

    // ── Title ──
    wxStaticText* title = new wxStaticText(this, wxID_ANY, "   ⚙ Parameters");
    wxFont titleFont = title->GetFont();
    titleFont.SetPointSize(titleFont.GetPointSize() + 2);
    titleFont.SetWeight(wxFONTWEIGHT_BOLD);
    title->SetFont(titleFont);
    title->SetForegroundColour(m_colors.text_primary);
    mainSizer->Add(title, 0, wxEXPAND | wxALL, 8);

    mainSizer->Add(new wxStaticLine(this), 0, wxEXPAND | wxLEFT | wxRIGHT, 8);

    // ── Converter Selector ──
    mainSizer->AddSpacer(8);
    wxStaticText* convLabel = new wxStaticText(this, wxID_ANY, "   Converter:");
    convLabel->SetForegroundColour(m_colors.text_secondary);
    mainSizer->Add(convLabel, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);

    wxArrayString choices;
    choices.Add("Buck Converter");
    choices.Add("Boost Converter");
    choices.Add("VSI Single-Phase");
    m_converter_choice = new wxChoice(this, wxID_ANY, wxDefaultPosition, wxDefaultSize, choices);
    m_converter_choice->SetSelection(0);
    m_converter_choice->Bind(wxEVT_CHOICE, &ParamPanel::OnConverterSelect, this);
    mainSizer->Add(m_converter_choice, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);

    mainSizer->AddSpacer(8);
    mainSizer->Add(new wxStaticLine(this), 0, wxEXPAND | wxLEFT | wxRIGHT, 8);

    // ── Input Section ──
    {
        wxStaticBoxSizer* inputBox = new wxStaticBoxSizer(wxVERTICAL, this, "Input");
        m_vin_ctrl = AddParamRow(inputBox, "Vin (V)", m_state.params.vin, 1.0, 500.0, 1.0, " V", &m_vin_label);
        m_vout_target_ctrl = AddParamRow(inputBox, "Vout target (V)", m_state.params.vout_target, 0.5, 500.0, 1.0, " V", &m_vout_label);
        mainSizer->Add(inputBox, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(4);

    // ── Switching Section ──
    {
        wxStaticBoxSizer* swBox = new wxStaticBoxSizer(wxVERTICAL, this, "Switching");
        m_freq_ctrl = AddParamRow(swBox, "Freq (Hz)", m_state.params.frequency, 100.0, 1'000'000.0, 1000.0, " Hz", &m_freq_label);
        m_duty_ctrl = AddParamRow(swBox, "Duty (%)", m_state.params.duty_cycle * 100.0, 1.0, 99.0, 0.5, " %", &m_duty_label);
        mainSizer->Add(swBox, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(4);

    // ── Components Section ──
    {
        wxStaticBoxSizer* compBox = new wxStaticBoxSizer(wxVERTICAL, this, "Components");
        m_inductance_ctrl = AddParamRow(compBox, "L (H)", m_state.params.inductance, 1e-6, 100e-3, 1e-6, " H", &m_inductance_label);
        m_capacitance_ctrl = AddParamRow(compBox, "C (F)", m_state.params.capacitance, 1e-9, 100e-3, 1e-9, " F", &m_capacitance_label);
        m_load_ctrl = AddParamRow(compBox, "R (Ω)", m_state.params.load_resistance, 0.1, 1000.0, 0.1, " Ω", &m_load_label);
        mainSizer->Add(compBox, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(4);

    // ── Inverter Section (VSI-specific) ──
    {
        wxStaticBoxSizer* invBox = new wxStaticBoxSizer(wxVERTICAL, this, "Inverter");
        m_mod_index_ctrl = AddParamRow(invBox, "Mod. index", m_state.params.modulation_index, 0.01, 1.0, 0.01, "", &m_mod_index_label);
        m_out_freq_ctrl = AddParamRow(invBox, "Out freq (Hz)", m_state.params.output_frequency, 1.0, 1000.0, 1.0, " Hz", &m_out_freq_label);
        mainSizer->Add(invBox, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddSpacer(4);

    // ── Numerical Simulation Toggle ──
    m_sim_checkbox = new wxCheckBox(this, wxID_ANY, " Numerical Simulation");
    m_sim_checkbox->SetValue(m_state.show_numerical_sim);
    m_sim_checkbox->SetForegroundColour(m_colors.text_primary);
    m_sim_checkbox->Bind(wxEVT_CHECKBOX, &ParamPanel::OnAnyParamChanged, this);
    mainSizer->Add(m_sim_checkbox, 0, wxEXPAND | wxLEFT | wxRIGHT, 12);

    mainSizer->AddSpacer(8);
    mainSizer->Add(new wxStaticLine(this), 0, wxEXPAND | wxLEFT | wxRIGHT, 8);

    // ── Computed Results ──
    {
        wxStaticBoxSizer* resultsBox = new wxStaticBoxSizer(wxVERTICAL, this, "Computed");
        m_inductance_result = new wxStaticText(this, wxID_ANY, "L = 0 H");
        m_inductance_result->SetForegroundColour(m_colors.text_value);
        resultsBox->Add(m_inductance_result, 0, wxEXPAND | wxALL, 4);

        m_capacitance_result = new wxStaticText(this, wxID_ANY, "C = 0 F");
        m_capacitance_result->SetForegroundColour(m_colors.text_value);
        resultsBox->Add(m_capacitance_result, 0, wxEXPAND | wxALL, 4);

        m_delta_il_amps = new wxStaticText(this, wxID_ANY, "ΔiL = 0 A");
        m_delta_il_amps->SetForegroundColour(m_colors.text_value);
        resultsBox->Add(m_delta_il_amps, 0, wxEXPAND | wxALL, 4);

        mainSizer->Add(resultsBox, 0, wxEXPAND | wxLEFT | wxRIGHT, 8);
    }

    mainSizer->AddStretchSpacer();

    SetSizer(mainSizer);

    // Update UI to reflect initial state
    UpdateControls();
}

wxSpinCtrlDouble* ParamPanel::AddParamRow(wxSizer* sizer, const wxString& label,
                                           double value, double min, double max, double inc,
                                           const wxString& suffix, wxStaticText** label_out)
{
    wxBoxSizer* rowSizer = new wxBoxSizer(wxHORIZONTAL);

    wxStaticText* lbl = new wxStaticText(this, wxID_ANY, "  " + label);
    lbl->SetForegroundColour(m_colors.text_secondary);
    wxFont lblFont = lbl->GetFont();
    lblFont.SetPointSize(lblFont.GetPointSize() - 1);
    lbl->SetFont(lblFont);

    if (label_out) *label_out = lbl;

    wxSpinCtrlDouble* spin = new wxSpinCtrlDouble(this, wxID_ANY, wxEmptyString,
                                                    wxDefaultPosition, wxDefaultSize,
                                                    wxSP_ARROW_KEYS, min, max, value, inc);
    spin->SetForegroundColour(m_colors.text_value);
    spin->SetBackgroundColour(m_colors.input_bg);

    rowSizer->Add(lbl, 1, wxALIGN_CENTER_VERTICAL | wxLEFT, 4);
    rowSizer->Add(spin, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 4);

    sizer->Add(rowSizer, 0, wxEXPAND | wxALL, 2);

    spin->Bind(wxEVT_SPINCTRLDOUBLE, &ParamPanel::OnAnyParamChanged, this);

    return spin;
}

void ParamPanel::OnConverterSelect(wxCommandEvent&) {
    int sel = m_converter_choice->GetSelection();
    ConverterType new_type;
    switch (sel) {
        case 0: new_type = ConverterType::Buck; break;
        case 1: new_type = ConverterType::Boost; break;
        default: new_type = ConverterType::VsiSinglePhase; break;
    }

    if (new_type != m_state.active_converter) {
        m_state.active_converter = new_type;
        // Set default params for converter type
        switch (new_type) {
            case ConverterType::Buck:
                m_state.params.vin = 48.0;
                m_state.params.vout_target = 12.0;
                m_state.params.frequency = 100'000.0;
                m_state.params.inductance = 100e-6;
                m_state.params.capacitance = 100e-6;
                m_state.params.load_resistance = 10.0;
                break;
            case ConverterType::Boost:
                m_state.params.vin = 12.0;
                m_state.params.vout_target = 24.0;
                m_state.params.frequency = 100'000.0;
                m_state.params.inductance = 100e-6;
                m_state.params.capacitance = 100e-6;
                m_state.params.load_resistance = 10.0;
                break;
            case ConverterType::VsiSinglePhase:
                m_state.params.vin = 300.0;
                m_state.params.vout_target = 240.0;
                m_state.params.frequency = 10'000.0;
                m_state.params.inductance = 1e-3;
                m_state.params.capacitance = 10e-6;
                m_state.params.load_resistance = 10.0;
                m_state.params.modulation_index = 0.8;
                m_state.params.output_frequency = 60.0;
                break;
        }
        m_state.recalculate();
        UpdateControls();

        // Notify parent
        wxCommandEvent evt(wxEVT_COMMAND_TOOL_CLICKED, 1002);
        GetParent()->GetEventHandler()->ProcessEvent(evt);
    }
}

void ParamPanel::OnAnyParamChanged(wxCommandEvent&) {
    // Read values from controls into state
    m_state.params.vin = m_vin_ctrl->GetValue();
    m_state.params.vout_target = m_vout_target_ctrl->GetValue();
    m_state.params.frequency = m_freq_ctrl->GetValue();
    m_state.params.duty_cycle = m_duty_ctrl->GetValue() / 100.0;
    m_state.params.inductance = m_inductance_ctrl->GetValue();
    m_state.params.capacitance = m_capacitance_ctrl->GetValue();
    m_state.params.load_resistance = m_load_ctrl->GetValue();
    m_state.params.modulation_index = m_mod_index_ctrl->GetValue();
    m_state.params.output_frequency = m_out_freq_ctrl->GetValue();
    m_state.show_numerical_sim = m_sim_checkbox->GetValue();

    m_state.recalculate();
    UpdateControls();

    // Forward event to parent
    wxCommandEvent evt(wxEVT_COMMAND_TOOL_CLICKED, 1002);
    GetParent()->GetEventHandler()->ProcessEvent(evt);
}

void ParamPanel::UpdateControls() {
    m_colors = ThemeColors::resolve(m_state.theme);
    SetBackgroundColour(m_colors.sidebar_bg);

    // Update choice selection
    int sel = 0;
    switch (m_state.active_converter) {
        case ConverterType::Buck:           sel = 0; break;
        case ConverterType::Boost:          sel = 1; break;
        case ConverterType::VsiSinglePhase: sel = 2; break;
    }
    m_converter_choice->SetSelection(sel);

    // Update spin controls without firing events
    m_vin_ctrl->SetValue(m_state.params.vin);
    m_vout_target_ctrl->SetValue(m_state.params.vout_target);
    m_freq_ctrl->SetValue(m_state.params.frequency);
    m_duty_ctrl->SetValue(m_state.params.duty_cycle * 100.0);
    m_inductance_ctrl->SetValue(m_state.params.inductance);
    m_capacitance_ctrl->SetValue(m_state.params.capacitance);
    m_load_ctrl->SetValue(m_state.params.load_resistance);
    m_mod_index_ctrl->SetValue(m_state.params.modulation_index);
    m_out_freq_ctrl->SetValue(m_state.params.output_frequency);
    m_sim_checkbox->SetValue(m_state.show_numerical_sim);

    // Update computed results
    auto sp = m_state.get_component_values();
    m_inductance_result->SetLabel("L = " + sp.inductance);
    m_capacitance_result->SetLabel("C = " + sp.capacitance);

    // Delta iL in Amps (placeholder - actual calculation from SharedParams equivalent)
    double dil = m_state.params.duty_cycle * m_state.params.vin / m_state.params.inductance / m_state.params.frequency;
    m_delta_il_amps->SetLabel(wxString::Format("ΔiL = %.4f A", dil));

    // Update all text colors
    m_vin_label->SetForegroundColour(m_colors.text_secondary);
    m_vout_label->SetForegroundColour(m_colors.text_secondary);
    m_freq_label->SetForegroundColour(m_colors.text_secondary);
    m_duty_label->SetForegroundColour(m_colors.text_secondary);
    m_inductance_label->SetForegroundColour(m_colors.text_secondary);
    m_capacitance_label->SetForegroundColour(m_colors.text_secondary);
    m_load_label->SetForegroundColour(m_colors.text_secondary);
    m_mod_index_label->SetForegroundColour(m_colors.text_secondary);
    m_out_freq_label->SetForegroundColour(m_colors.text_secondary);
}
