"""Shared rendering functions for schematic elements.

Draws schematic elements using QPainter.
Mirrors src/schematic/renderer.rs.
"""

import math
from PySide6.QtCore import QPointF, QRectF, Qt
from PySide6.QtGui import QPainter, QPen, QColor, QFont, QFontMetrics

from tupan.schematic.primitives import (
    Pos, Wire, Source, Switch, Diode, Inductor, Capacitor,
    Resistor, Ground, Label, SchematicElement,
)
from tupan.ui.theme import COLORS


def draw_element(painter: QPainter, element: SchematicElement,
                 origin: QPointF,
                 highlight: bool = False):
    """Draw a single schematic element on the canvas.

    Args:
        painter: QPainter to draw with
        element: The element to draw
        origin: Offset to add to all positions (for canvas panning)
        highlight: If True, draw selection highlight
    """
    colors = COLORS

    pen = QPen(QColor(colors["wire"]), 2.0)
    highlight_pen = QPen(QColor("#ffff00"), 1.5)

    painter.setFont(QFont("monospace", 10))

    if highlight:
        pos = _element_pos(element)
        painter.setPen(highlight_pen)
        painter.drawRect(QRectF(
            origin.x() + pos.x - 10,
            origin.y() + pos.y - 10,
            20, 20,
        ))

    if isinstance(element, Wire):
        pen.setWidth(2)
        painter.setPen(pen)
        p1 = QPointF(origin.x() + element.from_pos.x,
                     origin.y() + element.from_pos.y)
        p2 = QPointF(origin.x() + element.to_pos.x,
                     origin.y() + element.to_pos.y)
        painter.drawLine(p1, p2)

    elif isinstance(element, Source):
        cx = origin.x() + element.pos.x
        cy = origin.y() + element.pos.y
        r = 14.0
        painter.setPen(pen)
        painter.drawEllipse(QPointF(cx, cy), r, r)
        # +/- signs
        painter.drawText(QPointF(cx - 6, cy - 4), "+")
        painter.drawText(QPointF(cx - 6, cy + 10), "-")
        # Label
        label_color = QColor(colors["label"])
        painter.setPen(QPen(label_color, 1))
        painter.drawText(QPointF(cx + r + 5, cy - 4),
                         f"{element.label}: {element.value}")

    elif isinstance(element, Inductor):
        cx = origin.x() + element.pos.x
        cy = origin.y() + element.pos.y
        painter.setPen(pen)
        # Draw a zigzag (simplified inductor symbol)
        w = 30.0
        h = 12.0
        n_loops = 4
        points = []
        for i in range(n_loops + 1):
            x = cx - w / 2 + (w / n_loops) * i
            y = cy + (h if i % 2 == 0 else -h)
            if i == 0 or i == n_loops:
                y = cy
            points.append(QPointF(x, y))
        for i in range(len(points) - 1):
            painter.drawLine(points[i], points[i + 1])
        # Label
        label_color = QColor(colors["label"])
        painter.setPen(QPen(label_color, 1))
        painter.drawText(QPointF(cx + w / 2 + 5, cy - 4),
                         f"{element.label}: {element.value}")

    elif isinstance(element, Capacitor):
        cx = origin.x() + element.pos.x
        cy = origin.y() + element.pos.y
        painter.setPen(pen)
        # Two parallel plates
        plate_h = 12.0
        painter.drawLine(QPointF(cx - 4, cy - plate_h / 2),
                         QPointF(cx - 4, cy + plate_h / 2))
        painter.drawLine(QPointF(cx + 4, cy - plate_h / 2),
                         QPointF(cx + 4, cy + plate_h / 2))
        # Label
        label_color = QColor(colors["label"])
        painter.setPen(QPen(label_color, 1))
        painter.drawText(QPointF(cx + 12, cy - 4),
                         f"{element.label}: {element.value}")

    elif isinstance(element, Diode):
        cx = origin.x() + element.pos.x
        cy = origin.y() + element.pos.y
        painter.setPen(pen)
        s = 10.0
        # Triangle (anode side)
        painter.drawLine(QPointF(cx - s, cy - s),
                         QPointF(cx + s, cy))
        painter.drawLine(QPointF(cx - s, cy + s),
                         QPointF(cx + s, cy))
        # Bar (cathode side)
        painter.drawLine(QPointF(cx + s, cy - s),
                         QPointF(cx + s, cy + s))
        # Label
        label_color = QColor(colors["label"])
        painter.setPen(QPen(label_color, 1))
        painter.drawText(QPointF(cx + s + 5, cy + 4),
                         element.label)

    elif isinstance(element, Switch):
        cx = origin.x() + element.pos.x
        cy = origin.y() + element.pos.y
        painter.setPen(pen)
        # Simplified switch: angled line
        s = 10.0
        painter.drawLine(QPointF(cx - s, cy),
                         QPointF(cx - s, cy - s))  # vertical
        painter.drawLine(QPointF(cx - s, cy - s),
                         QPointF(cx + s, cy))  # diagonal to make contact
        painter.drawLine(QPointF(cx + s, cy),
                         QPointF(cx + s, cy - s))  # vertical right
        # Label
        label_color = QColor(colors["label"])
        painter.setPen(QPen(label_color, 1))
        painter.drawText(QPointF(cx + s + 5, cy + 4),
                         element.label)

    elif isinstance(element, Resistor):
        cx = origin.x() + element.pos.x
        cy = origin.y() + element.pos.y
        painter.setPen(pen)
        # Zigzag resistor
        w = 30.0
        h = 8.0
        n_zigs = 5
        points = [QPointF(cx - w / 2, cy)]
        for i in range(1, n_zigs + 1):
            x = cx - w / 2 + (w / n_zigs) * i
            y = cy + (h if i % 2 == 1 else -h)
            if i == n_zigs:
                y = cy
            points.append(QPointF(x, y))
        for i in range(len(points) - 1):
            painter.drawLine(points[i], points[i + 1])
        # Label
        label_color = QColor(colors["label"])
        painter.setPen(QPen(label_color, 1))
        painter.drawText(QPointF(cx + w / 2 + 5, cy - 4),
                         f"{element.label}: {element.value}")

    elif isinstance(element, Ground):
        cx = origin.x() + element.pos.x
        cy = origin.y() + element.pos.y
        ground_color = QColor(colors["ground"])
        painter.setPen(QPen(ground_color, 2))
        # Vertical line
        painter.drawLine(QPointF(cx, cy - 8), QPointF(cx, cy))
        # Horizontal lines
        painter.drawLine(QPointF(cx - 10, cy), QPointF(cx + 10, cy))  # main
        painter.drawLine(QPointF(cx - 6, cy + 4), QPointF(cx + 6, cy + 4))  # thinner
        painter.drawLine(QPointF(cx - 2, cy + 8), QPointF(cx + 2, cy + 8))  # thinnest

    elif isinstance(element, Label):
        label_color = QColor(colors["label"])
        painter.setPen(QPen(label_color, 1))
        painter.drawText(
            QPointF(origin.x() + element.pos.x, origin.y() + element.pos.y),
            element.text
        )


def _element_pos(element: SchematicElement) -> Pos:
    """Get the position of an element (for highlight bounds)."""
    if isinstance(element, (Source, Inductor, Capacitor, Diode,
                            Switch, Resistor, Ground, Label)):
        return element.pos
    elif isinstance(element, Wire):
        return Pos(
            (element.from_pos.x + element.to_pos.x) / 2,
            (element.from_pos.y + element.to_pos.y) / 2,
        )
    return Pos(0, 0)
