#pragma once

#include "app/AppState.h"
#include "app/Persistence.h"
#include "ui/ParamPanel.h"
#include "ui/ResultPanel.h"
#include "ui/SchematicPanel.h"
#include "ui/PlotPanel.h"

#include <wx/frame.h>
#include <wx/panel.h>
#include <wx/sizer.h>
#include <wx/splitter.h>
#include <wx/statusbr.h>
#include <wx/menu.h>
#include <wx/msgdlg.h>
#include <wx/filedlg.h>

class MainFrame : public wxFrame {
public:
    MainFrame(const wxString& title);
    virtual ~MainFrame() = default;

private:
    AppState m_state;

    wxSplitterWindow* m_main_splitter = nullptr;
    ParamPanel* m_param_panel = nullptr;
    SchematicPanel* m_schematic_panel = nullptr;
    PlotPanel* m_plot_panel = nullptr;
    ResultPanel* m_result_panel = nullptr;

    void OnFileSave(wxCommandEvent&);
    void OnFileLoad(wxCommandEvent&);
    void OnFileExportSvg(wxCommandEvent&);
    void OnParamChanged(wxCommandEvent&);
    void OnConverterChanged(wxCommandEvent&);
    void RefreshAll();

    wxDECLARE_EVENT_TABLE();
};
