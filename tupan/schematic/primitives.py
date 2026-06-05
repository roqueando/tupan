"""Primitives for drawing circuit schematic elements.

Mirrors src/schematic/primitives.rs.
"""

from dataclasses import dataclass
from enum import Enum, auto


@dataclass
class Pos:
    """A position in 2D space."""
    x: float
    y: float


class SchematicElement:
    """Base class for schematic elements."""
    pass


@dataclass
class Source(SchematicElement):
    """Voltage source (circle with +/-)."""
    pos: Pos
    label: str = ""
    value: str = ""


@dataclass
class Inductor(SchematicElement):
    """Inductor (curved line / zigzag)."""
    pos: Pos
    label: str = ""
    value: str = ""


@dataclass
class Capacitor(SchematicElement):
    """Capacitor (two parallel plates)."""
    pos: Pos
    label: str = ""
    value: str = ""


@dataclass
class Diode(SchematicElement):
    """Diode (triangle + bar)."""
    pos: Pos
    label: str = ""


@dataclass
class Switch(SchematicElement):
    """Switch / MOSFET."""
    pos: Pos
    label: str = ""


@dataclass
class Resistor(SchematicElement):
    """Load resistor."""
    pos: Pos
    label: str = ""
    value: str = ""


@dataclass
class Wire(SchematicElement):
    """Wire connection."""
    from_pos: Pos
    to_pos: Pos


@dataclass
class Node_(SchematicElement):  # Node is a Python built-in, use Node_
    """Connection node (dot)."""
    pos: Pos
    label: str = ""


@dataclass
class Ground(SchematicElement):
    """Ground symbol."""
    pos: Pos


@dataclass
class Label(SchematicElement):
    """Text label at position."""
    pos: Pos
    text: str = ""
