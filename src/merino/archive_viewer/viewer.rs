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
        // check if writing is requested
        
        if self.level_editor.has_writable_data() {
            let data = self.level_editor.take_writable_data().unwrap();
            self.file_context.replace_current_file_contents(data);
        }

        self.update_dock(ui);
    }

    pub fn on_exit(&mut self) {
        self.level_editor.on_exit();
    }
}
