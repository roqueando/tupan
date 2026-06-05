# Tupan ⚡ — Power Electronics Workbench

**Design buck converters visually.** Turn knobs, see component values and waveforms update in real time. No coding required.

Download the latest release for your OS, double-click, and start designing.

---

## Quick Start

1. **Download** the latest binary from [Releases](https://github.com/osogyian/tupan/releases)
2. **Make it executable** (macOS/Linux):
   ```sh
   chmod +x tupan-macos
   ```
3. **Run it**:
   ```sh
   ./tupan-macos
   ```
4. No installation, no Python, no dependencies needed.

---

## How to Use

### Main Window Layout

```
┌──────────────────────────────────────────────────────────┐
│ ⚡ Tupan      [🌙]  [💾 Save] [📂 Open] [📤 Export]   │
├──────────┬───────────────────────────┬──────────────────┤
│          │                           │                  │
│  Knobs   │   Schematic               │   Results        │
│  (design │                           │   ─────────      │
│   params)│   Waveform Plots          │   Vout 12.00 V   │
│          │                           │   Iout 5.00 A   │
│          │                           │   Eff   92.3%    │
│          │                           │                  │
├──────────┴───────────────────────────┴──────────────────┤
│ Computed: L=60.00μH  C=1.302μF  R=2.400Ω              │
└──────────────────────────────────────────────────────────┘
```

### Input Conditions (knobs on the left)

Turn each knob or type a value directly in the number field below it:

| Knob | What it does | Range |
|------|-------------|-------|
| **Vin** | DC input voltage | 1 – 500 V |
| **Vout** | Target output voltage | 0.5 – 500 V |
| **Duty** | Duty cycle (auto = Vout/Vin) | 1 – 99 % |
| **Frequency** | Switching frequency (log knob) | 100 Hz – 1 MHz |

**Vout ↔ Duty interaction:**
- Turn **Vout** → Duty auto-recalculates (`D = Vout / Vin`)
- Turn **Duty** → Vout auto-recalculates (`Vout = Vin × D`)
- Turn **Vin** → Duty adjusts to keep your Vout target

### Design Targets

| Knob | What it does | Range |
|------|-------------|-------|
| **Iout,max** | Maximum load current you need | 0.01 – 100 A |
| **ΔiL** | Inductor current ripple (as % of Iout,max) | 1 – 100 % |
| **ΔVo** | Output voltage ripple (as % of Vout) | 0.01 – 50 % |

### Computed Components (auto-update)

As you turn any knob, the tool immediately computes:

| Value | What it means |
|-------|-------------|
| **ΔiL (A)** | Inductor ripple current in amperes |
| **ΔVo (V)** | Output ripple voltage in volts |
| **L** | Required inductance |
| **C** | Required capacitance |
| **R** | Load resistance (Ohm's law: Vout / Iout,max) |

### Results Panel (right side)

Shows the operating point and performance:

- **Duty cycle** summary with Vout/Vin
- **Vout, Iout, Iin** — output and input currents
- **Ripple** — output voltage ripple and inductor current ripple (peak-to-peak)
- **Losses** — conduction and switching losses
- **Efficiency** — color-coded:
  - 🟢 **Green** (> 95%) — excellent
  - 🟡 **Yellow** (> 85%) — acceptable
  - 🔴 **Red** (< 85%) — needs improvement

### Waveform Plots (center bottom)

- **Output voltage** over 10 switching periods with ripple detail
- **Inductor current** triangular waveform
- Enable **"Numerical simulation"** checkbox for an RK4 time-domain overlay (more accurate waveform)

### Schematic (center top)

Auto-generated circuit diagram using your current component values. Updates as you change parameters.

---

## Toolbar

| Button | Action |
|--------|--------|
| 🌙 / ☀️ | Toggle dark/light theme |
| 💾 Save | Save your current design to a `.json` file |
| 📂 Open | Load a previously saved design |
| 📤 Export | Export the schematic as a PNG image |

---

## Tips

- **Start with defaults**: Vin=48V, Vout=12V, Iout,max=5A — then tweak
- **Higher frequency** → smaller L and C (cheaper components) but more switching losses
- **Higher ΔiL** → smaller inductor but more ripple current
- **Higher ΔVo** → smaller capacitor but more ripple voltage
- Click the **Numerical simulation** checkbox to see RK4 waveforms overlay

---

## Building from Source

```sh
git clone https://github.com/osogyian/tupan.git
cd tupan
pip install poetry
poetry install
poetry run python -m tupan
```

Run tests:
```sh
poetry run pytest
```

Build standalone executable with Nuitka:
```sh
poetry run nuitka --standalone --onefile --enable-plugin=pyside6 tupan/__main__.py
```

---

## License

MIT
