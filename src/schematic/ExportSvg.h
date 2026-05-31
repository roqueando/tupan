#pragma once

#include "schematic/Primitives.h"
#include <string>
#include <vector>

namespace export_svg {

/// Export a list of schematic elements to an SVG string.
std::string export_svg(const std::vector<SchematicElement>& elements, float width, float height);

} // namespace export_svg
