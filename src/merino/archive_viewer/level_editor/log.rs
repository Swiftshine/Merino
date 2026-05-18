use crate::merino::{archive_viewer::level_editor::LevelEditor, util::emoji::EmojiMessage};

impl LevelEditor {
    pub fn show_log_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button(EmojiMessage::discard_msg("Clear Logs")).clicked() {
            self.log_context.clear_logs();
        }
        ui.separator();

        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for (index, log) in self.log_context.logs().enumerate() {
                    ui.push_id(index, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.strong(format!("[{}]", log.category()));
                            ui.label(log.content());
                        });

                        ui.separator();
                    });
                }

                if let Some(log) = self.log_context.current_written_log() {
                    ui.horizontal_wrapped(|ui| {
                        ui.strong(format!("[{}]", log.category()));
                        ui.label(log.content());
                    });
                }
            });
    }
}
