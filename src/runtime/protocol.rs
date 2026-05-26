use crate::notebook::ids::CellId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KernelRequest {
    ExecuteRequest { cell_id: CellId, source: String },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KernelEvent {
    KernelReady,
    Stdout {
        cell_id: CellId,
        text: String,
    },
    Stderr {
        cell_id: CellId,
        text: String,
    },
    ExecuteResult {
        cell_id: CellId,
        repr: String,
    },
    Error {
        cell_id: CellId,
        name: String,
        message: String,
        traceback: String,
    },
    ExecuteDone {
        cell_id: CellId,
        status: ExecuteStatus,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteStatus {
    Success,
    Error,
}
