use crate::notebook::model::{Output, OutputData, OutputStream};
use egui::{Color32, RichText, Ui};

pub fn show_output(ui: &mut Ui, output: &Output) {
    match output {
        Output::Text { stream, text } => {
            let color = match stream {
                OutputStream::Stdout => Color32::from_rgb(210, 220, 225),
                OutputStream::Stderr => Color32::from_rgb(255, 180, 150),
            };
            ui.label(RichText::new(text).monospace().color(color));
        }
        Output::Result { data, .. } => match data {
            OutputData::Text(text) => {
                ui.label(
                    RichText::new(text)
                        .monospace()
                        .color(Color32::from_rgb(185, 230, 190)),
                );
            }
        },
        Output::Error {
            name,
            message,
            traceback,
        } => {
            ui.label(
                RichText::new(format!("{name}: {message}"))
                    .monospace()
                    .color(Color32::from_rgb(255, 120, 120)),
            );
            ui.collapsing("traceback", |ui| {
                ui.label(RichText::new(traceback).monospace());
            });
        }
    }
}
