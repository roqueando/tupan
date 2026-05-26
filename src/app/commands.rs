#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCommand {
    NewNotebook,
    OpenNotebook,
    SaveNotebook,
    AddCell,
    RestartKernel,
}
