#![allow(dead_code)]

use crate::notebook::ids::CellId;

#[derive(Debug, Clone)]
pub struct ExecuteRequest {
    pub cell_id: CellId,
    pub source: String,
}
