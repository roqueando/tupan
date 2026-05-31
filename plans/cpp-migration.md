# Tupan C++ Migration Plan — Rust/EGUI → C++/wxWidgets

## Context

The Tupan project is an **Interactive Power Electronics Workbench** — a desktop application for electrical engineering focused on power electronics. It provides an interactive visual environment where users adjust circuit parameters with sliders and see schematics, plots, and metrics update in real time.

The current codebase is **~4400 lines of Rust** using `eframe`/`egui` (immediate-mode GUI) and `egui_plot` for plotting. The goal is to **port the entire application to C++** using **CMake** as the build system, **wxWidgets** for the GUI framework, and a plotting library for waveform visualization.

### Current Architecture (Rust)

```
src/
├── main.rs                   # Entry point, eframe::run_native
├── app/
│   ├── mod.rs                # TupanApp: toolbar + dispatches to component_canvas
│   ├── state.rs              # AppState, SharedParams, ComponentCanvasState
│   ├── persistence.rs        # JSON save/load, SVG export
│   └── commands.rs           # AppCommand enum (notebook-related, unused)
├── domain/                   # Pure engineering models (no egui dependency)
│   ├── mod.rs                # ConverterType enum, ConverterParams, ConverterResults
│   ├── converters/
│   │   ├── buck.rs           # Buck analytical model (all formulas)
│   │   ├── boost.rs          # Boost analytical model
│   │   └── common.rs         # Shared utilities (clamp, angular_freq, etc.)
│   ├── inverter/
│   │   ├── vsi_single.rs     # VSI analytical model
│   │   └── pwm.rs            # PWM generation (sine-triangle)
│   ├── components/
│   │   ├── inductor.rs       # Inductor design calculations
│   │   ├── capacitor.rs      # Capacitor design calculations
│   │   └── load.rs           # Load models (resistive, RL, RC)
│   └── metrics/
│       ├── efficiency.rs     # Efficiency, MOSFET/diode losses
│       ├── ripple.rs         # Critical inductance, min capacitance
│       └── thd.rs            # THD calculation
├── simulation/
│   ├── integrator.rs         # RK4 integrator
│   └── circuit_odes.rs       # ODE systems (Buck, Boost, VSI)
├── schematic/
│   ├── layout.rs             # Component positions per converter type
│   ├── primitives.rs         # SchematicElement enum (Source, Inductor, etc.)
│   ├── renderer.rs           # Draw elements on egui Painter
│   └── export_svg.rs         # SVG string generation
├── ui/
│   ├── component_canvas.rs   # MAIN UI: drag-drop canvas with blocks
│   └── mod.rs
├── reactive/                 # Stub — dependency graph (not used)
├── runtime/                  # Python kernel runtime (notebook feature)
├── execution/                # Stub — execution scheduler
└── notebook/                 # Notebook model, persistence, IDs
```

## Approach

### Strategy

We will **recreate the application in C++ from scratch**, following the same clean separation of concerns:

1. **Domain layer** (`domain/`) — Pure C++ functions and structs, no wxWidgets dependency. Direct port of the Rust domain code.
2. **Simulation layer** (`simulation/`) — RK4 integrator and ODE systems. Same pure C++ approach.
3. **Schematic layer** (`schematic/`) — Primitives, layout definitions, rendering abstraction, SVG export.
4. **UI layer** (`ui/`) — wxWidgets panels: parameter sliders, result display, schematic view, plot panel.
5. **App layer** (`app/`) — Main application state, persistence, event wiring.

### UI Framework Mapping (egui → wxWidgets)

| egui Concept | wxWidgets Equivalent |
|---|---|
| `egui::Ui`, `CentralPanel`, `Panel::left/right` | `wxPanel` + `wxBoxSizer` / `wxSplitterWindow` |
| `egui::Slider` | `wxSlider` + `wxTextCtrl` (for numeric display) |
| `egui::DragValue` | `wxSpinCtrlDouble` |
| `egui::SelectableLabel` (tabs) | `wxNotebook` or `wxRadioBox` |
| `egui::Painter` (custom drawing) | `wxDC` / `wxGCDC` in `wxPaintDC` |
| `egui_plot::Plot` | **mpMath** (`mpWindow`) or **wxPlotCtrl** or **wxMathPlot** |
| `egui::ScrollArea` | `wxScrolledWindow` |
| `egui::Checkbox` | `wxCheckBox` |
| `egui::Button` | `wxButton` |
| `egui::Label` / `RichText` | `wxStaticText` |
| `eframe::App::ui()` callback | `wxFrame` + event handlers + `wxTimer` for real-time updates |
| Immediate mode (60 FPS loop) | Retained mode with `wxTimer` (refresh at ~30-60 FPS) |
| `ctx.request_repaint()` | `wxWindow::Refresh()` |
| Theme (Dark/Light) | Custom color management via `wxSystemSettings` or manual palette |
| SVG export | String concatenation (same logic as Rust) |
| JSON persistence | **nlohmann/json** or **pugixml** / manual JSON |

### Plot Library Choice

The primary candidates for C++ plotting are:
1. **mpMath** (`wxMathPlot`) — Mature, GPL-licensed, works directly with wxWidgets, supports line plots with multiple series, zoom, grid, legends. **Recommended.**
2. **PlotLib** — Lightweight, but less feature-rich.
3. Custom OpenGL plotting — Overkill for this project.

**Decision: Use wxMathPlot (mpMath)** — it's the standard plotting library for wxWidgets, supports everything we need (line plots, multiple datasets, zoom, legends, axis labels).

### Dependencies (CMake)

All dependencies fetched via CMake's `FetchContent` for zero manual setup:

```cmake
cmake_minimum_required(VERSION 3.20)
project(tupan VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# wxWidgets — system package (no FetchContent, relies on find_package)
find_package(wxWidgets REQUIRED COMPONENTS core base adv)

# nlohmann_json — header-only, fetched automatically
include(FetchContent)
FetchContent_Declare(nlohmann_json
    GIT_REPOSITORY https://github.com/nlohmann/json.git
    GIT_TAG v3.11.3)
FetchContent_MakeAvailable(nlohmann_json)

# wxMathPlot — fetched from GitHub, compiled as part of the project
FetchContent_Declare(wxmathplot
    GIT_REPOSITORY https://github.com/liquidassembler/wxMathPlot.git
    GIT_TAG master)  # or pin to a specific commit
FetchContent_MakeAvailable(wxmathplot)
```

### File Structure (C++)

```
tupan/
├── CMakeLists.txt              # Root CMake
├── README.md
├── src/
│   ├── main.cpp                # wxApp entry point
│   ├── app/
│   │   ├── AppState.h/cpp      # Application state (replaces app/state.rs)
│   │   ├── Persistence.h/cpp   # JSON save/load, SVG export
│   │   └── TupanApp.h/cpp      # Main wxApp-derived class
│   ├── domain/
│   │   ├── Types.h             # ConverterType, ConverterParams, ConverterResults
│   │   ├── converters/
│   │   │   ├── Common.h/cpp    # Clamp, angular_frequency, switching_period
│   │   │   ├── Buck.h/cpp      # Buck analytical calculations
│   │   │   └── Boost.h/cpp     # Boost analytical calculations
│   │   ├── inverter/
│   │   │   ├── VsiSingle.h/cpp # VSI analytical model
│   │   │   └── Pwm.h/cpp       # PWM generation
│   │   ├── components/
│   │   │   ├── Inductor.h/cpp  # Inductor design
│   │   │   ├── Capacitor.h/cpp # Capacitor design
│   │   │   └── Load.h/cpp      # Load models
│   │   └── metrics/
│   │       ├── Efficiency.h/cpp # Loss calculations
│   │       ├── Ripple.h/cpp    # Critical L/C calculations
│   │       └── Thd.h/cpp       # THD calculation
│   ├── simulation/
│   │   ├── Integrator.h/cpp    # RK4 integrator
│   │   └── CircuitOdes.h/cpp   # ODE systems for Buck, Boost, VSI
│   ├── schematic/
│   │   ├── Primitives.h        # SchematicElement enum/classes
│   │   ├── Layout.h/cpp        # Element positions per converter
│   │   ├── Renderer.h/cpp      # Draw elements onto wxDC
│   │   └── ExportSvg.h/cpp     # SVG string generation
│   ├── ui/
│   │   ├── MainFrame.h/cpp     # Top-level wxFrame (toolbar, 3-panel layout)
│   │   ├── ParamPanel.h/cpp    # Left panel: sliders, params
│   │   ├── ResultPanel.h/cpp   # Right panel: metrics display
│   │   ├── SchematicPanel.h/cpp# Center-top: schematic drawing
│   │   ├── PlotPanel.h/cpp     # Center-bottom: waveform plots
│   │   └── ConverterSelector.h/cpp # Converter type tabs/buttons
│   └── utils/
│       ├── Formatting.h/cpp    # SI prefix formatting (Rust format_value equivalent)
│       └── Theme.h/cpp         # Dark/Light theme colors
```

*(wxMathPlot and nlohmann_json are fetched automatically by CMake FetchContent, no manual third_party/ directory needed.)*
```

## Detailed Mapping: Rust → C++

### 1. Domain Types (`domain/Types.h`)

**Rust:**
```rust
pub enum ConverterType { Buck, Boost, VsiSinglePhase }
pub struct ConverterParams { vin, vout_target, frequency, duty_cycle, inductance, capacitance, load_resistance, modulation_index, output_frequency }
pub struct ConverterResults { vout, iout, iin, vout_ripple, il_ripple, conduction_losses, switching_losses, efficiency, thd, rms_output, fundamental_amplitude }
```

**C++:**
```cpp
enum class ConverterType { Buck, Boost, VsiSinglePhase };
struct ConverterParams { /* same fields as f64 */ };
struct ConverterResults { /* same fields, thd/rms_output/fundamental_amplitude as std::optional<double> */ };
```

### 2. Domain Calculations — Direct Port

Each Rust `pub fn calculate(params: &ConverterParams) -> ConverterResults` becomes:
```cpp
ConverterResults calculate(const ConverterParams& params);
```

All pure functions, formulas remain identical. Unit tests ported to C++ using any test framework (Catch2, GoogleTest, or doctest).

### 3. App State (`app/AppState.h`)

The Rust `AppState` combined from `app/mod.rs` and `app/state.rs` becomes a C++ class:
```cpp
class AppState {
public:
    // State
    ConverterParams params;
    ConverterResults results;
    std::optional<SimulationResult> simResults;
    CanvasState canvasState;  // from component_canvas state
    
    // UI state
    bool showNumericalSim = false;
    std::string statusMessage;
    Theme theme = Theme::Dark;
    
    // Methods
    void recalculate();  // dispatches to correct converter's calculate()
    void runSimulation(); // runs RK4 if numerical sim is enabled
};
```

### 4. UI Layout (Retained Mode with wxWidgets)

The Rust immediate-mode layout:
```
Toolbar (top) → app/mod.rs
├── Left panel (params) → component_canvas sidebar
├── Center (canvas with schematic + plots)
└── Status bar (bottom)
```

Becomes a wxWidgets retained-mode layout in `MainFrame`:
```
wxFrame
├── wxMenuBar / wxToolBar (top)
├── wxSplitterWindow (horizontal)
│   ├── wxPanel (left) = Parameter panel + Converter selector
│   └── wxSplitterWindow (vertical)
│       ├── wxPanel (center-top) = SchematicView (custom wxPanel with Paint)
│       └── wxPanel (center-bottom) = PlotPanel (wxMathPlot mpWindow)
├── wxPanel (right) = ResultPanel
└── wxStatusBar (bottom)
```

### 5. Real-time Updates

The Rust immediate-mode loop updates everything at 60 FPS automatically. In wxWidgets, we simulate this with:

```cpp
// In MainFrame constructor:
wxTimer* refreshTimer = new wxTimer(this, ID_REFRESH_TIMER);
refreshTimer->Start(33);  // ~30 FPS

// In timer handler:
void MainFrame::OnRefreshTimer(wxTimerEvent&) {
    // Recalculate if parameters changed (compare with previous values)
    if (paramsChanged) {
        appState.recalculate();
        // Update all panel controls
        UpdateParamControls();
        UpdateResultControls();
        schematicPanel->Refresh();
        plotPanel->Refresh();
    }
}

// OR: reactive updates on each slider/control change:
void MainFrame::OnSliderChanged(wxCommandEvent&) {
    // Read slider value → update AppState → recalculate → refresh all panels
    appState.recalculate();
    schematicPanel->Refresh();
    plotPanel->Refresh();
    UpdateResultPanel();
}
```

**Recommended approach:** Update on every control change (slider drag, spin ctrl change) rather than a timer loop. This is more responsive and avoids wasted CPU cycles.

### 6. Schematic Rendering

The Rust `schematic/renderer.rs` draws primitives using `egui::Painter`:
```rust
painter.line_segment([p1, p2], stroke);
painter.circle_stroke(center, radius, stroke);
painter.text(pos, align, text, font, color);
```

C++ equivalent in `SchematicPanel::OnPaint(wxPaintEvent&)`:
```cpp
wxPaintDC dc(this);
// or wxGCDC for antialiasing
dc.SetPen(wxPen(wxColour(255,255,255), 2));
dc.DrawLine(x1, y1, x2, y2);
dc.DrawCircle(cx, cy, radius);
dc.DrawText(label, x, y);
```

The `SchematicElement` enum directly maps to if/switch cases drawing on `wxDC`.

### 7. Plotting

The Rust `egui_plot::Plot` usage:
```rust
Plot::new("plot_id")
    .legend(Legend::default())
    .height(140)
    .show(ui, |pu| {
        pu.line(Line::new("iL [A]", PlotPoints::from(data)).color(c).width(1.5));
    });
```

C++ with wxMathPlot (`mpWindow`):
```cpp
mpWindow* plot = new mpWindow(this, wxID_ANY);
mpScaleX* xAxis = new mpScaleX(wxT("Time [ms]"), mpALIGN_BORDER_BOTTOM);
mpScaleY* yAxis = new mpScaleY(wxT("Value"), mpALIGN_ALIGN_BORDER_LEFT);
plot->AddLayer(xAxis);
plot->AddLayer(yAxis);

mpLineLayer* line = new mpLineLayer(data, wxColour(100,200,255));
line->SetContinuity(true);
plot->AddLayer(line);
plot->Fit();
```

### 8. Persistence (JSON)

Rust uses `serde_json` with derive macros for automatic serialization.
C++ will use **nlohmann/json** with manual `to_json`/`from_json` functions:

```cpp
#include <nlohmann/json.hpp>
using json = nlohmann::json;

void to_json(json& j, const ConverterParams& p);
void from_json(const json& j, ConverterParams& p);
```

### 9. Theme

Rust has a `Theme` enum and `ThemeColors` resolver pattern.
C++ will replicate this exactly:

```cpp
enum class Theme { Dark, Light };

struct ThemeColors {
    wxColour canvasBg, grid, sidebarBg, textPrimary, /* etc. */;
    static ThemeColors Resolve(Theme theme);
};
```

## Files to Create (Complete List)

### CMake Build System
- [ ] `CMakeLists.txt` — Root build file (uses FetchContent for wxMathPlot and nlohmann_json)

### Domain Layer (Pure C++, no wx dependency)
- [ ] `src/domain/Types.h` — ConverterType, ConverterParams, ConverterResults
- [ ] `src/domain/converters/Common.h` + `Common.cpp` — Clamp, angular_frequency, etc.
- [ ] `src/domain/converters/Buck.h` + `Buck.cpp` — All buck formulas + tests
- [ ] `src/domain/converters/Boost.h` + `Boost.cpp` — All boost formulas + tests
- [ ] `src/domain/inverter/VsiSingle.h` + `VsiSingle.cpp` — VSI analytical model
- [ ] `src/domain/inverter/Pwm.h` + `Pwm.cpp` — Sine-triangle PWM generation
- [ ] `src/domain/components/Inductor.h` + `Inductor.cpp` — Inductor design
- [ ] `src/domain/components/Capacitor.h` + `Capacitor.cpp` — Capacitor design
- [ ] `src/domain/components/Load.h` + `Load.cpp` — Load models
- [ ] `src/domain/metrics/Efficiency.h` + `Efficiency.cpp` — Loss calculations
- [ ] `src/domain/metrics/Ripple.h` + `Ripple.cpp` — Critical L/C
- [ ] `src/domain/metrics/Thd.h` + `Thd.cpp` — THD

### Simulation Layer
- [ ] `src/simulation/Integrator.h` + `Integrator.cpp` — RK4 integration
- [ ] `src/simulation/CircuitOdes.h` + `CircuitOdes.cpp` — ODE systems

### Schematic Layer
- [ ] `src/schematic/Primitives.h` — SchematicElement, Pos
- [ ] `src/schematic/Layout.h` + `Layout.cpp` — Position layouts per converter
- [ ] `src/schematic/Renderer.h` + `Renderer.cpp` — Draw onto wxDC
- [ ] `src/schematic/ExportSvg.h` + `ExportSvg.cpp` — SVG generation

### App Layer
- [ ] `src/app/AppState.h` + `AppState.cpp` — Complete application state
- [ ] `src/app/Persistence.h` + `Persistence.cpp` — JSON save/load, SVG export

### UI Layer (wxWidgets)
- [ ] `src/ui/MainFrame.h` + `MainFrame.cpp` — Main window with toolbar + 3-panel layout
- [ ] `src/ui/ParamPanel.h` + `ParamPanel.cpp` — Parameter sliders + converter selector
- [ ] `src/ui/ResultPanel.h` + `ResultPanel.cpp` — Metrics display
- [ ] `src/ui/SchematicPanel.h` + `SchematicPanel.cpp` — Custom paint schematic
- [ ] `src/ui/PlotPanel.h` + `PlotPanel.cpp` — Waveform plots (wxMathPlot)

### Utilities
- [ ] `src/utils/Formatting.h` + `Formatting.cpp` — SI prefix formatting
- [ ] `src/utils/Theme.h` + `Theme.cpp` — Dark/Light theme colors

### Entry Point
- [ ] `src/main.cpp` — wxApp subclass, frame creation

### Tests (optional but recommended)
- [ ] `tests/test_domain_buck.cpp`
- [ ] `tests/test_domain_boost.cpp`
- [ ] `tests/test_domain_vsi.cpp`
- [ ] `tests/test_simulation_integrator.cpp`
- [ ] `tests/test_schematic_export_svg.cpp`
- [ ] `CMakeLists.txt` in tests/ (if using a test framework)

## Steps (Implementation Order)

### Phase 1 — Project Scaffold & Domain Port
- [ ] **Step 1: CMake project setup** — Create root CMakeLists.txt, check wxWidgets and nlohmann_json availability, set up FetchContent for wxMathPlot
- [ ] **Step 2: Domain types** — Port `Types.h` (ConverterType, ConverterParams, ConverterResults). Port `Common.h/cpp` (clamp, angular_frequency, etc.)
- [ ] **Step 3: Converter models** — Port `Buck.h/cpp`, `Boost.h/cpp`, `VsiSingle.h/cpp`, `Pwm.h/cpp` with all formulas. Keep functions pure, no UI dependency.
- [ ] **Step 4: Component models** — Port `Inductor.h/cpp`, `Capacitor.h/cpp`, `Load.h/cpp`
- [ ] **Step 5: Metrics** — Port `Efficiency.h/cpp`, `Ripple.h/cpp`, `Thd.h/cpp`
- [ ] **Verify:** Domain layer compiles and produces correct numerical results matching Rust tests.

### Phase 2 — Simulation & Schematic Port
- [ ] **Step 6: Integrator** — Port `Integrator.h/cpp` (RK4 step + integration loop)
- [ ] **Step 7: Circuit ODEs** — Port `CircuitOdes.h/cpp` (BuckOde, BoostOde, VsiOde)
- [ ] **Step 8: Schematic primitives** — Port `Primitives.h` (Pos, SchematicElement enum)
- [ ] **Step 9: Schematic layout** — Port `Layout.h/cpp` (buck_layout, boost_layout, vsi_layout)
- [ ] **Step 10: SVG export** — Port `ExportSvg.h/cpp` (string building)
- [ ] **Verify:** Schematic layout + SVG export produce correct output matching Rust.

### Phase 3 — App State & Utilities
- [ ] **Step 11: Formatting utility** — Port `Formatting.h/cpp` (SI prefix formatting)
- [ ] **Step 12: Theme colors** — Port `Theme.h/cpp` (Theme enum, ThemeColors struct)
- [ ] **Step 13: AppState** — Port `AppState.h/cpp` with `recalculate()`, simulation dispatch
- [ ] **Step 14: Persistence** — Port `Persistence.h/cpp` (JSON save/load, SVG file export)
- [ ] **Verify:** AppState can be constructed, recalculates, serializes/deserializes.

### Phase 4 — wxWidgets UI Shell
- [ ] **Step 15: Main entry** — Create `main.cpp` with `wxApp` subclass
- [ ] **Step 16: MainFrame** — Create `MainFrame.h/cpp` with wxToolBar, three-panel splitter layout, status bar, menu bar (File: Save/Load/Export SVG)
- [ ] **Step 17: ParamPanel** — Create `ParamPanel.h/cpp` with converter selector (wxRadioBox or wxNotebook) + parameter sliders (wxSlider + wxSpinCtrlDouble for each param)
- [ ] **Step 18: ResultPanel** — Create `ResultPanel.h/cpp` with static text display of all metrics, color-coded efficiency
- [ ] **Step 19: SchematicPanel** — Create `SchematicPanel.h/cpp` with `OnPaint` handler that uses `Renderer` to draw schematic elements on `wxDC`
- [ ] **Step 20: PlotPanel** — Create `PlotPanel.h/cpp` wrapping `mpWindow` from wxMathPlot, populate with analytical waveform data + simulation overlay
- [ ] **Step 21: Wire up events** — Connect slider change events → AppState::recalculate() → refresh all panels

### Phase 5 — Real-time Updates & Polish
- [ ] **Step 22: Numerical simulation toggle** — wxCheckBox in ParamPanel to enable RK4 simulation
- [ ] **Step 23: Theme toggle** — Menu/toolbar button to switch dark/light theme
- [ ] **Step 24: Persistence UI** — File menu Save/Load dialog boxes
- [ ] **Step 25: SVG export** — File → Export SVG writes schematic to file
- [ ] **Step 26: Status bar messages** — Display status updates in wxStatusBar

### Phase 6 — Testing & Verification
- [ ] **Step 27: Manual testing** — Run app, test all three converters, verify all sliders update results in real time
- [ ] **Step 28: Cross-platform build check** — Verify on Linux (gcc/clang) and Windows (MSVC)
- [ ] **Step 29: Performance check** — Ensure numerical simulation doesn't block UI (run in background thread if needed)

## Verification

1. **Build:** `mkdir build && cd build && cmake .. && make` compiles cleanly
2. **Launch:** Application opens with the 3-panel layout (params left, schematic+plots center, results right)
3. **Converter selection:** Toggle between Buck, Boost, VSI — all params, schematic, plots update
4. **Parameter editing:** Each slider updates the result panel, schematic annotations, and plots in real time
5. **Numerical simulation:** Toggle enables RK4 simulation overlay on waveform plots
6. **Theme:** Dark/Light toggle switches all panel colors
7. **Persistence:** Save project to JSON → close app → reopen → Load → all state restored
8. **SVG export:** File → Export SVG produces a valid SVG file matching the on-screen schematic
9. **Numerical accuracy:** Results match the Rust version's output for identical inputs

## Key Technical Decisions

| Decision | Choice | Rationale |
|---|---|---|
| GUI Framework | wxWidgets 3.2+ | Mature, cross-platform (Linux/Windows/macOS), native look |
| Build System | CMake 3.20+ | Industry standard, works with wxWidgets |
| JSON Library | nlohmann/json (via FetchContent) | Header-only, easy to use, matches serde_json ergonomics |
| Plot Library Fetch | CMake FetchContent | Auto-downloads wxMathPlot from GitHub, no manual third_party/ needed |
| Plot Library | wxMathPlot (mpMath) | Direct wxWidgets integration, supports all needed plot features |
| Test Framework | doctest or Catch2 | Lightweight, header-only options available |
| C++ Standard | C++17 | std::optional for nullable results, structured bindings, if constexpr |
| Drawing | wxGCDC (anti-aliased) | Better visual quality than wxPaintDC for schematic rendering |
| Real-time Updates | Event-driven (not timer) | Each slider/control change triggers recalculation + refresh |

### Why NOT Other Options

- **Qt** — Heavier dependency, different licensing model, more complex than needed
- **FLTK** — Less feature-rich, fewer widgets for engineering UIs
- **SFML/GLFW** — Game-oriented, no native widgets
- **Dear ImGui** — Immediate mode like egui, but C++; lacks native look & feel
- **matplotlib (C++ bindings)** — Heavy Python dependency, not suitable for standalone app
- **Custom OpenGL plotting** — Over-engineering for line/bar charts

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| wxMathPlot API changes or bugs | Pin to specific version, fallback to custom wxDC plotting |
| Immediate-mode → Retained-mode paradigm shift | Plan carefully: all UI state lives in AppState, panels just read from it |
| Numerical simulation blocking UI | Run RK4 in `std::thread` with atomic cancellation flag |
| wxWidgets look different per platform | Test on Linux (GTK) and Windows; use `wxGCDC` for consistent rendering |
| SVG export bugs | Port Rust SVG generation directly (string building, no XML library needed) |
| Slider precision (f64 in wxSlider) | Use `wxSpinCtrlDouble` for precision; `wxSlider` for coarse adjustment |
