"""Export schematic diagrams as SVG strings.

Converts a list of SchematicElements into an SVG document.
Mirrors src/schematic/export_svg.rs — uses string building instead of
QSvgGenerator for maximum compatibility with Nuitka packaging.
"""

from tupan.schematic.primitives import (
    Pos, Wire, Source, Switch, Diode, Inductor, Capacitor,
    Resistor, Ground, Label, SchematicElement,
)
from xml.etree import ElementTree as ET
from xml.dom import minidom


def _prettify(elem: ET.Element) -> str:
    """Return a pretty-printed XML string."""
    rough = ET.tostring(elem, encoding="unicode")
    reparsed = minidom.parseString(rough.encode())
    return reparsed.toprettyxml(indent="  ")


def export_svg(elements: list, width: float = 500.0,
               height: float = 300.0) -> str:
    """Export a list of schematic elements to an SVG string.

    Args:
        elements: List of SchematicElement objects to render
        width: SVG canvas width in points
        height: SVG canvas height in points

    Returns:
        A complete SVG document as a String.
    """
    svg = ET.Element("svg")
    svg.set("xmlns", "http://www.w3.org/2000/svg")
    svg.set("viewBox", f"0 0 {int(width)} {int(height)}")
    svg.set("width", f"{int(width)}pt")
    svg.set("height", f"{int(height)}pt")

    # White background
    rect = ET.SubElement(svg, "rect")
    rect.set("width", "100%")
    rect.set("height", "100%")
    rect.set("fill", "white")

    # Style element
    style = ET.SubElement(svg, "style")
    style.text = """
    .wire { stroke: black; stroke-width: 1.5; fill: none; }
    .comp { stroke: black; stroke-width: 1.5; fill: none; }
    .label { font-family: monospace; font-size: 11px; fill: #333; }
    .value { font-family: monospace; font-size: 10px; fill: #666; }
    .ground { stroke: black; stroke-width: 1.5; fill: none; }
    """

    for element in elements:
        if isinstance(element, Wire):
            line = ET.SubElement(svg, "line")
            line.set("class", "wire")
            line.set("x1", str(element.from_pos.x))
            line.set("y1", str(element.from_pos.y))
            line.set("x2", str(element.to_pos.x))
            line.set("y2", str(element.to_pos.y))

        elif isinstance(element, Source):
            cx, cy = element.pos.x, element.pos.y
            r = 14.0
            circle = ET.SubElement(svg, "circle")
            circle.set("class", "comp")
            circle.set("cx", str(cx))
            circle.set("cy", str(cy))
            circle.set("r", str(r))

            # Plus sign
            plus = ET.SubElement(svg, "text")
            plus.set("class", "label")
            plus.set("x", str(cx - 4))
            plus.set("y", str(cy - 4))
            plus.set("text-anchor", "middle")
            plus.text = "+"

            # Minus sign
            minus = ET.SubElement(svg, "text")
            minus.set("class", "label")
            minus.set("x", str(cx - 4))
            minus.set("y", str(cy + 8))
            minus.set("text-anchor", "middle")
            minus.text = "-"

            # Label
            lbl = ET.SubElement(svg, "text")
            lbl.set("class", "label")
            lbl.set("x", str(cx + r + 5))
            lbl.set("y", str(cy + 3))
            lbl.text = f"{element.label}: {element.value}"

        elif isinstance(element, Inductor):
            cx, cy = element.pos.x, element.pos.y
            w, h = 30.0, 12.0
            n_loops = 4
            points = []
            for i in range(n_loops + 1):
                x = cx - w / 2 + (w / n_loops) * i
                y = cy + (h if i % 2 == 0 else -h)
                if i == 0 or i == n_loops:
                    y = cy
                points.append(f"{x},{y}")
            poly = ET.SubElement(svg, "polyline")
            poly.set("class", "comp")
            poly.set("points", " ".join(points))

            lbl = ET.SubElement(svg, "text")
            lbl.set("class", "label")
            lbl.set("x", str(cx + w / 2 + 5))
            lbl.set("y", str(cy + 3))
            lbl.text = f"{element.label}: {element.value}"

        elif isinstance(element, Capacitor):
            cx, cy = element.pos.x, element.pos.y
            plate_h = 12.0
            line1 = ET.SubElement(svg, "line")
            line1.set("class", "comp")
            line1.set("x1", str(cx - 4))
            line1.set("y1", str(cy - plate_h / 2))
            line1.set("x2", str(cx - 4))
            line1.set("y2", str(cy + plate_h / 2))
            line2 = ET.SubElement(svg, "line")
            line2.set("class", "comp")
            line2.set("x1", str(cx + 4))
            line2.set("y1", str(cy - plate_h / 2))
            line2.set("x2", str(cx + 4))
            line2.set("y2", str(cy + plate_h / 2))

            lbl = ET.SubElement(svg, "text")
            lbl.set("class", "label")
            lbl.set("x", str(cx + 12))
            lbl.set("y", str(cy + 3))
            lbl.text = f"{element.label}: {element.value}"

        elif isinstance(element, Diode):
            cx, cy = element.pos.x, element.pos.y
            s = 10.0
            # Triangle
            tri = ET.SubElement(svg, "polygon")
            tri.set("class", "comp")
            tri.set("points",
                    f"{cx - s},{cy - s} {cx + s},{cy} {cx - s},{cy + s}")
            # Bar
            bar = ET.SubElement(svg, "line")
            bar.set("class", "comp")
            bar.set("x1", str(cx + s))
            bar.set("y1", str(cy - s))
            bar.set("x2", str(cx + s))
            bar.set("y2", str(cy + s))

            lbl = ET.SubElement(svg, "text")
            lbl.set("class", "label")
            lbl.set("x", str(cx + s + 5))
            lbl.set("y", str(cy + 4))
            lbl.text = element.label

        elif isinstance(element, Switch):
            cx, cy = element.pos.x, element.pos.y
            s = 10.0
            lines = [
                (cx - s, cy, cx - s, cy - s),
                (cx - s, cy - s, cx + s, cy),
                (cx + s, cy, cx + s, cy - s),
            ]
            for x1, y1, x2, y2 in lines:
                line = ET.SubElement(svg, "line")
                line.set("class", "comp")
                line.set("x1", str(x1))
                line.set("y1", str(y1))
                line.set("x2", str(x2))
                line.set("y2", str(y2))

            lbl = ET.SubElement(svg, "text")
            lbl.set("class", "label")
            lbl.set("x", str(cx + s + 5))
            lbl.set("y", str(cy + 4))
            lbl.text = element.label

        elif isinstance(element, Resistor):
            cx, cy = element.pos.x, element.pos.y
            w, h = 30.0, 8.0
            n_zigs = 5
            points = [f"{cx - w / 2},{cy}"]
            for i in range(1, n_zigs + 1):
                x = cx - w / 2 + (w / n_zigs) * i
                y = cy + (h if i % 2 == 1 else -h)
                if i == n_zigs:
                    y = cy
                points.append(f"{x},{y}")
            poly = ET.SubElement(svg, "polyline")
            poly.set("class", "comp")
            poly.set("points", " ".join(points))

            lbl = ET.SubElement(svg, "text")
            lbl.set("class", "label")
            lbl.set("x", str(cx + w / 2 + 5))
            lbl.set("y", str(cy + 3))
            lbl.text = f"{element.label}: {element.value}"

        elif isinstance(element, Ground):
            cx, cy = element.pos.x, element.pos.y
            lines = [
                (cx, cy - 8, cx, cy),
                (cx - 10, cy, cx + 10, cy),
                (cx - 6, cy + 4, cx + 6, cy + 4),
                (cx - 2, cy + 8, cx + 2, cy + 8),
            ]
            for x1, y1, x2, y2 in lines:
                line = ET.SubElement(svg, "line")
                line.set("class", "ground")
                line.set("x1", str(x1))
                line.set("y1", str(y1))
                line.set("x2", str(x2))
                line.set("y2", str(y2))

        elif isinstance(element, Label):
            lbl = ET.SubElement(svg, "text")
            lbl.set("class", "label")
            lbl.set("x", str(element.pos.x))
            lbl.set("y", str(element.pos.y))
            lbl.text = element.text

    return _prettify(svg)
