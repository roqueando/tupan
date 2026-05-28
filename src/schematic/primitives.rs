/// Primitives for drawing circuit schematic elements.

/// A position in 2D space (in egui canvas coordinates).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Types of schematic elements that can be drawn.
#[derive(Debug, Clone)]
pub enum SchematicElement {
    /// Voltage source (circle with +/-)
    Source {
        pos: Pos,
        label: String,
        value: String,
    },
    /// Inductor (curved line / zigzag)
    Inductor {
        pos: Pos,
        label: String,
        value: String,
    },
    /// Capacitor (two parallel plates)
    Capacitor {
        pos: Pos,
        label: String,
        value: String,
    },
    /// Diode (triangle + bar)
    Diode {
        pos: Pos,
        label: String,
    },
    /// Switch / MOSFET
    Switch {
        pos: Pos,
        label: String,
    },
    /// Load resistor
    Resistor {
        pos: Pos,
        label: String,
        value: String,
    },
    /// Wire connection
    Wire {
        from: Pos,
        to: Pos,
    },
    /// Connection node (dot) — reserved for future use
    #[allow(dead_code)]
    Node {
        pos: Pos,
        label: String,
    },
    /// Ground symbol
    Ground {
        pos: Pos,
    },
    /// Text label at position
    Label {
        pos: Pos,
        text: String,
    },
}
