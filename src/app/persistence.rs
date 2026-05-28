/// Persistence module for saving/loading project state.
///
/// Serializes the `AppState` to/from JSON files using the existing
/// serde derives already on `AppState`, `ConverterParams`, etc.

use crate::app::state::AppState;
use anyhow::{Context, Result};
use std::fs;

/// Default project file name
#[allow(dead_code)]
pub const DEFAULT_PROJECT_PATH: &str = "project.tupan.json";

/// Save the current project state to a JSON file.
pub fn save_project(path: &str, state: &AppState) -> Result<()> {
    let data = serde_json::to_string_pretty(state)
        .with_context(|| format!("failed to serialize project to {}", path))?;

    // Create backup of existing file if present
    if fs::metadata(path).is_ok() {
        let backup_path = format!("{}.bak", path);
        let _ = fs::copy(path, &backup_path);
    }

    fs::write(path, data)
        .with_context(|| format!("failed to write project to {}", path))?;

    Ok(())
}

/// Load a project state from a JSON file.
pub fn load_project(path: &str) -> Result<AppState> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read project from {}", path))?;

    let mut state: AppState = serde_json::from_str(&data)
        .with_context(|| format!("failed to parse project from {}", path))?;

    // After loading, recalculate to ensure results are consistent
    state.recalculate();

    Ok(state)
}

/// Export the current schematic as SVG to a file.
pub fn export_schematic_svg(
    path: &str,
    elements: &[crate::schematic::primitives::SchematicElement],
) -> Result<()> {
    let svg = crate::schematic::export_svg::export_svg(elements, 500.0, 300.0);
    fs::write(path, svg)
        .with_context(|| format!("failed to write SVG to {}", path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::AppState;
    use std::fs;

    #[test]
    fn test_save_load_roundtrip() {
        let path = "/tmp/test_tupan_project.json";

        // Clean up
        let _ = fs::remove_file(path);

        let state = AppState::default();

        // Save
        save_project(path, &state).expect("save should succeed");

        // Load
        let loaded = load_project(path).expect("load should succeed");

        // Verify
        assert_eq!(state.active_converter, loaded.active_converter);
        assert_eq!(state.params.vin, loaded.params.vin);
        assert_eq!(state.params.vout_target, loaded.params.vout_target);
        assert_eq!(state.params.frequency, loaded.params.frequency);

        // Clean up
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_project("/tmp/nonexistent_tupan_file.json");
        assert!(result.is_err());
    }
}
