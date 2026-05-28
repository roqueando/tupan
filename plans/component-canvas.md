# Component Canvas — Interactive Grid with Dependent Parameter Components

## Context

We need a new tab/page in the Tupan app that provides an infinite canvas grid where users can place and interact with electrical component "blocks". These blocks show computed values that automatically update when dependent parameters change.

The key difference from the existing Schematic Editor: this is a **calculator/visualizer** for power converter component sizing, not a circuit drawing tool. The user adjusts parameters (Vin, Vout, Duty Cycle, Frequency, delta iL, Iout_max, delta Vo) and sees the resulting inductor and capacitor values change in real-time on the canvas.

### Components (Sidebar Palette)
- **Vin** — input voltage (editable number, Volts)
- **Vout** — output voltage (editable number, Volts)
- **Duty Cycle** — result of Vout/Vin, but also editable (0..1)
- **Frequency** — switching frequency (editable number, Hz)
- **ΔiL** — inductor current ripple percentage (editable, %)
- **Iout,max** — max output current (editable, Amperes)
- **ΔVo** — output voltage ripple percentage (editable, %)

### Computed Components (automatically update)
- **Inductor (L)** — computed from: `(Vout * (1 - DutyCycle)) / (ΔiL * Frequency)`
- **Capacitor (C)** — computed from: `(1 - DutyCycle) / (8 * L * ΔVo * Frequency²)`

### Canvas Behavior
- Infinite grid (pan & zoom)
- Drag-and-drop components from sidebar onto canvas
- Each placed component shows its label and value
- When a dependent parameter (e.g., Frequency) changes, all placed L and C blocks update in real-time
- Editable input fields let the user change values

## Approach

We will add a new `AppTab::ComponentCanvas` variant and a new UI module `component_canvas.rs`. The core state lives in a new `ComponentCanvasState` struct inside `AppState`. The canvas rendering reuses the infinite-grid pattern from `schematic_editor.rs` but with different interaction logic (drag from sidebar, editable fields inline on canvas blocks).

### State Model

```rust
struct ComponentCanvasState {
    placed_components: Vec<PlacedComponent>,
    shared_params: SharedParams,  // the "live" parameters that drive computations
}

struct SharedParams {
    vin: f64,
    vout: f64,        // editable; if changed, duty_cycle recalculates
    duty_cycle: f64,  // editable; if changed, vout recalculates from vin
    frequency: f64,
    delta_il: f64,    // percentage (e.g., 0.3 = 30%)
    iout_max: f64,    // Amperes
    delta_vo: f64,    // percentage
}

struct PlacedComponent {
    id: u64,
    component_type: CanvasComponentType,
    pos: Pos,         // position on canvas
}

enum CanvasComponentType {
    Vin,
    Vout,
    DutyCycle,
    Frequency,
    DeltaIl,
    IoutMax,
    DeltaVo,
    Inductor,   // computed
    Capacitor,  // computed
}
```

### Formulas
- `L = (Vout * (1 - DutyCycle)) / (delta_il * Frequency)`  — in Henries
- `C = (1 - DutyCycle) / (8 * L * delta_vo * Frequency²)` — in Farads

Note: `delta_il` and `delta_vo` are expressed as decimals (e.g., 30% = 0.30) in the formula, but shown as percentages in the UI.

## Files to Modify

| File | Change |
|------|--------|
| `src/ui/mod.rs` | Add `pub mod component_canvas;` |
| `src/ui/component_canvas.rs` | **New file** — the entire tab UI |
| `src/ui/workspace.rs` | Route to the new tab |
| `src/app/state.rs` | Add `AppTab::ComponentCanvas`, add `ComponentCanvasState` field, add `SchematicTool`? no — separate tool state |
| `src/app/mod.rs` | Add tab button for Component Canvas |

## Reuse

- **Infinite grid drawing**: copy/adapt `draw_grid()` from `schematic_editor.rs` — the grid rendering logic is identical.
- **Pan/zoom**: reuse the same pattern (pan_offset, zoom, scroll-to-zoom, drag-to-pan) from `schematic_editor.rs`.
- **`Pos` type**: reuse `src/schematic/primitives.rs::Pos` for positioning.
- **`format_eng()`**: reuse the SI-prefix formatting from `src/app/mod.rs` or `src/ui/result_panel.rs`.
- **`CanvasComponentType` rendering**: draw colored rectangles with labels and editable fields, similar to how `draw_element` works in `src/schematic/renderer.rs` but simpler.

## Steps

- [ ] **1. Add `AppTab::ComponentCanvas` variant** — in `src/app/state.rs`, add the enum variant and update `switch_tab()`.
- [ ] **2. Add `ComponentCanvasState` struct** — with `placed_components: Vec<PlacedComponent>`, `shared_params: SharedParams`, canvas transform (pan_offset, zoom), and selection state.
- [ ] **3. Add tab button** — in `src/app/mod.rs`, add a new `selectable_label` for the Component Canvas tab.
- [ ] **4. Wire up in workspace** — in `src/ui/workspace.rs`, add a match arm for the new tab.
- [ ] **5. Create `src/ui/component_canvas.rs`** — the big new module with:
  - [ ] Sidebar with component palette (Vin, Vout, Duty Cycle, Frequency, ΔiL, Iout,max, ΔVo)
  - [ ] Shared parameters editing panel (maybe a collapsible section in the sidebar)
  - [ ] Infinite grid canvas with pan/zoom
  - [ ] Drag-and-drop from palette onto canvas (click-to-place as MVP)
  - [ ] Inline editing of input values on placed components
  - [ ] Real-time computation of L and C values
  - [ ] Visual rendering of placed component blocks (rectangles with label + value)
  - [ ] Status bar with instructions

## Verification

1. Build the project: `cargo build`
2. Run: `cargo run`
3. Click the "Component Canvas" tab in the toolbar
4. Verify the canvas shows with an infinite grid and a sidebar
5. Click a sidebar component (e.g., Vin) — it should place on the canvas
6. Click it on the canvas — should be able to edit its value
7. Place Vin=48, Vout=12, DutyCycle=0.25, Frequency=100000, ΔiL=0.3, Iout,max=5, ΔVo=0.01
8. Verify computed Inductor and Capacitor values appear when those components are placed
9. Change Frequency — verify L and C update in real-time
10. Change Vout (input) — verify Duty Cycle recalculates (and L/C update)
