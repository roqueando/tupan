use super::{
    protocol::{ExecuteStatus, KernelEvent, KernelRequest},
    python_process::PythonProcess,
};
use crate::notebook::ids::CellId;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

#[derive(Debug, Clone)]
pub enum RuntimeCommand {
    Execute { cell_id: CellId, source: String },
    Restart,
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    KernelReady,
    CellRunning {
        cell_id: CellId,
    },
    Stdout {
        cell_id: CellId,
        text: String,
    },
    Stderr {
        cell_id: CellId,
        text: String,
    },
    Result {
        cell_id: CellId,
        repr: String,
    },
    Error {
        cell_id: CellId,
        name: String,
        message: String,
        traceback: String,
    },
    CellFinished {
        cell_id: CellId,
        status: ExecuteStatus,
    },
    RuntimeError {
        message: String,
    },
}

#[derive(Clone)]
pub struct RuntimeClient {
    command_tx: Sender<RuntimeCommand>,
}

impl RuntimeClient {
    pub fn start() -> (Self, Receiver<RuntimeEvent>) {
        let (command_tx, command_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        thread::spawn(move || run_worker(command_rx, event_tx));

        (Self { command_tx }, event_rx)
    }

    pub fn execute(&self, cell_id: CellId, source: String) {
        let _ = self
            .command_tx
            .send(RuntimeCommand::Execute { cell_id, source });
    }

    pub fn restart(&self) {
        let _ = self.command_tx.send(RuntimeCommand::Restart);
    }

    pub fn shutdown(&self) {
        let _ = self.command_tx.send(RuntimeCommand::Shutdown);
    }
}

fn run_worker(command_rx: Receiver<RuntimeCommand>, event_tx: Sender<RuntimeEvent>) {
    let mut process = start_process(&event_tx);

    while let Ok(command) = command_rx.recv() {
        match command {
            RuntimeCommand::Execute { cell_id, source } => {
                if process.is_none() {
                    process = start_process(&event_tx);
                }

                let Some(proc_ref) = process.as_mut() else {
                    continue;
                };

                let _ = event_tx.send(RuntimeEvent::CellRunning { cell_id });
                let request = KernelRequest::ExecuteRequest { cell_id, source };
                match proc_ref.execute(request) {
                    Ok(events) => {
                        for event in events {
                            forward_kernel_event(event, &event_tx);
                        }
                    }
                    Err(error) => {
                        let _ = event_tx.send(RuntimeEvent::RuntimeError {
                            message: error.to_string(),
                        });
                        process = None;
                    }
                }
            }
            RuntimeCommand::Restart => {
                process = start_process(&event_tx);
            }
            RuntimeCommand::Shutdown => break,
        }
    }
}

fn start_process(event_tx: &Sender<RuntimeEvent>) -> Option<PythonProcess> {
    match PythonProcess::start() {
        Ok(process) => {
            let _ = event_tx.send(RuntimeEvent::KernelReady);
            Some(process)
        }
        Err(error) => {
            let _ = event_tx.send(RuntimeEvent::RuntimeError {
                message: error.to_string(),
            });
            None
        }
    }
}

fn forward_kernel_event(event: KernelEvent, event_tx: &Sender<RuntimeEvent>) {
    let runtime_event = match event {
        KernelEvent::KernelReady => RuntimeEvent::KernelReady,
        KernelEvent::Stdout { cell_id, text } => RuntimeEvent::Stdout { cell_id, text },
        KernelEvent::Stderr { cell_id, text } => RuntimeEvent::Stderr { cell_id, text },
        KernelEvent::ExecuteResult { cell_id, repr } => RuntimeEvent::Result { cell_id, repr },
        KernelEvent::Error {
            cell_id,
            name,
            message,
            traceback,
        } => RuntimeEvent::Error {
            cell_id,
            name,
            message,
            traceback,
        },
        KernelEvent::ExecuteDone { cell_id, status } => {
            RuntimeEvent::CellFinished { cell_id, status }
        }
    };
    let _ = event_tx.send(runtime_event);
}
