use crate::merino::archive_viewer::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        if !self.has_mapdata() {
            ui.centered_and_justified(|ui| {
                ui.label("No file loaded.");
            });

            return;
        }

        self.process_messages();
        self.update_dock(ui);
    }
}
