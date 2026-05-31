#include "ui/MainFrame.h"
#include "utils/Theme.h"
#include <wx/splitter.h>
#include <wx/stattext.h>

wxBEGIN_EVENT_TABLE(MainFrame, wxFrame)
    EVT_MENU(wxID_SAVE,       MainFrame::OnFileSave)
    EVT_MENU(wxID_OPEN,       MainFrame::OnFileLoad)
    EVT_MENU(2001,            MainFrame::OnFileExportSvg)
wxEND_EVENT_TABLE()

MainFrame::MainFrame(const wxString& title)
    : wxFrame(nullptr, wxID_ANY, title, wxDefaultPosition, wxSize(1200, 800))
{
    SetMinSize(wxSize(900, 600));

    wxMenu* fileMenu = new wxMenu();
    fileMenu->Append(wxID_OPEN, "&Open...\tCtrl+O");
    fileMenu->Append(wxID_SAVE, "&Save\tCtrl+S");
    fileMenu->AppendSeparator();
    fileMenu->Append(2001, "Export &SVG...");
    fileMenu->AppendSeparator();
    fileMenu->Append(wxID_EXIT, "E&xit\tCtrl+Q");
    wxMenuBar* menuBar = new wxMenuBar();
    menuBar->Append(fileMenu, "&File");
    SetMenuBar(menuBar);
    CreateStatusBar(1)->SetStatusText("Tupan ready");

    // Simply test: splitter + ParamPanel vs placeholder
    m_main_splitter = new wxSplitterWindow(this, wxID_ANY);
    m_main_splitter->SetMinimumPaneSize(200);

    m_param_panel = new ParamPanel(m_main_splitter, m_state);

    // Right side: SchematicPanel + placeholder
    wxPanel* rightPanel = new wxPanel(m_main_splitter, wxID_ANY);
    wxBoxSizer* rs = new wxBoxSizer(wxVERTICAL);

    m_schematic_panel = new SchematicPanel(rightPanel, m_state);
    rs->Add(m_schematic_panel, 1, wxEXPAND | wxALL, 2);

    m_plot_panel = new PlotPanel(rightPanel, m_state);
    rs->Add(m_plot_panel, 2, wxEXPAND | wxALL, 2);

    rightPanel->SetSizer(rs);

    m_main_splitter->SplitVertically(m_param_panel, rightPanel, 250);

    wxBoxSizer* mainSizer = new wxBoxSizer(wxVERTICAL);
    mainSizer->Add(m_main_splitter, 1, wxEXPAND);
    SetSizer(mainSizer);

    // Initial state
    m_state.recalculate();
    RefreshAll();
}

void MainFrame::OnFileSave(wxCommandEvent&) { GetStatusBar()->SetStatusText("Save"); }
void MainFrame::OnFileLoad(wxCommandEvent&) { GetStatusBar()->SetStatusText("Open"); }
void MainFrame::OnFileExportSvg(wxCommandEvent&) { GetStatusBar()->SetStatusText("Export"); }
void MainFrame::OnParamChanged(wxCommandEvent&) {}
void MainFrame::OnConverterChanged(wxCommandEvent&) {}
void MainFrame::RefreshAll() {
    if (m_param_panel)     { m_param_panel->Refresh(); }
    if (m_schematic_panel) { m_schematic_panel->UpdateSchematic(); }
    if (m_plot_panel)      { m_plot_panel->UpdatePlots(); }
    if (m_result_panel)    { m_result_panel->UpdateDisplay(); }
}
