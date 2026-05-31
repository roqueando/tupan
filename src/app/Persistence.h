#pragma once

#include "app/AppState.h"
#include "schematic/Primitives.h"
#include <string>
#include <vector>

namespace persistence {

/// Save the current project state to a JSON file.
bool save_project(const std::string& path, const AppState& state);

/// Load a project state from a JSON file.
bool load_project(const std::string& path, AppState& state);

/// Export schematic as SVG to a file.
bool export_schematic_svg(const std::string& path,
                          const std::vector<SchematicElement>& elements,
                          float width = 500.0f,
                          float height = 300.0f);

} // namespace persistence
