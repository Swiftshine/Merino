use crate::merino::archive_viewer::{
    contexts::file_context::FileContext, docking::ArchiveViewerTab, level_editor::LevelEditor,
};

/// View and edit mapbin or bson files.
pub struct ArchiveViewer {
    // editors
    pub level_editor: LevelEditor,
    // contexts
    pub file_context: FileContext,
    // dock state
    pub dock_state: Option<egui_dock::DockState<ArchiveViewerTab>>,
    pub tab_to_open: Option<ArchiveViewerTab>,
}

impl ArchiveViewer {
    pub fn new() -> Self {
        // todo! load state from file
        let dock_state = Some(Self::default_dock());

        Self {
            level_editor: LevelEditor::new(),
            file_context: FileContext::new(),
            dock_state,
            tab_to_open: None,
        }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        self.update_dock(ui);
    }
}
