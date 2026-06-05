"""Layout definitions for converter schematics.

Provides pre-defined positions for each converter type.
Mirrors src/schematic/layout.rs.
"""

from dataclasses import dataclass
from tupan.schematic.primitives import (
    Pos, Wire, Source, Switch, Diode, Inductor, Capacitor,
    Resistor, Ground, Label, SchematicElement,
)
from tupan.domain import ConverterType


@dataclass
class ComponentValues:
    """Values to annotate on the schematic components."""
    vin: str = ""
    vout: str = ""
    inductance: str = ""
    capacitance: str = ""
    load: str = ""
    frequency: str = ""
    duty_cycle: str = ""


def generate_schematic(converter_type: ConverterType,
                       comp_values: ComponentValues) -> list:
    """Generate the schematic elements for the given converter type."""
    if converter_type == ConverterType.Buck:
        return _buck_layout(comp_values)
    elif converter_type == ConverterType.Boost:
        return _boost_layout(comp_values)
    elif converter_type == ConverterType.VsiSinglePhase:
        return _vsi_layout(comp_values)
    return []


def _buck_layout(values: ComponentValues) -> list:
    """Buck converter schematic layout.

    Vin+ ──── SW ──┬── L ──┬── Rload ──── Vout+
                    │       │
                   Diod    C
                    │       │
    Vin- ──────────┴───────┴────────────── Vout-
    """
    mid_y = 0.0
    x_start = 20.0
    x_switch = 80.0
    x_l_start = 140.0
    x_l_end = 200.0
    x_load = 260.0
    x_end = 320.0
    ground_y = 80.0

    elements = []

    # Top wire segments
    elements.append(Wire(from_pos=Pos(x_start, mid_y),
                         to_pos=Pos(x_switch, mid_y)))
    elements.append(Wire(from_pos=Pos(x_switch, mid_y),
                         to_pos=Pos(x_l_start, mid_y)))
    elements.append(Wire(from_pos=Pos(x_l_end, mid_y),
                         to_pos=Pos(x_load, mid_y)))
    elements.append(Wire(from_pos=Pos(x_load, mid_y),
                         to_pos=Pos(x_end, mid_y)))

    # Source
    elements.append(Source(pos=Pos(x_start - 10.0, mid_y - 30.0),
                           label="Vin", value=values.vin))

    # Switch
    elements.append(Switch(pos=Pos(x_switch, mid_y), label="SW"))

    # Diode (vertical, down to ground)
    elements.append(Wire(from_pos=Pos(x_switch, mid_y),
                         to_pos=Pos(x_switch, mid_y + 5.0)))
    elements.append(Diode(pos=Pos(x_switch, mid_y + 5.0), label="D"))
    elements.append(Wire(from_pos=Pos(x_switch, mid_y + 25.0),
                         to_pos=Pos(x_switch, ground_y)))

    # Inductor
    elements.append(Inductor(pos=Pos(x_l_start + 30.0, mid_y),
                             label="L", value=values.inductance))

    # Capacitor branch (vertical)
    elements.append(Wire(from_pos=Pos(x_l_start + 30.0, mid_y),
                         to_pos=Pos(x_l_start + 30.0, mid_y + 30.0)))
    elements.append(Capacitor(pos=Pos(x_l_start + 30.0, mid_y + 30.0),
                              label="C", value=values.capacitance))
    elements.append(Wire(from_pos=Pos(x_l_start + 30.0, mid_y + 50.0),
                         to_pos=Pos(x_l_start + 30.0, ground_y)))

    # Load resistor
    elements.append(Resistor(pos=Pos(x_load, mid_y),
                             label="R", value=values.load))

    # Ground
    elements.append(Ground(pos=Pos(x_end, ground_y)))

    # Output label
    elements.append(Label(pos=Pos(x_end + 10.0, mid_y - 8.0),
                          text=f"Vout = {values.vout}"))

    return elements


def _boost_layout(values: ComponentValues) -> list:
    """Boost converter schematic layout.

    Vin ──── L ──┬── SW ──┬── D ──┬── Rload ──── Vout
                  │        │       │
                 C        GND     C
                  │               │
    GND ─────────┴───────────────┴─────────────────
    """
    mid_y = 0.0
    x_start = 20.0
    x_l = 80.0
    x_switch_l = 140.0
    x_diode_start = 200.0
    x_load = 260.0
    x_end = 320.0
    ground_y = 80.0

    elements = []

    # Top wire
    elements.append(Wire(from_pos=Pos(x_start, mid_y),
                         to_pos=Pos(x_l, mid_y)))
    elements.append(Wire(from_pos=Pos(x_l + 40.0, mid_y),
                         to_pos=Pos(x_switch_l, mid_y)))
    elements.append(Wire(from_pos=Pos(x_switch_l, mid_y),
                         to_pos=Pos(x_diode_start, mid_y)))
    elements.append(Wire(from_pos=Pos(x_diode_start + 30.0, mid_y),
                         to_pos=Pos(x_load, mid_y)))
    elements.append(Wire(from_pos=Pos(x_load, mid_y),
                         to_pos=Pos(x_end, mid_y)))

    # Source
    elements.append(Source(pos=Pos(x_start - 10.0, mid_y - 30.0),
                           label="Vin", value=values.vin))

    # Inductor
    elements.append(Inductor(pos=Pos(x_l + 20.0, mid_y),
                             label="L", value=values.inductance))

    # Switch to ground
    elements.append(Wire(from_pos=Pos(x_switch_l, mid_y),
                         to_pos=Pos(x_switch_l, mid_y + 5.0)))
    elements.append(Switch(pos=Pos(x_switch_l, mid_y + 5.0), label="SW"))
    elements.append(Wire(from_pos=Pos(x_switch_l, mid_y + 25.0),
                         to_pos=Pos(x_switch_l, ground_y)))

    # Diode (forward)
    elements.append(Wire(from_pos=Pos(x_diode_start, mid_y),
                         to_pos=Pos(x_diode_start + 5.0, mid_y)))
    elements.append(Diode(pos=Pos(x_diode_start + 5.0, mid_y), label="D"))

    # Capacitor branch (vertical, after diode)
    cap_x = x_diode_start + 30.0
    elements.append(Wire(from_pos=Pos(cap_x, mid_y),
                         to_pos=Pos(cap_x, mid_y + 30.0)))
    elements.append(Capacitor(pos=Pos(cap_x, mid_y + 30.0),
                              label="C", value=values.capacitance))
    elements.append(Wire(from_pos=Pos(cap_x, mid_y + 50.0),
                         to_pos=Pos(cap_x, ground_y)))

    # Load resistor
    elements.append(Resistor(pos=Pos(x_load, mid_y),
                             label="R", value=values.load))

    # Ground at end
    elements.append(Ground(pos=Pos(x_end, ground_y)))

    # Input ground wire
    elements.append(Wire(from_pos=Pos(x_start, ground_y),
                         to_pos=Pos(x_switch_l, ground_y)))

    # Output label
    elements.append(Label(pos=Pos(x_end + 10.0, mid_y - 8.0),
                          text=f"Vout = {values.vout}"))

    return elements


def _vsi_layout(values: ComponentValues) -> list:
    """Single-phase VSI schematic layout (simplified).

    Shows DC source, 4-switch H-bridge, and RL load.
    """
    mid_y = 0.0
    x_start = 20.0
    x_bridge_l = 100.0
    x_bridge_r = 220.0
    x_load = 280.0
    x_end = 340.0
    top_y = -60.0
    bot_y = 60.0

    elements = []

    # DC source
    elements.append(Source(pos=Pos(x_start - 10.0, mid_y - 30.0),
                           label="Vdc", value=values.vin))

    # Top rail
    elements.append(Wire(from_pos=Pos(x_start, top_y),
                         to_pos=Pos(x_bridge_l, top_y)))
    elements.append(Wire(from_pos=Pos(x_bridge_l, top_y),
                         to_pos=Pos(x_bridge_r, top_y)))

    # Bottom rail
    elements.append(Wire(from_pos=Pos(x_start, bot_y),
                         to_pos=Pos(x_bridge_l, bot_y)))
    elements.append(Wire(from_pos=Pos(x_bridge_l, bot_y),
                         to_pos=Pos(x_bridge_r, bot_y)))

    # Switches (4 in H-bridge)
    # Top-left switch
    elements.append(Switch(pos=Pos(x_bridge_l, top_y + 10.0),
                           label="S1"))
    elements.append(Wire(from_pos=Pos(x_bridge_l, top_y + 30.0),
                         to_pos=Pos(x_bridge_l, mid_y)))

    # Bottom-left switch
    elements.append(Switch(pos=Pos(x_bridge_l, mid_y + 10.0),
                           label="S2"))
    elements.append(Wire(from_pos=Pos(x_bridge_l, mid_y + 30.0),
                         to_pos=Pos(x_bridge_l, bot_y)))

    # Top-right switch
    elements.append(Switch(pos=Pos(x_bridge_r, top_y + 10.0),
                           label="S3"))
    elements.append(Wire(from_pos=Pos(x_bridge_r, top_y + 30.0),
                         to_pos=Pos(x_bridge_r, mid_y)))

    # Bottom-right switch
    elements.append(Switch(pos=Pos(x_bridge_r, mid_y + 10.0),
                           label="S4"))
    elements.append(Wire(from_pos=Pos(x_bridge_r, mid_y + 30.0),
                         to_pos=Pos(x_bridge_r, bot_y)))

    # AC output load
    elements.append(Wire(from_pos=Pos(x_bridge_l, mid_y),
                         to_pos=Pos(x_load, mid_y)))
    elements.append(Resistor(pos=Pos(x_load, mid_y),
                             label="R", value=values.load))

    # Complete circuit via top/bottom
    # (simplified - just show the load connected to the H-bridge midpoint)
    elements.append(Label(pos=Pos(x_end, top_y - 10.0),
                          text=f"f_out = {values.frequency}"))

    elements.append(Label(pos=Pos(x_end, bot_y + 5.0),
                          text=f"Vout = {values.vout}"))

    return elements
