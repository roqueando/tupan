use super::ids::{CellId, NotebookId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const NOTEBOOK_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub id: NotebookId,
    pub title: String,
    pub cells: Vec<Cell>,
    pub metadata: NotebookMetadata,
    pub version: u32,
}

impl Notebook {
    pub fn new() -> Self {
        Self {
            id: NotebookId::new(),
            title: "Untitled notebook".to_owned(),
            cells: vec![Cell::python("print('Hello from Tupan')")],
            metadata: NotebookMetadata::default(),
            version: NOTEBOOK_VERSION,
        }
    }

    pub fn add_python_cell(&mut self) {
        self.cells.push(Cell::python(""));
    }

    pub fn mark_following_stale(&mut self, index: usize) {
        for cell in self.cells.iter_mut().skip(index + 1) {
            if !matches!(
                cell.execution,
                CellExecutionState::Queued | CellExecutionState::Running
            ) {
                cell.execution = CellExecutionState::Stale;
                cell.dependencies.stale = true;
            }
        }
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for NotebookMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: CellId,
    pub kind: CellKind,
    pub source: String,
    pub outputs: Vec<Output>,
    pub execution: CellExecutionState,
    pub dependencies: CellDependencies,
}

impl Cell {
    pub fn python(source: impl Into<String>) -> Self {
        Self {
            id: CellId::new(),
            kind: CellKind::Python,
            source: source.into(),
            outputs: Vec::new(),
            execution: CellExecutionState::Idle,
            dependencies: CellDependencies::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellKind {
    Python,
    Markdown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CellDependencies {
    pub reads: Vec<String>,
    pub writes: Vec<String>,
    pub depends_on: Vec<CellId>,
    pub dependents: Vec<CellId>,
    pub stale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    Text {
        stream: OutputStream,
        text: String,
    },
    Result {
        mime: String,
        data: OutputData,
    },
    Error {
        name: String,
        message: String,
        traceback: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputStream {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputData {
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellExecutionState {
    Idle,
    Queued,
    Running,
    Success { last_run_at: DateTime<Utc> },
    Error { last_run_at: DateTime<Utc> },
    Stale,
}

impl CellExecutionState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Success { .. } => "success",
            Self::Error { .. } => "error",
            Self::Stale => "stale",
        }
    }
}
