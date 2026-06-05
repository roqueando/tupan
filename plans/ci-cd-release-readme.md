# CI/CD Pipeline, Release, and User-Focused README

## Context
We need three things:
1. **GitHub Actions CI/CD** — build a Nuitka standalone executable on pushes/tags
2. **GitHub Releases** — attach the executable as a release artifact when tags are pushed
3. **README rewrite** — user-focused documentation on how to use the application, not how to develop it

## Approach

### 1. GitHub Actions Workflow
Create `.github/workflows/build.yml`:
- Trigger on: `push` to main, `pull_request` to main, `tags: v*.*.*`
- Matrix: `macos-latest`, `ubuntu-latest` (both needed per plan: macOS + Linux)
- Steps:
  1. Checkout code
  2. Setup Python 3.11
  3. Install Poetry
  4. `poetry install`
  5. `poetry run pytest` (run tests)
  6. Build with Nuitka:
     - `nuitka --standalone --onefile --enable-plugin=pyside6 --output-dir=dist tupan/__main__.py`
  7. Upload artifact: `dist/__main__` (renamed to `tupan-macos` / `tupan-linux`)
  8. If tag push: create GitHub Release with artifacts

### 2. `.gitignore`
Add `dist/`, `*.bin`, `*.exe`, `*.app` to `.gitignore`

### 3. README — User-Focused

**Structure:**
```
# Tupan ⚡ — Power Electronics Workbench

## What is Tupan?
(one-paragraph elevator pitch: interactive buck converter designer)

## Quick Start
- Download the latest release for your OS
- Double-click to run
- No installation needed

## Screenshots
(2-3 screenshots showing the app: main window, knobs, plots)

## How to Use

### Input Conditions
- Vin: input voltage (knob + number)
- Vout: target output voltage
- Duty: auto-calculated from Vout/Vin, but editable
- Freq: switching frequency (logarithmic knob)

### Design Targets
- Iout,max: maximum load current
- ΔiL: inductor current ripple as % of Iout,max
- ΔVo: output voltage ripple as % of Vout

### Computed Components
(auto-updates as you turn knobs)
- ΔiL in Amperes, ΔVo in Volts
- Inductance L, Capacitance C, Load Resistance R

### Results Panel
- Output voltage, current, input current
- Ripple values
- Efficiency (color-coded: green > 95%, yellow > 85%, red < 85%)

### Waveform Plots
- Output voltage and inductor current over 10 switching periods
- Toggle numerical simulation for RK4 overlay

### Theme
- Toggle dark/light with the button in the toolbar

### File
- Save: saves current design to .json
- Open: loads a previous design
- Export: exports schematic as image

## Building from Source
(short section for developers)
```

**Required files:**
- Screenshots in `docs/screenshots/` directory
- `.github/workflows/build.yml`

## Steps
1. Create `.github/workflows/build.yml`
2. Create `.gitignore` entries
3. Create `docs/screenshots/` directory
4. Rewrite `README.md` as user-focused guide
5. Test workflow locally or push to validate
