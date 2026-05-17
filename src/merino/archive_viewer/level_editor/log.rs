use crate::merino::{archive_viewer::level_editor::LevelEditor, util::emoji::EmojiMessage};

impl LevelEditor {
    pub fn show_log_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button(EmojiMessage::discard_msg("Clear Logs")).clicked() {
            self.log_context.clear_logs();
        }

        for (index, log) in self.log_context.logs().enumerate() {
            ui.push_id(index, |ui|{
                ui.collapsing(log.category(), |ui|{
                    ui.label(log.content());
                });
            });
        }

        if let Some(log) = self.log_context.current_written_log() {
            ui.collapsing(log.category(), |ui|{
                ui.label(log.content());
            });
        }
    }
}
