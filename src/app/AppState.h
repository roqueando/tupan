#pragma once

#include "domain/Types.h"
#include <string>

class AppState {
public:
    CanvasState canvas;
    Theme theme = Theme::Dark;
    std::string status_message = "ready — place components on the canvas";
};
