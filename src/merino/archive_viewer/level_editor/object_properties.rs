use crate::merino::archive_viewer::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_object_properties(&mut self, ui: &mut egui::Ui) {
        if !self.canvas_context.can_edit_node_properties() {
            ui.centered_and_justified(|ui|{
                ui.label("Select exactly one node to edit its properties.");
            });

            return;
        }

        ui.label("properties!");
    }
}
