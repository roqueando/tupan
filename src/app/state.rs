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
