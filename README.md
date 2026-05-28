# Tupan ⚡

**Interactive Power Electronics Workbench** — built with Rust + egui.

Tupan is a desktop application for electrical engineering, focused on power electronics. It provides an interactive visual environment where you can adjust circuit parameters with sliders and see schematics, plots, and metrics update in real time — no coding required.

## Features

### Supported Converters
- **Buck Converter** — step-down DC-DC with full analytical model
- **Boost Converter** — step-up DC-DC with full analytical model
- **Single-Phase VSI** — voltage source inverter with sine-triangle PWM

### Real-Time Calculations
Every parameter change instantly recalculates:
- Output voltage and current
- Inductor current ripple (peak-to-peak)
- Output voltage ripple (peak-to-peak)
- Conduction and switching losses
- Efficiency with color-coded indicator
- THD (for VSI)

### Numerical Simulation
Enable the "Numerical Simulation" checkbox to run an RK4 time-domain simulation:
- Buck/Boost: inductor current and capacitor voltage waveforms
- VSI: output current waveform with PWM switching

### Visual Elements
- **Parameter Panel** — sliders and inputs with tooltips for every parameter
- **Schematic View** — functional circuit diagram with component values annotated
- **Waveform Plots** — Vout and inductor current (analytical + simulation)
- **Results Panel** — all computed metrics with SI-prefix formatting

### Persistence & Export
- **Save/Load** project state as JSON (`project.tupan.json`)
- **Export SVG** — export the schematic as a scalable vector graphic

## Quick Start

```sh
cargo run
```

### Usage
1. Select converter type (Buck, Boost, or VSI) using the tabs in the left panel
2. Adjust parameters with sliders — all results update instantly
3. Toggle "Numerical Simulation" for time-domain waveforms
4. Click "Save" to persist your project, "Export SVG" to save the schematic
5. Toggle dark/light theme with 🌙/☀️ button

## Architecture

```
src/
├── main.rs                 # Entry point
├── app/                    # Application state and persistence
│   ├── state.rs            # AppState, ConverterParams, ConverterResults
│   ├── persistence.rs      # JSON save/load, SVG export
│   └── mod.rs              # TupanApp, toolbar, event loop
├── domain/                 # Pure functional engineering models
│   ├── converters/         # Buck, Boost analytical calculations
│   ├── inverter/           # VSI, PWM generation
│   ├── components/         # Inductor, capacitor, load design
│   └── metrics/            # Ripple, efficiency, THD
├── simulation/             # Numerical simulation
│   ├── integrator.rs       # Runge-Kutta 4 (RK4)
│   └── circuit_odes.rs     # ODE systems for each converter
├── schematic/              # Circuit diagram rendering
│   ├── layout.rs           # Pre-defined component positions
│   ├── primitives.rs       # Schematic element types
│   └── export_svg.rs       # SVG export
└── ui/                     # egui interface panels
    ├── workspace.rs        # Main layout (3-panel split)
    ├── param_panel.rs      # Parameter sliders
    ├── result_panel.rs     # Computed metrics
    ├── plot_panel.rs       # Waveform plots (egui_plot)
    ├── schematic_view.rs   # egui::Painter schematic rendering
    └── converter_selector.rs  # Buck/Boost/VSI tabs
```

### Key Design Decisions
- **Pure domain layer**: `domain/` modules have no egui dependency — just structs and pure functions
- **Single source of truth**: `AppState` holds all state; `recalculate()` dispatches to the correct converter
- **Immediate mode rendering**: egui's 60 FPS loop provides automatic real-time updates
- **SI-prefix formatting**: values displayed with μ, m, k, M prefixes automatically

## Technical Notes

- Built with **eframe** (egui framework) and **egui_plot** for graphics
- **~4400 lines of Rust**, 59 unit tests
- Analytical models assume Continuous Conduction Mode (CCM)
- Loss models use typical component parameters (100 mΩ Rds(on), 20ns switching times)
- PWM: bipolar sine-triangle modulation

## License

MIT
