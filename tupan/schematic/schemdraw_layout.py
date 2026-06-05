"""Schematic generation using schemdraw library."""

import os
import tempfile
from dataclasses import dataclass

import schemdraw
from schemdraw import elements as elm
from tupan.domain import ConverterType


@dataclass
class ComponentLabels:
    """Labels for schematic components."""
    vin: str = "48V"
    vout: str = "12V"
    inductance: str = "100μH"
    capacitance: str = "100μF"
    load: str = "10Ω"
    frequency: str = "100kHz"
    duty_cycle: str = "25%"


def _save_png(d: schemdraw.Drawing) -> bytes:
    """Save a schemdraw drawing to PNG bytes using a temp file."""
    with tempfile.NamedTemporaryFile(suffix='.png', delete=False) as f:
        tmp = f.name
    try:
        d.save(tmp, transparent=True)
        with open(tmp, 'rb') as f:
            data = f.read()
        return data
    finally:
        if os.path.exists(tmp):
            os.unlink(tmp)


def draw_buck(labels: ComponentLabels) -> bytes:
    """Draw a Buck converter schematic as PNG bytes.

    Top rail: Vin -> SW -> L -> R -> GND
    Diode and Capacitor branching down from midpoints.
    """
    d = schemdraw.Drawing()

    d += elm.SourceV().label(f"Vin\n{labels.vin}")
    d += elm.Line().right()
    d += elm.Switch(action='open').label('SW', 'top')
    d += elm.Line().right()
    d += elm.Inductor().label(f"L\n{labels.inductance}")
    d += elm.Line().right()
    d += elm.Resistor().label(f"Rload\n{labels.load}")
    d += elm.Line()
    d += elm.Ground()

    # Diode from switch midpoint to ground
    sw_end = d.elements[3].end
    d += elm.Diode().down().at(sw_end).label('D', 'bot')
    d += elm.Ground()

    # Capacitor from inductor input to ground
    ind_start = d.elements[5].start
    d += elm.Capacitor().down().at(ind_start).label(f"C\n{labels.capacitance}")
    d += elm.Ground()

    return _save_png(d)


def draw_boost(labels: ComponentLabels) -> bytes:
    """Draw a Boost converter schematic as PNG bytes.

    Top rail: Vin -> L -> D -> R -> GND
    Switch branches down from L-D midpoint.
    Capacitor branches down from D-R midpoint.
    """
    d = schemdraw.Drawing()

    d += elm.SourceV().label(f"Vin\n{labels.vin}")
    d += elm.Line().right()
    d += elm.Inductor().label(f"L\n{labels.inductance}")
    d += elm.Line().right()
    d += elm.Diode().label('D')
    d += elm.Line().right()
    d += elm.Resistor().label(f"Rload\n{labels.load}")
    d += elm.Line()
    d += elm.Ground()

    # Switch from L-D midpoint to ground
    sw_pos = d.elements[4].start
    d += elm.Switch(action='open').down().at(sw_pos).label('SW', 'bot')
    d += elm.Ground()

    # Capacitor from D-R midpoint to ground
    cap_pos = d.elements[6].start
    d += elm.Capacitor().down().at(cap_pos).label(f"C\n{labels.capacitance}")
    d += elm.Ground()

    return _save_png(d)


def draw_vsi(labels: ComponentLabels) -> bytes:
    """Draw a simplified single-phase VSI schematic as PNG bytes."""
    d = schemdraw.Drawing()

    d += elm.SourceV().label(f"Vdc\n{labels.vin}")
    d += elm.Line().right()

    dc_pos = d.here
    d += elm.Line().right().length(4)
    right_rail = d.here

    # H-bridge left leg
    d += elm.Switch(action='open').down().at(dc_pos).label('S1', 'left')
    d += elm.Line().down()
    ac_left = d.here
    d += elm.Switch(action='open').down().label('S2', 'left')
    d += elm.Ground()

    # H-bridge right leg
    d += elm.Switch(action='open').down().at(right_rail).label('S3', 'right')
    d += elm.Line().down()
    ac_right = d.here
    d += elm.Switch(action='open').down().label('S4', 'right')
    d += elm.Ground()

    # Load across H-bridge midpoint
    d += elm.Resistor().label(f"Rload\n{labels.load}").at(ac_left)
    d += elm.Line().right()
    d += elm.Line().to(ac_right)

    return _save_png(d)


def draw_converter(converter_type: ConverterType,
                   labels: ComponentLabels) -> bytes:
    """Draw the appropriate converter schematic as PNG bytes."""
    if converter_type == ConverterType.Buck:
        return draw_buck(labels)
    elif converter_type == ConverterType.Boost:
        return draw_boost(labels)
    elif converter_type == ConverterType.VsiSinglePhase:
        return draw_vsi(labels)
    return b""


def save_png(converter_type: ConverterType, labels: ComponentLabels,
             filepath: str):
    """Generate and save a converter schematic PNG."""
    data = draw_converter(converter_type, labels)
    with open(filepath, 'wb') as f:
        f.write(data)
    return data
