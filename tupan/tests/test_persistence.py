"""Tests for persistence module."""

import os
import tempfile
from tupan.app.state import AppState, Theme
from tupan.app.persistence import (
    save_project, load_project, state_to_dict, dict_to_state
)
from tupan.domain import ConverterType
from tupan.domain.design_params import DesignParams


def test_save_load_roundtrip():
    """Verify state survives a save/load cycle."""
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".json", delete=False
    ) as f:
        path = f.name

    try:
        state = AppState()
        state.theme = Theme.Light
        state.design.vin = 24.0
        state.design.vout = 48.0
        state.design.frequency = 200_000.0
        state.recalculate()

        save_project(path, state)
        loaded = load_project(path)

        assert loaded.theme == state.theme
        assert loaded.design.vin == state.design.vin
        assert loaded.design.vout == state.design.vout
        assert loaded.design.frequency == state.design.frequency
    finally:
        os.unlink(path)


def test_load_nonexistent_file():
    """Loading a non-existent file should raise an error."""
    try:
        load_project("/tmp/nonexistent_tupan_file_xyz.json")
        assert False, "Should have raised an exception"
    except (FileNotFoundError, OSError):
        pass


def test_state_to_dict_roundtrip():
    """Verify state serialization/deserialization works."""
    state = AppState()
    state.theme = Theme.Light
    state.design.vin = 48.0
    state.design.vout = 12.0
    state.recalculate()

    d = state_to_dict(state)
    assert d["theme"] == "Light"
    assert d["design"]["vin"] == 48.0

    restored = dict_to_state(d)
    assert restored.theme == state.theme
    assert restored.design.vin == state.design.vin
    assert restored.results.efficiency > 0.9
