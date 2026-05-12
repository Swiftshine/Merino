use crate::merino::archive_viewer::viewer::ArchiveViewer;

impl ArchiveViewer {
    pub fn show_archive_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("example");
    }
}
