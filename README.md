# Tupan

Tupan is a local desktop notebook prototype built with Rust, egui/eframe, and a persistent Python subprocess runtime.

## Run

```sh
cargo run
```

The app saves notebooks as JSON files. By default it reads and writes `notebook.tupan.json` in the current directory.

## Current MVP

- Editable Python cells
- Persistent Python namespace across cell executions
- Text, result, and traceback outputs
- Responsive UI through a runtime worker thread
- Kernel restart
- JSON notebook persistence
- Basic stale marking after edits

