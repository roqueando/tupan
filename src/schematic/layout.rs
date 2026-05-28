/// Layout definitions for converter schematics.
/// Provides pre-defined positions for each converter type.
use crate::domain::ConverterType;
use crate::schematic::primitives::{Pos, SchematicElement};

/// Generate the schematic elements for the given converter type.
pub fn generate_schematic(
    converter_type: ConverterType,
    comp_values: &ComponentValues,
) -> Vec<SchematicElement> {
    match converter_type {
        ConverterType::Buck => buck_layout(comp_values),
        ConverterType::Boost => boost_layout(comp_values),
        ConverterType::VsiSinglePhase => vsi_layout(comp_values),
    }
}

/// Values to annotate on the schematic components.
pub struct ComponentValues {
    pub vin: String,
    pub vout: String,
    pub inductance: String,
    pub capacitance: String,
    pub load: String,
    pub frequency: String,
    pub _duty_cycle: String,
}

/// Buck converter schematic layout.
///
///  Vin+ ──── SW ──┬── L ──┬── Rload ──── Vout+
///                  │       │
///                 Diod    C
///                  │       │
///  Vin- ──────────┴───────┴────────────── Vout-
fn buck_layout(values: &ComponentValues) -> Vec<SchematicElement> {
    let mid_y = 0.0;
    let mut elements = Vec::new();

    // Main horizontal wire (top)
    let x_start = 20.0;
    let x_switch = 80.0;
    let x_inductor_start = 140.0;
    let x_inductor_end = 200.0;
    let x_load = 260.0;
    let x_end = 320.0;

    // Ground/bottom wire
    let ground_y = 80.0;

    // Top wire segments
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_start, mid_y),
        to: Pos::new(x_switch, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, mid_y),
        to: Pos::new(x_inductor_start, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_inductor_end, mid_y),
        to: Pos::new(x_load, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_load, mid_y),
        to: Pos::new(x_end, mid_y),
    });

    // Source (Vin)
    elements.push(SchematicElement::Source {
        pos: Pos::new(x_start - 10.0, mid_y - 30.0),
        label: "Vin".to_owned(),
        value: values.vin.clone(),
    });

    // Switch
    elements.push(SchematicElement::Switch {
        pos: Pos::new(x_switch, mid_y),
        label: "SW".to_owned(),
    });

    // Diode (vertical, goes down to ground)
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, mid_y),
        to: Pos::new(x_switch, mid_y + 5.0),
    });
    elements.push(SchematicElement::Diode {
        pos: Pos::new(x_switch, mid_y + 5.0),
        label: "D".to_owned(),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, mid_y + 25.0), // below diode
        to: Pos::new(x_switch, ground_y),
    });

    // Inductor
    elements.push(SchematicElement::Inductor {
        pos: Pos::new(x_inductor_start + 30.0, mid_y),
        label: "L".to_owned(),
        value: values.inductance.clone(),
    });

    // Capacitor (vertical, goes down to ground)
    let cap_x = x_inductor_end - 10.0;
    elements.push(SchematicElement::Wire {
        from: Pos::new(cap_x, mid_y),
        to: Pos::new(cap_x, mid_y + 15.0),
    });
    elements.push(SchematicElement::Capacitor {
        pos: Pos::new(cap_x, mid_y + 15.0),
        label: "C".to_owned(),
        value: values.capacitance.clone(),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(cap_x, mid_y + 35.0),
        to: Pos::new(cap_x, ground_y),
    });

    // Load resistor
    elements.push(SchematicElement::Resistor {
        pos: Pos::new(x_load, mid_y),
        label: "R".to_owned(),
        value: values.load.clone(),
    });

    // Ground symbols
    elements.push(SchematicElement::Ground {
        pos: Pos::new(x_switch, ground_y),
    });
    elements.push(SchematicElement::Ground {
        pos: Pos::new(cap_x, ground_y),
    });
    elements.push(SchematicElement::Ground {
        pos: Pos::new(x_end, ground_y),
    });

    // Bottom wire
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_start, ground_y),
        to: Pos::new(x_switch, ground_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, ground_y),
        to: Pos::new(cap_x, ground_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(cap_x, ground_y),
        to: Pos::new(x_end, ground_y),
    });

    // Labels
    elements.push(SchematicElement::Label {
        pos: Pos::new(x_end + 20.0, mid_y - 5.0),
        text: format!("Vout = {}", values.vout),
    });
    elements.push(SchematicElement::Label {
        pos: Pos::new(x_end + 20.0, mid_y + 10.0),
        text: format!("f = {}", values.frequency),
    });

    elements
}

/// Boost converter schematic layout.
///
///  Vin+ ──── L ──┬── SW ──── Diod ──┬── Rload ─── Vout+
///                 │                   │
///                C                   C (output)
///                 │                   │
///  Vin- ─────────┴───────────────────┴─────────── Vout-
fn boost_layout(values: &ComponentValues) -> Vec<SchematicElement> {
    let mid_y = 0.0;
    let ground_y = 80.0;
    let mut elements = Vec::new();

    let x_start = 20.0;
    let x_inductor = 80.0;
    let x_switch = 150.0;
    let x_diode = 200.0;
    let x_cap = 230.0;
    let x_load = 280.0;
    let x_end = 330.0;

    // Top wire
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_start, mid_y),
        to: Pos::new(x_inductor, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_inductor, mid_y),
        to: Pos::new(x_switch, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, mid_y),
        to: Pos::new(x_diode, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_diode, mid_y),
        to: Pos::new(x_cap, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_cap, mid_y),
        to: Pos::new(x_load, mid_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_load, mid_y),
        to: Pos::new(x_end, mid_y),
    });

    // Source
    elements.push(SchematicElement::Source {
        pos: Pos::new(x_start - 10.0, mid_y - 30.0),
        label: "Vin".to_owned(),
        value: values.vin.clone(),
    });

    // Inductor
    elements.push(SchematicElement::Inductor {
        pos: Pos::new(x_inductor, mid_y),
        label: "L".to_owned(),
        value: values.inductance.clone(),
    });

    // Switch (goes down to ground)
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, mid_y),
        to: Pos::new(x_switch, mid_y + 5.0),
    });
    elements.push(SchematicElement::Switch {
        pos: Pos::new(x_switch, mid_y + 5.0),
        label: "SW".to_owned(),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, mid_y + 25.0),
        to: Pos::new(x_switch, ground_y),
    });

    // Diode
    elements.push(SchematicElement::Diode {
        pos: Pos::new(x_diode, mid_y),
        label: "D".to_owned(),
    });

    // Output capacitor
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_cap, mid_y),
        to: Pos::new(x_cap, mid_y + 15.0),
    });
    elements.push(SchematicElement::Capacitor {
        pos: Pos::new(x_cap, mid_y + 15.0),
        label: "C".to_owned(),
        value: values.capacitance.clone(),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_cap, mid_y + 35.0),
        to: Pos::new(x_cap, ground_y),
    });

    // Load
    elements.push(SchematicElement::Resistor {
        pos: Pos::new(x_load, mid_y),
        label: "R".to_owned(),
        value: values.load.clone(),
    });

    // Ground
    elements.push(SchematicElement::Ground {
        pos: Pos::new(x_start, ground_y),
    });
    elements.push(SchematicElement::Ground {
        pos: Pos::new(x_switch, ground_y),
    });
    elements.push(SchematicElement::Ground {
        pos: Pos::new(x_cap, ground_y),
    });

    // Bottom wire
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_start, ground_y),
        to: Pos::new(x_switch, ground_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_switch, ground_y),
        to: Pos::new(x_cap, ground_y),
    });
    elements.push(SchematicElement::Wire {
        from: Pos::new(x_cap, ground_y),
        to: Pos::new(x_end, ground_y),
    });

    // Labels
    elements.push(SchematicElement::Label {
        pos: Pos::new(x_end + 20.0, mid_y - 5.0),
        text: format!("Vout = {}", values.vout),
    });

    elements
}

/// VSI single-phase schematic layout (simplified H-bridge).
fn vsi_layout(values: &ComponentValues) -> Vec<SchematicElement> {
    let mut elements = Vec::new();
    let mid_y = 0.0;
    let top_y = -80.0;
    let bot_y = 80.0;

    // Just a simplified representation
    elements.push(SchematicElement::Source {
        pos: Pos::new(30.0, top_y - 20.0),
        label: "Vdc".to_owned(),
        value: values.vin.clone(),
    });

    // Label for H-bridge
    elements.push(SchematicElement::Label {
        pos: Pos::new(100.0, mid_y - 30.0),
        text: "H-Bridge".to_owned(),
    });

    elements.push(SchematicElement::Label {
        pos: Pos::new(100.0, mid_y - 10.0),
        text: "PWM Inverter".to_owned(),
    });

    // Output
    elements.push(SchematicElement::Resistor {
        pos: Pos::new(200.0, mid_y),
        label: "R".to_owned(),
        value: values.load.clone(),
    });

    // Grounds
    elements.push(SchematicElement::Ground {
        pos: Pos::new(30.0, bot_y),
    });
    elements.push(SchematicElement::Ground {
        pos: Pos::new(200.0, bot_y),
    });

    // Label
    elements.push(SchematicElement::Label {
        pos: Pos::new(250.0, mid_y - 5.0),
        text: format!("Vrms = {}", values.vout),
    });

    elements
}
