// Tupan — Interactive Power Electronics Workbench
// C++ wxWidgets port

#include "ui/MainFrame.h"
#include <wx/app.h>

class TupanApp : public wxApp {
public:
    virtual bool OnInit() override {
        MainFrame* frame = new MainFrame("Tupan — Power Electronics Workbench");
        frame->Show(true);
        frame->Raise();
        return true;
    }
};

wxIMPLEMENT_APP(TupanApp);
