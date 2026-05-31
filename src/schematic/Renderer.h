#pragma once

#include "schematic/Primitives.h"
#include <vector>

// Forward declare wxWidgets types
class wxDC;

namespace renderer {

/// Draw a single schematic element on a wxDC.
void draw_element(wxDC& dc, const SchematicElement& element, float origin_x, float origin_y, bool highlight = false);

/// Draw all elements in a list.
void draw_all(wxDC& dc, const std::vector<SchematicElement>& elements, float origin_x, float origin_y);

} // namespace renderer
