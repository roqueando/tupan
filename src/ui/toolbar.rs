use egui::{TextEdit, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarAction {
    New,
    Open,
    Save,
    AddCell,
    RestartKernel,
    None,
}

pub fn show_toolbar(
    ui: &mut Ui,
    title: &mut String,
    path: &mut String,
    status: &str,
) -> ToolbarAction {
    let mut action = ToolbarAction::None;

    ui.horizontal(|ui| {
        ui.label("Title");
        ui.add(TextEdit::singleline(title).desired_width(180.0));
        ui.separator();
        ui.label("Path");
        ui.add(TextEdit::singleline(path).desired_width(280.0));
        if ui.button("New").clicked() {
            action = ToolbarAction::New;
        }
        if ui.button("Open").clicked() {
            action = ToolbarAction::Open;
        }
        if ui.button("Save").clicked() {
            action = ToolbarAction::Save;
        }
        if ui.button("Add cell").clicked() {
            action = ToolbarAction::AddCell;
        }
        if ui.button("Restart kernel").clicked() {
            action = ToolbarAction::RestartKernel;
        }
        ui.separator();
        ui.label(status);
    });

    action
}
