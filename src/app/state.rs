use crate::domain::{ConverterParams, ConverterResults, ConverterType};
use crate::schematic::primitives::{Pos, SchematicElement};
use crate::simulation::integrator::SimulationResult;
use serde::{Deserialize, Serialize};

// ── Theme ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

// ── App tabs ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AppTab {
    Converters,
    SchematicEditor,
    ComponentCanvas,
}

impl Default for AppTab {
    fn default() -> Self {
        Self::Converters
    }
}

// ── Schematic editor tool ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchematicTool {
    Select,
    Source,
    Resistor,
    Inductor,
    Capacitor,
    Diode,
    Switch,
    Ground,
    Wire,
    Label,
}

impl SchematicTool {
    pub fn name(self) -> &'static str {
        match self {
            Self::Select => "Select",
            Self::Source => "Source",
            Self::Resistor => "Resistor",
            Self::Inductor => "Inductor",
            Self::Capacitor => "Capacitor",
            Self::Diode => "Diode",
            Self::Switch => "Switch",
            Self::Ground => "Ground",
            Self::Wire => "Wire",
            Self::Label => "Label",
        }
    }


}

// ── Schematic editor state ────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SchematicEditorState {
    /// All placed elements on the canvas
    pub elements: Vec<SchematicElement>,
    /// Currently selected tool from palette
    pub selected_tool: SchematicTool,
    /// Index of the selected element on canvas (for move/delete)
    pub selected_element: Option<usize>,
    /// Index of element being edited (double-click popup)
    pub editing_element: Option<usize>,
    /// Whether we're in wire-drawing mode (first point placed)
    pub wire_start: Option<Pos>,
    /// Canvas pan/scroll offset
    pub pan_offset: (f32, f32),
    /// Canvas zoom factor
    pub zoom: f32,
    /// Pending label text (for Label tool)
    pub pending_label_text: String,
    /// Whether we're typing label text
    pub typing_label: bool,
    /// Whether snap-to-grid is enabled
    pub snap_to_grid: bool,
    /// Clipboard for copy/paste
    pub clipboard: Option<Box<SchematicElement>>,
    /// Whether to use orthogonal (90°) wire routing
    pub orthogonal_wires: bool,
}

impl Default for SchematicEditorState {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
            selected_tool: SchematicTool::Select,
            selected_element: None,
            editing_element: None,
            wire_start: None,
            pan_offset: (0.0, 0.0),
            zoom: 1.0,
            pending_label_text: String::new(),
            typing_label: false,
            snap_to_grid: true,
            clipboard: None,
            orthogonal_wires: true,
        }
    }
}

impl SchematicEditorState {
    /// Clear all elements
    pub fn clear(&mut self) {
        self.elements.clear();
        self.selected_element = None;
        self.wire_start = None;
    }

    /// Delete the selected element
    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_element {
            if idx < self.elements.len() {
                self.elements.remove(idx);
                self.selected_element = None;
            }
        }
    }
}

// ── Main app state ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Active tab
    pub active_tab: AppTab,

    /// Active converter type
    pub active_converter: ConverterType,

    /// Parameters for the active converter
    pub params: ConverterParams,

    /// Results from analytical calculation
    pub results: ConverterResults,

    /// Results from numerical simulation (if enabled) — skipped in serialization
    #[serde(skip)]
    pub sim_results: Option<SimulationResult>,

    /// Whether numerical simulation is enabled
    pub show_numerical_sim: bool,

    /// Whether to show the schematic panel
    pub show_schematic: bool,

    /// UI theme
    pub theme: Theme,

    /// Status message displayed in the toolbar
    pub status_message: String,

    /// Schematic editor state (not serialized)
    #[serde(skip)]
    pub editor: SchematicEditorState,

    /// Component canvas state (not serialized)
    #[serde(skip)]
    pub component_canvas: ComponentCanvasState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_tab: AppTab::default(),
            active_converter: ConverterType::Buck,
            params: ConverterParams::default(),
            results: ConverterResults::zero(),
            sim_results: None,
            show_numerical_sim: false,
            show_schematic: true,
            theme: Theme::Dark,
            status_message: "ready".to_owned(),
            editor: SchematicEditorState::default(),
            component_canvas: ComponentCanvasState::default(),
        }
    }
}

impl AppState {
    /// Switch to a tab
    pub fn switch_tab(&mut self, tab: AppTab) {
        self.active_tab = tab;
        self.status_message = match tab {
            AppTab::Converters => "Converter workspace".to_owned(),
            AppTab::SchematicEditor => "Schematic editor — place components on the canvas".to_owned(),
            AppTab::ComponentCanvas => "Component Canvas — drag components to design power stage".to_owned(),
        };
    }

    /// Recalculate all results based on current params and active converter.
    pub fn recalculate(&mut self) {
        self.status_message = "calculating...".to_owned();

        self.results = match self.active_converter {
            ConverterType::Buck => crate::domain::converters::buck::calculate(&self.params),
            ConverterType::Boost => crate::domain::converters::boost::calculate(&self.params),
            ConverterType::VsiSinglePhase => {
                crate::domain::inverter::vsi_single::calculate(&self.params, true)
            }
        };

        if self.show_numerical_sim {
            self.run_simulation();
        } else {
            self.sim_results = None;
        }

        self.status_message = format!("{} — updated", self.active_converter.name());
    }

    /// Run numerical simulation for the active converter.
    pub fn run_simulation(&mut self) {
        use crate::simulation::circuit_odes::{BoostOde, BuckOde, VsiOde};
        use crate::simulation::integrator::integrate_fixed;

        match self.active_converter {
            ConverterType::Buck => {
                let ode = BuckOde::from_params(&self.params);
                let vout_est = self.results.vout;
                let iout_est = self.results.iout;
                let y0 = vec![iout_est, vout_est];

                let t_end = 10.0 / self.params.frequency;
                let dt = 1.0 / self.params.frequency / 500.0;

                let f: crate::simulation::integrator::DerivFn =
                    Box::new(move |t, y| ode.derivatives(t, y));

                self.sim_results = Some(integrate_fixed(&f, &y0, (0.0, t_end), dt, 10000));
            }
            ConverterType::Boost => {
                let ode = BoostOde::from_params(&self.params);
                let vout_est = self.results.vout;
                let iin_est = self.results.iin;
                let y0 = vec![iin_est, vout_est];

                let t_end = 10.0 / self.params.frequency;
                let dt = 1.0 / self.params.frequency / 500.0;

                let f: crate::simulation::integrator::DerivFn =
                    Box::new(move |t, y| ode.derivatives(t, y));

                self.sim_results = Some(integrate_fixed(&f, &y0, (0.0, t_end), dt, 10000));
            }
            ConverterType::VsiSinglePhase => {
                let ode = VsiOde::from_params(&self.params);
                let y0 = vec![0.0];

                let t_end = 3.0 / self.params.output_frequency;
                let dt = 1.0 / self.params.frequency / 20.0;

                let f: crate::simulation::integrator::DerivFn =
                    Box::new(move |t, y| ode.derivatives(t, y));

                self.sim_results = Some(integrate_fixed(&f, &y0, (0.0, t_end), dt, 20000));
            }
        }
    }
}

// ── Component canvas types ────────────────────────────────────────────

/// Types of components that can be placed on the component canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasComponentType {
    Vin,
    Vout,
    DutyCycle,
    Frequency,
    DeltaIl,
    IoutMax,
    DeltaVo,
    Inductor,
    Capacitor,
    Plot,
}

impl CanvasComponentType {
    pub fn name(self) -> &'static str {
        match self {
            Self::Vin => "Vin",
            Self::Vout => "Vout",
            Self::DutyCycle => "Duty Cycle",
            Self::Frequency => "Frequency",
            Self::DeltaIl => "ΔiL",
            Self::IoutMax => "Iout,max",
            Self::DeltaVo => "ΔVo",
            Self::Inductor => "Inductor (L)",
            Self::Capacitor => "Capacitor (C)",
            Self::Plot => "Plot",
        }
    }

    pub fn unit(self) -> &'static str {
        match self {
            Self::Vin => "V",
            Self::Vout => "V",
            Self::DutyCycle => "%",
            Self::Frequency => "Hz",
            Self::DeltaIl => "%",
            Self::IoutMax => "A",
            Self::DeltaVo => "%",
            Self::Inductor => "H",
            Self::Capacitor => "F",
            Self::Plot => "",
        }
    }

    /// Returns true if this component type is user-editable.
    pub fn is_editable(self) -> bool {
        matches!(
            self,
            Self::Vin
                | Self::Vout
                | Self::DutyCycle
                | Self::Frequency
                | Self::DeltaIl
                | Self::IoutMax
                | Self::DeltaVo
        )
    }

    pub fn is_plot(self) -> bool {
        matches!(self, Self::Plot)
    }

    /// Returns true if this component is computed (not directly editable).
    #[allow(dead_code)]
    pub fn is_computed(self) -> bool {
        matches!(self, Self::Inductor | Self::Capacitor)
    }
}

/// A component placed on the canvas.
#[derive(Debug, Clone)]
pub struct PlacedComponent {
    #[allow(dead_code)]
    pub id: u64,
    pub component_type: CanvasComponentType,
    pub pos: Pos,
}

/// Shared parameters that drive all computations on the component canvas.
#[derive(Debug, Clone)]
pub struct SharedParams {
    pub vin: f64,
    pub vout: f64,
    pub duty_cycle: f64,
    pub frequency: f64,
    pub delta_il: f64,
    pub iout_max: f64,
    pub delta_vo: f64,
}

impl Default for SharedParams {
    fn default() -> Self {
        Self {
            vin: 48.0,
            vout: 12.0,
            duty_cycle: 0.25,
            frequency: 100_000.0,
            delta_il: 0.3,
            iout_max: 5.0,
            delta_vo: 0.01,
        }
    }
}

impl SharedParams {
    /// Calculate inductance from the shared parameters.
    /// ΔiL in Amperes = delta_il_pct * iout_max
    /// L = (Vout * (1 - DutyCycle)) / (delta_il_amps * Frequency)
    pub fn calc_inductance(&self) -> f64 {
        if self.delta_il <= 0.0 || self.frequency <= 0.0 {
            return 0.0;
        }
        let delta_il_amps = self.delta_il * self.iout_max;
        if delta_il_amps <= 0.0 {
            return 0.0;
        }
        (self.vout * (1.0 - self.duty_cycle)) / (delta_il_amps * self.frequency)
    }

    /// Calculate delta iL in Amperes: delta_il_pct * iout_max
    pub fn calc_delta_il_amps(&self) -> f64 {
        self.delta_il * self.iout_max
    }

    /// Calculate the inductor current ripple (peak-to-peak) in Amperes
    /// for a given duty cycle, using L, Vout, Frequency.
    /// ΔiL_pp = (Vout * (1 - D)) / (L * f)
    pub fn calc_il_ripple_for_duty(&self, duty: f64, l: f64) -> f64 {
        if l <= 0.0 || self.frequency <= 0.0 {
            return 0.0;
        }
        (self.vout * (1.0 - duty)).abs() / (l * self.frequency)
    }

    /// Calculate output voltage ripple (peak-to-peak) for a given duty cycle.
    /// ΔVo_pp = (1 - D) / (8 * L * C * f²)
    pub fn calc_vo_ripple_for_duty(&self, duty: f64, l: f64, c: f64) -> f64 {
        if l <= 0.0 || c <= 0.0 || self.frequency <= 0.0 {
            return 0.0;
        }
        (1.0 - duty).abs() / (8.0 * l * c * self.frequency * self.frequency)
    }

    /// Calculate capacitance from the shared parameters.
    /// C = (1 - DutyCycle) / (8 * L * delta_vo * Frequency²)
    pub fn calc_capacitance(&self) -> f64 {
        let l = self.calc_inductance();
        if l <= 0.0 || self.delta_vo <= 0.0 || self.frequency <= 0.0 {
            return 0.0;
        }
        (1.0 - self.duty_cycle) / (8.0 * l * self.delta_vo * self.frequency * self.frequency)
    }
}

/// State for the component canvas tab.
#[derive(Debug, Clone)]
pub struct ComponentCanvasState {
    /// All placed components on the canvas
    pub placed_components: Vec<PlacedComponent>,
    /// Shared parameters driving all computations
    pub shared_params: SharedParams,
    /// Canvas pan offset (x, y)
    pub pan_offset: (f32, f32),
    /// Canvas zoom factor
    pub zoom: f32,
    /// Next ID to assign to a placed component (auto-increment)
    pub next_id: u64,
    /// Index of the selected component (for move/delete)
    pub selected_index: Option<usize>,
    /// The component type currently selected from the palette (for placing)
    pub palette_selection: Option<CanvasComponentType>,
    /// Whether we're currently editing a value inline
    pub editing_index: Option<usize>,
    /// Buffer for inline text editing
    pub edit_buffer: String,
}

impl Default for ComponentCanvasState {
    fn default() -> Self {
        Self {
            placed_components: Vec::new(),
            shared_params: SharedParams::default(),
            pan_offset: (0.0, 0.0),
            zoom: 1.0,
            next_id: 1,
            selected_index: None,
            palette_selection: None,
            editing_index: None,
            edit_buffer: String::new(),
        }
    }
}

impl ComponentCanvasState {
    pub fn clear(&mut self) {
        self.placed_components.clear();
        self.selected_index = None;
        self.editing_index = None;
    }

    pub fn delete_selected(&mut self) {
        if let Some(idx) = self.selected_index {
            if idx < self.placed_components.len() {
                self.placed_components.remove(idx);
                self.selected_index = None;
                self.editing_index = None;
            }
        }
    }

    /// Place a new component of the given type at the given position.
    pub fn place_component(&mut self, component_type: CanvasComponentType, pos: Pos) {
        let id = self.next_id;
        self.next_id += 1;
        self.placed_components.push(PlacedComponent {
            id,
            component_type,
            pos,
        });
    }

    /// Get the display value for a given component type based on current shared params.
    pub fn get_value(&self, component_type: CanvasComponentType) -> f64 {
        match component_type {
            CanvasComponentType::Vin => self.shared_params.vin,
            CanvasComponentType::Vout => self.shared_params.vout,
            CanvasComponentType::DutyCycle => self.shared_params.duty_cycle * 100.0,
            CanvasComponentType::Frequency => self.shared_params.frequency,
            CanvasComponentType::DeltaIl => self.shared_params.delta_il * 100.0,
            CanvasComponentType::IoutMax => self.shared_params.iout_max,
            CanvasComponentType::DeltaVo => self.shared_params.delta_vo * 100.0,
            CanvasComponentType::Inductor => self.shared_params.calc_inductance(),
            CanvasComponentType::Capacitor => self.shared_params.calc_capacitance(),
            CanvasComponentType::Plot => 0.0, // not a scalar value
        }
    }

    /// Set a shared param from a canvas component type and a new value.
    /// Returns true if any shared param changed.
    pub fn set_value(&mut self, component_type: CanvasComponentType, value: f64) -> bool {
        match component_type {
            CanvasComponentType::Vin => {
                if (self.shared_params.vin - value).abs() > 1e-12 {
                    self.shared_params.vin = value;
                    return true;
                }
            }
            CanvasComponentType::Vout => {
                if (self.shared_params.vout - value).abs() > 1e-12 {
                    self.shared_params.vout = value;
                    // Recalculate duty cycle: D = Vout / Vin (for buck/boost approximation)
                    if self.shared_params.vin > 0.0 {
                        self.shared_params.duty_cycle =
                            (value / self.shared_params.vin).clamp(0.0, 1.0);
                    }
                    return true;
                }
            }
            CanvasComponentType::DutyCycle => {
                let dc = (value / 100.0).clamp(0.0, 1.0);
                if (self.shared_params.duty_cycle - dc).abs() > 1e-12 {
                    self.shared_params.duty_cycle = dc;
                    // Recalculate Vout: Vout = Vin * D
                    self.shared_params.vout = self.shared_params.vin * dc;
                    return true;
                }
            }
            CanvasComponentType::Frequency => {
                if (self.shared_params.frequency - value).abs() > 1e-12 && value > 0.0 {
                    self.shared_params.frequency = value;
                    return true;
                }
            }
            CanvasComponentType::DeltaIl => {
                let pct = (value / 100.0).max(0.001);
                if (self.shared_params.delta_il - pct).abs() > 1e-12 {
                    self.shared_params.delta_il = pct;
                    return true;
                }
            }
            CanvasComponentType::IoutMax => {
                if (self.shared_params.iout_max - value).abs() > 1e-12 {
                    self.shared_params.iout_max = value;
                    return true;
                }
            }
            CanvasComponentType::DeltaVo => {
                let pct = (value / 100.0).max(0.0001);
                if (self.shared_params.delta_vo - pct).abs() > 1e-12 {
                    self.shared_params.delta_vo = pct;
                    return true;
                }
            }
            CanvasComponentType::Inductor | CanvasComponentType::Capacitor | CanvasComponentType::Plot => {
                // Computed/plot values are read-only here
            }
        }
        false
    }
}
