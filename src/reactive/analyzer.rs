#![allow(dead_code)]

use crate::notebook::model::CellDependencies;

pub fn empty_dependencies() -> CellDependencies {
    CellDependencies::default()
}
