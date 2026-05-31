#include "ui/MainFrame.h"
#include <wx/sizer.h>

wxBEGIN_EVENT_TABLE(MainFrame, wxFrame)
    EVT_MENU(wxID_SAVE,  MainFrame::OnFileSave)
    EVT_MENU(wxID_OPEN,  MainFrame::OnFileLoad)
    EVT_MENU(2001,       MainFrame::OnThemeToggle)
wxEND_EVENT_TABLE()

MainFrame::MainFrame(const wxString& title)
    : wxFrame(nullptr, wxID_ANY, title, wxDefaultPosition, wxSize(1200, 800))
{
    SetMinSize(wxSize(900, 600));

    // Menu
    wxMenuBar* mb = new wxMenuBar();
    wxMenu* fm = new wxMenu();
    fm->Append(wxID_OPEN, "&Open...\tCtrl+O");
    fm->Append(wxID_SAVE, "&Save\tCtrl+S");
    fm->AppendSeparator();
    fm->Append(2001, "Toggle &Theme\tCtrl+T");
    fm->AppendSeparator();
    fm->Append(wxID_EXIT, "E&xit\tCtrl+Q");
    mb->Append(fm, "&File");
    SetMenuBar(mb);

    CreateStatusBar(1)->SetStatusText(m_state.status_message.c_str());

    // Main content: just the CanvasPanel
    m_canvas = new CanvasPanel(this, m_state);
    wxBoxSizer* s = new wxBoxSizer(wxVERTICAL);
    s->Add(m_canvas, 1, wxEXPAND);
    SetSizer(s);
}

void MainFrame::OnFileSave(wxCommandEvent&) {
    wxFileDialog dlg(this, "Save Project", "", "project.tupan.json",
                     "*.tupan.json", wxFD_SAVE | wxFD_OVERWRITE_PROMPT);
    if (dlg.ShowModal() == wxID_OK) {
        if (persistence::save_project(dlg.GetPath().ToStdString(), m_state))
            GetStatusBar()->SetStatusText("Project saved");
        else
            wxMessageBox("Failed to save", "Error", wxOK | wxICON_ERROR);
    }
}

void MainFrame::OnFileLoad(wxCommandEvent&) {
    wxFileDialog dlg(this, "Open Project", "", "",
                     "*.tupan.json", wxFD_OPEN | wxFD_FILE_MUST_EXIST);
    if (dlg.ShowModal() == wxID_OK) {
        if (persistence::load_project(dlg.GetPath().ToStdString(), m_state)) {
            m_canvas->RefreshCanvas();
            GetStatusBar()->SetStatusText("Project loaded");
        } else {
            wxMessageBox("Failed to load", "Error", wxOK | wxICON_ERROR);
        }
    }
}

void MainFrame::OnThemeToggle(wxCommandEvent&) {
    m_state.theme = (m_state.theme == Theme::Dark) ? Theme::Light : Theme::Dark;
    m_canvas->RefreshCanvas();
    GetStatusBar()->SetStatusText(m_state.theme == Theme::Dark ? "Dark theme" : "Light theme");
}
