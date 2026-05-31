#pragma once

#include "domain/Types.h"
#include "schematic/Primitives.h"
#include <vector>

namespace layout {

/// Generate schematic elements for the given converter type.
std::vector<SchematicElement> generate_schematic(
    ConverterType converter_type,
    const ComponentValues& values);

} // namespace layout
