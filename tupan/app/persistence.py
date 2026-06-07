"""Persistence module for saving/loading project state."""

import json
import os
from tupan.app.state import AppState
from tupan.domain import ConverterResults


def state_to_dict(state: AppState) -> dict:
    """Convert AppState to a JSON-serializable dict."""
    return {
        "status_message": state.status_message,
        "show_numerical_sim": state.show_numerical_sim,
        "design": {
            "vin": state.design.vin,
            "vout": state.design.vout,
            "duty_cycle": state.design.duty_cycle,
            "frequency": state.design.frequency,
            "iout_max": state.design.iout_max,
            "delta_il_pct": state.design.delta_il_pct,
            "delta_vo_pct": state.design.delta_vo_pct,
        },
        "results": {
            "vout": state.results.vout,
            "iout": state.results.iout,
            "iin": state.results.iin,
            "vout_ripple": state.results.vout_ripple,
            "il_ripple": state.results.il_ripple,
            "conduction_losses": state.results.conduction_losses,
            "switching_losses": state.results.switching_losses,
            "efficiency": state.results.efficiency,
        },
    }


def dict_to_state(d: dict) -> AppState:
    """Restore AppState from a JSON-deserialized dict."""
    state = AppState()

    state.status_message = d.get("status_message", "Loaded")
    state.show_numerical_sim = d.get("show_numerical_sim", False)

    des = d.get("design", {})
    state.design.vin = des.get("vin", 48.0)
    state.design.vout = des.get("vout", 12.0)
    state.design.duty_cycle = des.get("duty_cycle", 0.25)
    state.design.frequency = des.get("frequency", 100_000.0)
    state.design.iout_max = des.get("iout_max", 5.0)
    state.design.delta_il_pct = des.get("delta_il_pct", 0.30)
    state.design.delta_vo_pct = des.get("delta_vo_pct", 0.01)

    # Recompute from saved design params
    state.recalculate()

    # Optionally override results from saved values
    res = d.get("results", {})
    if res:
        state.results.vout = res.get("vout", state.results.vout)
        state.results.iout = res.get("iout", state.results.iout)
        state.results.iin = res.get("iin", state.results.iin)
        state.results.vout_ripple = res.get("vout_ripple", state.results.vout_ripple)
        state.results.il_ripple = res.get("il_ripple", state.results.il_ripple)
        state.results.conduction_losses = res.get("conduction_losses", state.results.conduction_losses)
        state.results.switching_losses = res.get("switching_losses", state.results.switching_losses)
        state.results.efficiency = res.get("efficiency", state.results.efficiency)

    return state


def save_project(path: str, state: AppState) -> None:
    """Save the current project state to a JSON file."""
    data = state_to_dict(state)
    if os.path.isfile(path):
        backup_path = path + ".bak"
        try:
            os.replace(path, backup_path)
        except OSError:
            pass
    with open(path, "w") as f:
        json.dump(data, f, indent=2)


def load_project(path: str) -> AppState:
    """Load a project state from a JSON file."""
    with open(path, "r") as f:
        data = json.load(f)
    return dict_to_state(data)
