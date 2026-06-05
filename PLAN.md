# Tupan — Parameter Model Restructuring (Buck Converter Focus)

## Context

The current application inherits a mixed parameter model from the Rust original. We need a clean **design-by-parameter** workflow focused on **Buck converters only** (Boost and VSI are deferred):

### The Design Workflow (Buck Converter)

The user specifies **operating conditions** and **design targets**, and the tool computes **required component values**:

| Parameter | Role | UI Control |
|---|---|---|
| **Vin** | Input voltage — user specifies | Number + slider (1-500V) |
| **Vout** | Target output voltage — user specifies | Number + slider (0.5-500V) |
| **Duty Cycle (D)** | Auto = Vout/Vin. **User can override** — if they change D, Vout recalculates from Vin×D | Number + slider (0-100%) |
| **Frequency** | Switching frequency — user specifies | Log slider + spinbox pair |
| **Iout,max** | Max output current spec — user specifies | Number + slider (0.1-100A) |
| **ΔiL (%)** | Inductor current ripple as % of Iout,max | Number + slider (1-100%) |
| **ΔVo (%)** | Output voltage ripple as % of Vout | Number + slider (0.01-50%) |

### Computed Values (auto-displayed, read-only)

| Value | Formula |
|---|---|
| **ΔiL (A)** | `= ΔiL% × Iout,max` |
| **ΔVo (V)** | `= ΔVo% × Vout` |
| **Load R** | `= Vout / Iout,max` (Ohm's law) |
| **Inductance L** | `= (Vout × (1-D)) / (ΔiL_A × f)` |
| **Capacitance C** | `= (1-D) / (8 × L × ΔVo_V × f²)` |

### Key Insight
The **component values (L, C, R)** are **outputs** of the design process, not inputs. The existing analytical engine takes L, C, R as inputs. We need to **invert the calculation**: compute L, C, R from design specs, then feed them into the analytical engine to get the full operating point (ripple, losses, efficiency).

### Scoping
- **Only Buck converter** — Boost and VSI models are frozen/deferred
- The existing `ConverterType` enum and switch stay but Boost/VSI options may produce "not available" messages
- Simulation (RK4) stays working for buck

---

## Approach

### 1. New `DesignParams` Dataclass

A clean, user-facing parameter model:
```python
@dataclass
class DesignParams:
    vin: float = 48.0
    vout: float = 12.0
    duty_cycle: float = 0.25       # derived from Vout/Vin, but user-overridable
    frequency: float = 100_000.0
    iout_max: float = 5.0
    delta_il_pct: float = 0.30     # 30%
    delta_vo_pct: float = 0.01     # 1%
```

### 2. New `Designer` Module

`tupan/domain/designer.py` — pure functions that compute required L, C, R from design params:

```
DesignParams
  │
  ├── constrain duty_cycle = clamp(Vout/Vin)
  │   (but user can override → then Vout = Vin × D)
  │
  ├── delta_il_amps = delta_il_pct × iout_max
  ├── delta_vo_volts = delta_vo_pct × vout
  ├── r_load = vout / iout_max
  ├── L = (vout × (1-D)) / (ΔiL_A × f)
  └── C = (1-D) / (8 × L × ΔVo_V × f²)
```

### 3. Simplified Pipeline

```
User edits (DesignParams)
    │
    ▼
recalculate()
    │
    ├── 1. Sync Vout ↔ Duty (whichever was just edited)
    ├── 2. Compute ΔiL(A), ΔVo(V), R, L, C
    ├── 3. Feed (vin, duty, f, L, C, R) → buck.calculate()
    └── 4. Update results panel + schematic + plots
```

### 4. Frequency: Slider + Spinbox Combo

`QSlider` (logarithmic, range 100Hz-1MHz) + `QDoubleSpinBox`, two-way bound.

### 5. Vout ↔ Duty Sync Logic

- If user edits **Vout**: `D = clamp(Vout / Vin, 0.01, 0.99)`
- If user edits **Duty**: `Vout = Vin × D`
- If user edits **Vin**: `D = clamp(Vout / Vin, 0.01, 0.99)` (Vout takes priority as the target)

---

## Files to Create/Modify

| File | Action | Notes |
|---|---|---|
| `tupan/domain/design_params.py` | **New** | `DesignParams`, `DesignResults` dataclasses |
| `tupan/domain/designer.py` | **New** | `design_buck()` — compute L, C, R from specs |
| `tupan/domain/__init__.py` | Modify | Export new types, keep ConverterParams/Results |
| `tupan/app/state.py` | **Rewrite** | Replace params with DesignParams, new recalculate() |
| `tupan/ui/slider_spinbox.py` | **New** | SliderSpinBox reusable widget |
| `tupan/ui/param_panel.py` | **Rewrite** | New layout with all design params |
| `tupan/ui/result_panel.py` | **Rewrite** | Show computed values section + results |
| `tupan/ui/workspace.py` | Modify | Update wiring for new state |
| `tupan/ui/converter_selector.py` | Modify | Disable/note for non-Buck converters |
| `tupan/tests/test_designer.py` | **New** | Tests for designer module |
| `tupan/tests/test_buck.py` | Update | Adapt to new API |

---

## UI Layout (Left Panel)

```
┌─────────────────────────────────┐
│ Converter: [Buck]                │
├─────────────────────────────────┤
│ ⚡ Input Conditions              │
│ ─────────────────────────────── │
│ Vin:       48.0 V      [slider] │
│ Vout:      12.0 V      [slider] │
│ Duty:      25.0 %      [slider] │
│ ─────────────────────────────── │
│ Freq:  100.0 kHz  [===●=======] │  ← slider + spinbox
├─────────────────────────────────┤
│ 📐 Design Targets                │
│ ─────────────────────────────── │
│ Iout,max:   5.0 A      [slider] │
│ ΔiL:       30.0 %      [slider] │
│ ΔVo:        1.0 %      [slider] │
├─────────────────────────────────┤
│ 🔧 Computed Components           │
│ ─────────────────────────────── │
│ ΔiL:  1.500 A  (ripple current) │
│ ΔVo:  0.120 V  (ripple voltage) │
│ L:   60.000 μH  (inductance)    │
│ C:   13.158 μF  (capacitance)   │
│ R:    2.400 Ω   (load)          │
├─────────────────────────────────┤
│ 🧮 Numerical sim [☐]            │
└─────────────────────────────────┘
```

---

## Implementation Steps

### Step 1: Create `DesignParams` + `Designer` module
- [ ] `tupan/domain/design_params.py` — `DesignParams`, `DesignResults` dataclasses
- [ ] `tupan/domain/designer.py` — `design_buck()` pure function
- [ ] Tests for designer

### Step 2: Update State + Recalculate pipeline
- [ ] Rewrite `tupan/app/state.py` — DesignParams-based, sync Vout↔Duty, compute L,C,R
- [ ] Update `tupan/domain/__init__.py` exports

### Step 3: Create `SliderSpinBox` widget
- [ ] `tupan/ui/slider_spinbox.py` — reusable QWidget with log slider + spinbox

### Step 4: Rewrite Param Panel
- [ ] `tupan/ui/param_panel.py` — new layout matching the design spec
- [ ] SliderSpinBox for frequency
- [ ] Computed components section

### Step 5: Rewrite Result Panel
- [ ] `tupan/ui/result_panel.py` — show results based on DesignParams pipeline

### Step 6: Update Workspace + Selector
- [ ] `tupan/ui/workspace.py` — connect new signals
- [ ] `tupan/ui/converter_selector.py` — buck-only focus

### Step 7: Tests + Verification
- [ ] New designer tests
- [ ] Update existing buck tests
- [ ] Run full test suite
- [ ] Manual UI verification

---

## Verification

```bash
# Tests
poetry run pytest tupan/tests/ -v

# Manual UI check
poetry run python -m tupan
```

**Checklist:**
- [ ] Vin=48, Vout=12 → Duty auto=25%, L, C, R computed and displayed
- [ ] Edit Duty to 50% → Vout recalculates to 24V, L, C, R update
- [ ] Edit Vout back to 12 → Duty recalculates to 25%
- [ ] Change Iout,max from 5A to 10A → L goes down (less ΔiL in A), R goes down
- [ ] Change ΔiL from 30% to 50% → L goes down (more ripple allowed)
- [ ] Change Frequency from 100kHz to 200kHz → L and C both go down
- [ ] Numerical simulation toggle → RK4 waveforms overlay
- [ ] Theme toggle works, SVG export works, Save/Load works
- [ ] Boost/VSI show "not available" or are hidden
