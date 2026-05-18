use crate::merino::archive_viewer::{
    bson_editor::BSONEditor, contexts::file_context::FileContext, docking::ArchiveViewerTab,
    level_editor::LevelEditor,
};

/// View and edit mapbin or bson files.
pub struct ArchiveViewer {
    // editors
    pub bson_editor: BSONEditor,
    pub level_editor: LevelEditor,
    // contexts
    pub file_context: FileContext,
    // dock state
    pub dock_state: Option<egui_dock::DockState<ArchiveViewerTab>>,
    pub tab_to_open: Option<ArchiveViewerTab>,
    // other
    // 1 in 10 chance
    pub special: bool,
}

impl ArchiveViewer {
    pub fn new(special: bool) -> Self {
        let dock_state = Some(Self::default_dock());

        let mut viewer = Self {
            bson_editor: BSONEditor::new(),
            level_editor: LevelEditor::new(),
            file_context: FileContext::new(),
            dock_state,
            tab_to_open: None,
            special
        };

        viewer.on_start();

        viewer
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        // check if writing is requested

        if self.level_editor.has_writable_data() {
            let data = self.level_editor.take_writable_data().unwrap();
            self.file_context.replace_current_file_contents(data);
        }

        if self.bson_editor.has_writable_data() {
            let data = self.bson_editor.take_writable_data().unwrap();
            self.file_context.replace_current_file_contents(data);
        }

        self.update_dock(ui);
    }

    fn on_start(&mut self) {
        self.level_editor.on_start();
    }

    pub fn on_exit(&mut self) {
        self.level_editor.on_exit();
    }
}
