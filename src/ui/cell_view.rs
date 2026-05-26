use crate::{
    notebook::model::{Cell, CellExecutionState, CellKind},
    ui::output_view,
};
use egui::{Frame, RichText, TextEdit, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellAction {
    Run,
    Delete,
    MoveUp,
    MoveDown,
    ClearOutputs,
    None,
}

pub struct CellViewResponse {
    pub action: CellAction,
    pub source_changed: bool,
}

pub fn show_cell(ui: &mut Ui, cell: &mut Cell, index: usize) -> CellViewResponse {
    let mut action = CellAction::None;
    let mut source_changed = false;

    Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("#{}", index + 1)).strong());
            ui.label(cell.kind_label());
            ui.separator();
            ui.label(RichText::new(cell.execution.label()).monospace());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Delete").clicked() {
                    action = CellAction::Delete;
                }
                if ui.button("Clear").clicked() {
                    action = CellAction::ClearOutputs;
                }
                if ui.button("Down").clicked() {
                    action = CellAction::MoveDown;
                }
                if ui.button("Up").clicked() {
                    action = CellAction::MoveUp;
                }
                if ui.button("Run").clicked()
                    && matches!(
                        cell.execution,
                        CellExecutionState::Idle
                            | CellExecutionState::Success { .. }
                            | CellExecutionState::Error { .. }
                            | CellExecutionState::Stale
                    )
                {
                    action = CellAction::Run;
                }
            });
        });

        let editor = TextEdit::multiline(&mut cell.source)
            .font(egui::TextStyle::Monospace)
            .desired_rows(5)
            .code_editor()
            .lock_focus(true)
            .desired_width(f32::INFINITY);

        if ui.add(editor).changed() {
            source_changed = true;
        }

        if !cell.outputs.is_empty() {
            ui.separator();
            for output in &cell.outputs {
                output_view::show_output(ui, output);
            }
        }
    });

    CellViewResponse {
        action,
        source_changed,
    }
}

trait CellKindLabel {
    fn kind_label(&self) -> &'static str;
}

impl CellKindLabel for Cell {
    fn kind_label(&self) -> &'static str {
        match self.kind {
            CellKind::Python => "Python",
            CellKind::Markdown => "Markdown",
        }
    }
}
