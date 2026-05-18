use crate::merino::{archive_viewer::bson_editor::BSONEditor, util::emoji::EmojiMessage};

impl BSONEditor {
    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        self.show_top_menu(ui);
        self.show_error_box(ui);
        self.show_textedit(ui);
    }

    pub fn show_error_box(&mut self, ui: &mut egui::Ui) {
        let error_text = self.error_message.clone();

        if let Some(err) = error_text {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(60, 20, 20))
                .stroke(egui::Stroke::new(1.0, egui::Color32::RED))
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.colored_label(egui::Color32::LIGHT_RED, "JSON Error:");
                    ui.label(err);

                    if ui.button(EmojiMessage::cross_msg("Clear")).clicked() {
                        self.error_message = None;
                    }
                });
        }

        ui.add_space(6.0);
    }

    pub fn show_top_menu(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(ui.next_auto_id())
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    if self.is_individual_file {
                        if ui.button("Save BSON As").clicked() {
                            match self.write_bson_to_file() {
                                Ok(_) => {
                                    self.show_error_popup = false;
                                }

                                Err(e) => {
                                    self.error_message = Some(e.to_string());
                                    self.show_error_popup = true;
                                }
                            }
                        }
                    } else if ui.button("Save BSON to Archive").clicked() {
                        match self.write_bson() {
                            Ok(data) => {
                                self.writable_data = Some(data);
                                self.show_error_popup = false;
                            }
                            Err(e) => {
                                self.error_message = Some(e.to_string());
                                self.show_error_popup = true;
                            }
                        }
                    }

                    if ui.button("Import JSON").clicked() {
                        let _ = self.import_json();
                    }
                    
                    if ui.button("Export JSON").clicked() {
                        let _ = self.export_json();
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
