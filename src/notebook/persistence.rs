use super::model::{Notebook, NOTEBOOK_VERSION};
use anyhow::{Context, Result};
use chrono::Utc;
use std::{fs, path::Path};

pub fn load_notebook(path: impl AsRef<Path>) -> Result<Notebook> {
    let path = path.as_ref();
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read notebook at {}", path.display()))?;
    let mut notebook: Notebook = serde_json::from_str(&data)
        .with_context(|| format!("failed to parse notebook at {}", path.display()))?;
    notebook.version = NOTEBOOK_VERSION;
    Ok(notebook)
}

pub fn save_notebook(path: impl AsRef<Path>, notebook: &mut Notebook) -> Result<()> {
    let path = path.as_ref();
    notebook.version = NOTEBOOK_VERSION;
    notebook.metadata.updated_at = Utc::now();
    let data = serde_json::to_string_pretty(notebook).context("failed to serialize notebook")?;
    fs::write(path, data).with_context(|| format!("failed to write notebook at {}", path.display()))
}
