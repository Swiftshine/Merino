use crate::merino::archive_viewer::bson_editor::BSONEditor;

impl BSONEditor {
    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        self.show_top_menu(ui);
        self.show_textedit(ui);
    }

    pub fn show_top_menu(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(ui.next_auto_id())
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    if ui.button("Save to Archive").clicked() {
                        if let Ok(data) = self.write_bson() {
                            self.writable_data = Some(data);
                        }
                    }
                });
            });
    }

    pub fn show_textedit(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.json_string)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_width(f32::INFINITY),
            );
        });
    }
}
