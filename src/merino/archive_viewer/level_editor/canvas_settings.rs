use crate::merino::archive_viewer::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_canvas_settings_ui(&mut self, ui: &mut egui::Ui) {
        let mut prune = false;

        // scoped borrow
        {
            let settings = self.canvas_context.settings_mut();

            // visibility/editability settings
            ui.label(
                egui::RichText::new("Visibility/Editability Settings")
                    .strong()
                    .underline(),
            )
            .on_hover_text(
                "Disabling any of these values will deselect every node of the corresponding type.",
            );
    
            let node_edit_settings = settings.node_edit_settings_mut();
    
            for (node_type, settings) in node_edit_settings.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(node_type.to_string());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            prune |= ui
                                .add_enabled(
                                    settings.visible,
                                    egui::Checkbox::new(&mut settings.editable, "Editable"),
                                )
                                .changed();
                            prune |= ui.checkbox(&mut settings.visible, "Visible").changed();
                        });
                    });
                });
            }
    
            
            // canvas settings
            ui.label(egui::RichText::new("Other Settings").strong());
            ui.checkbox(settings.display_grid_mut(), "Display Grid");
            ui.checkbox(settings.snap_to_grid_mut(), "Snap to Grid");
        }

        if prune {
            self.canvas_context.prune_invalid_selections();
        }
    }
}
