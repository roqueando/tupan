# UI Redesign — Modern Clean Look with Dark/Light Theme

## Context

The current UI is functional but uses raw egui defaults with hardcoded colors. We want a modern, polished look with:
- A cohesive design palette (accent blue `#6382FF`, clean dark/light backgrounds)
- Theme-aware colors everywhere (sidebar, canvas, component blocks, toolbar, plots)
- Sidebar: section headers, card-style palette items with hover/selected states, cleaner parameter rows
- Canvas: subtle grid lines, component blocks with top accent bars, rounded corners, selection glow
- Toolbar: icon + app name, cleaner spacing, theme-aware toggle
- Inline editor (selected component): same block styling but with interactive controls

## Approach

Create a central `ThemeColors` struct resolved per theme, and use it throughout the component canvas UI. Replace hardcoded `Color32` values with theme-aware palette constants. Redesign the sidebar palette as interactive cards. Give component blocks a subtle top accent line and rounded corners.

## Files to Modify

| File | Change |
|------|--------|
| `src/ui/component_canvas.rs` | Full rewrite with theme-aware colors, modern palette cards, cleaner canvas blocks, styled inline editor, refined plot block |
| `src/app/mod.rs` | Redesign toolbar with accent-colored logo, clean spacing, theme-aware toggle |

## Design Palette

```
Accent:       #6382FF (blue brand)
Accent Light: #82A0FF
Accent Dim:   #3C5AC8

Dark theme:
  Canvas BG:    #12121A
  Sidebar BG:   #161620
  Grid:         rgba(60,60,80,40)
  Input Block:  rgba(25,35,60,230)
  Computed Blk: rgba(40,25,15,230)
  Card BG:      rgba(30,35,50,180)
  Selected:     #FFD23C (gold)
  Text Primary: #DCE1F0
  Text Value:   #82BEFF

Light theme:
  Canvas BG:    #f1efe7
  Sidebar BG:   #FFFFFF
  Grid:         rgba(0,0,0,10)
  Input Block:  rgba(230,240,255,230)
  Computed Blk: rgba(255,240,225,230)
  Card BG:      rgba(240,242,248,180)
  Selected:     #FFD23C
  Text Primary: #1E2332
  Text Value:   #1E64C8
```

## Steps

- [ ] **1. Add `palette` module with all color constants** — organized by theme (Dark/Light variants)
- [ ] **2. Add `ThemeColors` struct with `resolve(theme)`** — maps Theme enum to resolved colors
- [ ] **3. Update `show_component_canvas`** — resolve colors, pass to sidebar + canvas
- [ ] **4. Redesign sidebar** — section headers, card-style palette items with hover/selected states, cleaner param rows
- [ ] **5. Redesign component blocks** — rounded corners, top accent bar, cleaner text, `"click"` hint
- [ ] **6. Redesign inline editor** — same block look but with slider + input
- [ ] **7. Redesign plot blocks** — theme-aware background, accent border
- [ ] **8. Redesign toolbar** — accent-colored logo icon, clean spacing, theme-aware toggle
- [ ] **9. Update toolbar background** — match sidebar color, proper padding

## Verification

1. `cargo build` — must compile with zero warnings
2. `cargo run` — app should open with the new modern look
3. Toggle theme — verify all colors switch properly
4. Place components, select/edit, delete — verify all interactions work
5. Check sidebar palette cards show hover/selected states
