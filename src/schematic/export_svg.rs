/// Export schematic diagrams as SVG strings.
///
/// Converts a list of `SchematicElement`s into an SVG document
/// that can be saved to a file or copied to clipboard.

use crate::schematic::primitives::SchematicElement;
use std::fmt::Write;

/// Export a list of schematic elements to an SVG string.
///
/// # Arguments
/// * `elements` - List of schematic elements to render
/// * `width` - SVG canvas width in points
/// * `height` - SVG canvas height in points
///
/// # Returns
/// A complete SVG document as a String.
pub fn export_svg(elements: &[SchematicElement], width: f32, height: f32) -> String {
    let mut svg = String::new();

    writeln!(
        svg,
        r#"<?xml version="1.0" encoding="UTF-8"?>"#
    )
    .unwrap();
    writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}pt" height="{h}pt">"#,
        w = width as i32,
        h = height as i32,
    )
    .unwrap();

    // Add a white background
    writeln!(svg, r#"  <rect width="100%" height="100%" fill="white"/>"#).unwrap();

    // Default styles
    writeln!(
        svg,
        r#"  <style>
    .wire {{ stroke: black; stroke-width: 1.5; fill: none; }}
    .comp {{ stroke: black; stroke-width: 1.5; fill: none; }}
    .label {{ font-family: monospace; font-size: 11px; fill: #333; }}
    .value {{ font-family: monospace; font-size: 10px; fill: #666; }}
    .ground {{ stroke: black; stroke-width: 1.5; fill: none; }}
  </style>"#
    )
    .unwrap();

    for element in elements {
        match element {
            SchematicElement::Wire { from, to } => {
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#,
                    x1 = from.x,
                    y1 = from.y,
                    x2 = to.x,
                    y2 = to.y,
                )
                .unwrap();
            }
            SchematicElement::Source { pos, label, value } => {
                let cx = pos.x;
                let cy = pos.y;
                let r = 14.0;
                // Circle
                writeln!(
                    svg,
                    r#"  <circle class="comp" cx="{cx}" cy="{cy}" r="{r}"/>"#,
                    cx = cx,
                    cy = cy,
                    r = r,
                )
                .unwrap();
                // +/- signs
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x1}" y="{y1}" text-anchor="middle">+</text>"#,
                    x1 = cx - 4.0,
                    y1 = cy - 8.0 + 4.0,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x1}" y="{y1}" text-anchor="middle">-</text>"#,
                    x1 = cx - 4.0,
                    y1 = cy + 8.0 + 4.0,
                )
                .unwrap();
                // Label
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}">{l}: {v}</text>"#,
                    x = cx + r + 5.0,
                    y = cy + 3.0,
                    l = label,
                    v = value,
                )
                .unwrap();
            }
            SchematicElement::Resistor { pos, label, value } => {
                let cx = pos.x;
                let cy = pos.y;
                let w = 30.0;
                let h = 14.0;
                // Rectangle
                writeln!(
                    svg,
                    r#"  <rect class="comp" x="{x}" y="{y}" width="{w}" height="{h}"/>"#,
                    x = cx - w / 2.0,
                    y = cy - h / 2.0,
                    w = w,
                    h = h,
                )
                .unwrap();
                // Leads
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx - w / 2.0 - 10.0,
                    x2 = cx - w / 2.0,
                    y = cy,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx + w / 2.0,
                    x2 = cx + w / 2.0 + 10.0,
                    y = cy,
                )
                .unwrap();
                // Label
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}">{l}: {v}</text>"#,
                    x = cx + w / 2.0 + 15.0,
                    y = cy + 3.0,
                    l = label,
                    v = value,
                )
                .unwrap();
            }
            SchematicElement::Inductor { pos, label, value } => {
                let cx = pos.x;
                let cy = pos.y;
                let segments = 4;
                let seg_w = 8.0;
                let seg_h = 10.0;
                let total_w = segments as f32 * seg_w;
                let start_x = cx - total_w / 2.0;

                // Build zigzag path
                let mut d = format!("M {:.1} {:.1}", start_x - 10.0, cy);
                for i in 0..segments {
                    let x1 = start_x + (i as f32 + 0.5) * seg_w;
                    let y1 = if i % 2 == 0 {
                        cy - seg_h
                    } else {
                        cy + seg_h
                    };
                    write!(d, " L {:.1} {:.1}", x1, y1).unwrap();
                }
                write!(d, " L {:.1} {:.1}", start_x + total_w + 10.0, cy).unwrap();

                writeln!(svg, r#"  <path class="comp" d="{d}"/>"#, d = d).unwrap();
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}" text-anchor="middle">{l}: {v}</text>"#,
                    x = cx,
                    y = cy - seg_h - 5.0,
                    l = label,
                    v = value,
                )
                .unwrap();
            }
            SchematicElement::Capacitor { pos, label, value } => {
                let cx = pos.x;
                let cy = pos.y;
                let half_plate = 10.0;

                // Two plates
                writeln!(
                    svg,
                    r#"  <line class="comp" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#,
                    x1 = cx - half_plate,
                    y1 = cy,
                    x2 = cx + half_plate,
                    y2 = cy,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <line class="comp" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#,
                    x1 = cx - half_plate,
                    y1 = cy + 12.0,
                    x2 = cx + half_plate,
                    y2 = cy + 12.0,
                )
                .unwrap();
                // Leads
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x}" y1="{y1}" x2="{x}" y2="{y2}"/>"#,
                    x = cx,
                    y1 = cy - 8.0,
                    y2 = cy,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x}" y1="{y1}" x2="{x}" y2="{y2}"/>"#,
                    x = cx,
                    y1 = cy + 12.0,
                    y2 = cy + 20.0,
                )
                .unwrap();
                // Label
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}">{l}: {v}</text>"#,
                    x = cx + half_plate + 12.0,
                    y = cy + 8.0,
                    l = label,
                    v = value,
                )
                .unwrap();
            }
            SchematicElement::Diode { pos, label } => {
                let cx = pos.x;
                let cy = pos.y;

                // Triangle (anode side)
                let tri_points = format!(
                    "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
                    cx,
                    cy - 8.0,
                    cx,
                    cy + 8.0,
                    cx + 12.0,
                    cy
                );
                writeln!(
                    svg,
                    r#"  <polygon class="comp" points="{pts}"/>"#,
                    pts = tri_points
                )
                .unwrap();

                // Bar (cathode)
                writeln!(
                    svg,
                    r#"  <line class="comp" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#,
                    x1 = cx + 14.0,
                    y1 = cy - 10.0,
                    x2 = cx + 14.0,
                    y2 = cy + 10.0,
                )
                .unwrap();

                // Leads
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx - 8.0,
                    x2 = cx,
                    y = cy,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx + 14.0,
                    x2 = cx + 22.0,
                    y = cy,
                )
                .unwrap();

                // Label
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}">{l}</text>"#,
                    x = cx + 22.0 + 12.0,
                    y = cy + 3.0,
                    l = label,
                )
                .unwrap();
            }
            SchematicElement::Switch { pos, label } => {
                let cx = pos.x;
                let cy = pos.y;

                // Lead in
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx - 12.0,
                    x2 = cx,
                    y = cy,
                )
                .unwrap();

                // Switch contact (angled)
                writeln!(
                    svg,
                    r#"  <line class="comp" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#,
                    x1 = cx,
                    y1 = cy,
                    x2 = cx + 6.0,
                    y2 = cy - 8.0,
                )
                .unwrap();

                // Lead out
                writeln!(
                    svg,
                    r#"  <line class="wire" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx + 12.0,
                    x2 = cx + 16.0,
                    y = cy,
                )
                .unwrap();

                // Label
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}" text-anchor="middle">{l}</text>"#,
                    x = cx + 6.0,
                    y = cy - 14.0,
                    l = label,
                )
                .unwrap();
            }
            SchematicElement::Ground { pos } => {
                let cx = pos.x;
                let cy = pos.y;

                // Vertical
                writeln!(
                    svg,
                    r#"  <line class="ground" x1="{x}" y1="{y1}" x2="{x}" y2="{y2}"/>"#,
                    x = cx,
                    y1 = cy,
                    y2 = cy + 6.0,
                )
                .unwrap();

                // Three horizontal lines
                writeln!(
                    svg,
                    r#"  <line class="ground" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx - 10.0,
                    x2 = cx + 10.0,
                    y = cy + 6.0,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <line class="ground" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx - 6.0,
                    x2 = cx + 6.0,
                    y = cy + 10.0,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <line class="ground" x1="{x1}" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                    x1 = cx - 3.0,
                    x2 = cx + 3.0,
                    y = cy + 14.0,
                )
                .unwrap();
            }
            SchematicElement::Node { pos, label } => {
                let cx = pos.x;
                let cy = pos.y;
                writeln!(
                    svg,
                    r#"  <circle class="comp" cx="{cx}" cy="{cy}" r="3" fill="black"/>"#,
                    cx = cx,
                    cy = cy,
                )
                .unwrap();
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}">{l}</text>"#,
                    x = cx + 6.0,
                    y = cy + 3.0,
                    l = label,
                )
                .unwrap();
            }
            SchematicElement::Label { pos, text } => {
                writeln!(
                    svg,
                    r#"  <text class="label" x="{x}" y="{y}">{t}</text>"#,
                    x = pos.x,
                    y = pos.y + 3.0,
                    t = text,
                )
                .unwrap();
            }
        }
    }

    writeln!(svg, "</svg>").unwrap();
    svg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::primitives::Pos;
    use crate::schematic::primitives::SchematicElement::*;

    #[test]
    fn test_export_svg_basic() {
        let elements = vec![
            Wire {
                from: Pos::new(10.0, 10.0),
                to: Pos::new(100.0, 10.0),
            },
            Ground {
                pos: Pos::new(50.0, 30.0),
            },
            Label {
                pos: Pos::new(10.0, 40.0),
                text: "Test".to_owned(),
            },
        ];

        let svg = export_svg(&elements, 200.0, 100.0);

        assert!(svg.starts_with("<?xml"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Test"));
        assert!(svg.contains("white"));
    }

    #[test]
    fn test_export_svg_source() {
        let elements = vec![Source {
            pos: Pos::new(50.0, 50.0),
            label: "Vin".to_owned(),
            value: "12 V".to_owned(),
        }];

        let svg = export_svg(&elements, 200.0, 100.0);
        assert!(svg.contains("Vin"));
        assert!(svg.contains("12 V"));
    }
}
