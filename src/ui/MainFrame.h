#pragma once

#include "app/AppState.h"
#include "app/Persistence.h"
#include "ui/CanvasPanel.h"
#include <wx/frame.h>
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
    CanvasPanel* m_canvas = nullptr;

    void OnFileSave(wxCommandEvent&);
    void OnFileLoad(wxCommandEvent&);
    void OnThemeToggle(wxCommandEvent&);

    wxDECLARE_EVENT_TABLE();
};
