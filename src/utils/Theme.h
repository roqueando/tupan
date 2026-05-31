#pragma once

#include "domain/Types.h"  // for Theme enum
#include <wx/colour.h>

/// ThemeColors struct: resolved per theme, provides all UI colors.
struct ThemeColors {
    wxColour canvas_bg;
    wxColour grid;
    wxColour sidebar_bg;
    wxColour section_title;
    wxColour input_bg;
    wxColour input_border;
    wxColour computed_bg;
    wxColour computed_border;
    wxColour text_primary;
    wxColour text_secondary;
    wxColour text_value;
    wxColour card_bg;
    wxColour card_hover;
    wxColour card_selected;
    wxColour status;
    wxColour plot_bg;

    // Accent colors (not theme-dependent)
    static wxColour accent()       { return wxColour(99, 130, 255); }
    static wxColour accent_light() { return wxColour(130, 160, 255); }
    static wxColour accent_dim()   { return wxColour(60, 90, 200); }
    static wxColour selected()     { return wxColour(255, 210, 60); }

    static ThemeColors resolve(Theme theme);
};
