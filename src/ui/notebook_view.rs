#![allow(dead_code)]
use crate::{
    notebook::model::Notebook,
    ui::cell_view::{self, CellAction},
};
use egui::Ui;

#[derive(Debug, Clone, Copy)]
pub struct NotebookAction {
    pub index: usize,
    pub action: CellAction,
    pub source_changed: bool,
}

pub fn show_notebook(ui: &mut Ui, notebook: &mut Notebook) -> Vec<NotebookAction> {
    let mut actions = Vec::new();

    for index in 0..notebook.cells.len() {
        let response = cell_view::show_cell(ui, &mut notebook.cells[index], index);
        if response.action != CellAction::None || response.source_changed {
            actions.push(NotebookAction {
                index,
                action: response.action,
                source_changed: response.source_changed,
            });
        }
        ui.add_space(8.0);
    }

    actions
}
