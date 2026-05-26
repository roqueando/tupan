pub mod commands;

use crate::{
    notebook::{
        ids::CellId,
        model::{Cell, CellExecutionState, Notebook, Output, OutputData, OutputStream},
        persistence,
    },
    runtime::{protocol::ExecuteStatus, RuntimeClient, RuntimeEvent},
    ui::{
        cell_view::CellAction,
        notebook_view,
        toolbar::{self, ToolbarAction},
    },
};
use chrono::Utc;
use eframe::egui;
use std::sync::mpsc::Receiver;

pub struct TupanApp {
    notebook: Notebook,
    notebook_path: String,
    runtime: RuntimeClient,
    runtime_events: Receiver<RuntimeEvent>,
    status: String,
}

impl TupanApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (runtime, runtime_events) = RuntimeClient::start();
        Self {
            notebook: Notebook::new(),
            notebook_path: "notebook.tupan.json".to_owned(),
            runtime,
            runtime_events,
            status: "starting kernel".to_owned(),
        }
    }

    fn drain_runtime_events(&mut self, ctx: &egui::Context) {
        while let Ok(event) = self.runtime_events.try_recv() {
            self.apply_runtime_event(event);
            ctx.request_repaint();
        }
    }

    fn apply_runtime_event(&mut self, event: RuntimeEvent) {
        match event {
            RuntimeEvent::KernelReady => {
                self.status = "kernel ready".to_owned();
            }
            RuntimeEvent::CellRunning { cell_id } => {
                if let Some(cell) = self.cell_mut(cell_id) {
                    cell.execution = CellExecutionState::Running;
                }
            }
            RuntimeEvent::Stdout { cell_id, text } => {
                if let Some(cell) = self.cell_mut(cell_id) {
                    cell.outputs.push(Output::Text {
                        stream: OutputStream::Stdout,
                        text,
                    });
                }
            }
            RuntimeEvent::Stderr { cell_id, text } => {
                if let Some(cell) = self.cell_mut(cell_id) {
                    cell.outputs.push(Output::Text {
                        stream: OutputStream::Stderr,
                        text,
                    });
                }
            }
            RuntimeEvent::Result { cell_id, repr } => {
                if let Some(cell) = self.cell_mut(cell_id) {
                    cell.outputs.push(Output::Result {
                        mime: "text/plain".to_owned(),
                        data: OutputData::Text(repr),
                    });
                }
            }
            RuntimeEvent::Error {
                cell_id,
                name,
                message,
                traceback,
            } => {
                if let Some(cell) = self.cell_mut(cell_id) {
                    cell.outputs.push(Output::Error {
                        name,
                        message,
                        traceback,
                    });
                }
            }
            RuntimeEvent::CellFinished { cell_id, status } => {
                if let Some(cell) = self.cell_mut(cell_id) {
                    let last_run_at = Utc::now();
                    cell.execution = match status {
                        ExecuteStatus::Success => CellExecutionState::Success { last_run_at },
                        ExecuteStatus::Error => CellExecutionState::Error { last_run_at },
                    };
                    cell.dependencies.stale = false;
                }
            }
            RuntimeEvent::RuntimeError { message } => {
                self.status = format!("runtime error: {message}");
            }
        }
    }

    fn cell_mut(&mut self, cell_id: CellId) -> Option<&mut Cell> {
        self.notebook
            .cells
            .iter_mut()
            .find(|cell| cell.id == cell_id)
    }

    fn handle_toolbar_action(&mut self, action: ToolbarAction) {
        match action {
            ToolbarAction::New => {
                self.notebook = Notebook::new();
                self.status = "new notebook".to_owned();
                self.runtime.restart();
            }
            ToolbarAction::Open => match persistence::load_notebook(&self.notebook_path) {
                Ok(notebook) => {
                    self.notebook = notebook;
                    self.status = "notebook loaded".to_owned();
                    self.runtime.restart();
                }
                Err(error) => {
                    self.status = format!("open failed: {error}");
                }
            },
            ToolbarAction::Save => {
                match persistence::save_notebook(&self.notebook_path, &mut self.notebook) {
                    Ok(()) => {
                        self.status = "notebook saved".to_owned();
                    }
                    Err(error) => {
                        self.status = format!("save failed: {error}");
                    }
                }
            }
            ToolbarAction::AddCell => {
                self.notebook.add_python_cell();
            }
            ToolbarAction::RestartKernel => {
                for cell in &mut self.notebook.cells {
                    if matches!(
                        cell.execution,
                        CellExecutionState::Running | CellExecutionState::Queued
                    ) {
                        cell.execution = CellExecutionState::Stale;
                    }
                }
                self.status = "restarting kernel".to_owned();
                self.runtime.restart();
            }
            ToolbarAction::None => {}
        }
    }

    fn handle_cell_action(&mut self, index: usize, action: CellAction) {
        if index >= self.notebook.cells.len() {
            return;
        }

        match action {
            CellAction::Run => {
                let cell = &mut self.notebook.cells[index];
                cell.outputs.clear();
                cell.execution = CellExecutionState::Queued;
                self.runtime.execute(cell.id, cell.source.clone());
            }
            CellAction::Delete => {
                self.notebook.cells.remove(index);
                if self.notebook.cells.is_empty() {
                    self.notebook.add_python_cell();
                }
            }
            CellAction::MoveUp => {
                if index > 0 {
                    self.notebook.cells.swap(index, index - 1);
                }
            }
            CellAction::MoveDown => {
                if index + 1 < self.notebook.cells.len() {
                    self.notebook.cells.swap(index, index + 1);
                }
            }
            CellAction::ClearOutputs => {
                self.notebook.cells[index].outputs.clear();
            }
            CellAction::None => {}
        }
    }
}

impl eframe::App for TupanApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain_runtime_events(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            let action = toolbar::show_toolbar(
                ui,
                &mut self.notebook.title,
                &mut self.notebook_path,
                &self.status,
            );
            self.handle_toolbar_action(action);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let actions = notebook_view::show_notebook(ui, &mut self.notebook);
                for action in actions {
                    if action.source_changed {
                        self.notebook.cells[action.index].execution = CellExecutionState::Stale;
                        self.notebook.mark_following_stale(action.index);
                    }
                    self.handle_cell_action(action.index, action.action);
                }
            });
        });
    }
}

impl Drop for TupanApp {
    fn drop(&mut self) {
        self.runtime.shutdown();
    }
}
