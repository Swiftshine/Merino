use crate::merino::archive_viewer::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        println!("hi");
        ui.label("level editor");
    }
}
