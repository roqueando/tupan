#include "Theme.h"

ThemeColors ThemeColors::resolve(Theme theme) {
    if (theme == Theme::Dark) {
        return {
            wxColour(18, 18, 26),       // canvas_bg
            wxColour(60, 60, 80, 40),   // grid
            wxColour(22, 22, 32),       // sidebar_bg
            wxColour(160, 170, 200),    // section_title
            wxColour(25, 35, 60, 230),  // input_bg
            wxColour(99, 130, 255, 100),// input_border
            wxColour(40, 25, 15, 230),  // computed_bg
            wxColour(200, 140, 80, 100),// computed_border
            wxColour(220, 225, 240),    // text_primary
            wxColour(140, 150, 175),    // text_secondary
            wxColour(130, 190, 255),    // text_value
            wxColour(30, 35, 50, 180),  // card_bg
            wxColour(40, 48, 70, 200),  // card_hover
            wxColour(50, 65, 100, 230), // card_selected
            wxColour(140, 150, 175, 160), // status
            wxColour(20, 22, 32, 230)   // plot_bg
        };
    } else {
        return {
            wxColour(241, 239, 231),     // canvas_bg
            wxColour(0, 0, 0, 10),       // grid
            wxColour(255, 255, 255),     // sidebar_bg
            wxColour(80, 85, 110),       // section_title
            wxColour(230, 240, 255, 230),// input_bg
            wxColour(99, 130, 255, 100), // input_border
            wxColour(255, 240, 225, 230),// computed_bg
            wxColour(200, 140, 80, 100), // computed_border
            wxColour(30, 35, 50),        // text_primary
            wxColour(110, 115, 135),     // text_secondary
            wxColour(30, 100, 200),      // text_value
            wxColour(240, 242, 248, 180),// card_bg
            wxColour(225, 230, 245, 200),// card_hover
            wxColour(200, 215, 240, 230),// card_selected
            wxColour(110, 115, 135, 160),// status
            wxColour(245, 245, 250, 230) // plot_bg
        };
    }
}
