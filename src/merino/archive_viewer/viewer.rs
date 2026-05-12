use crate::merino::archive_viewer::{
    contexts::file_context::FileContext, docking::{Tab, TabViewer}, level_editor::LevelEditor
};

/// View and edit mapbin or bson files.
pub struct ArchiveViewer {
    // editors
    pub level_editor: LevelEditor,
    // contexts
    pub file_context: FileContext,
    // dock state
    pub dock_state: egui_dock::DockState<Tab>,
    tab_to_open: Option<Tab>,
}

impl ArchiveViewer {
    pub fn new() -> Self {
        // todo! load state from file
        let dock_state = Self::default_dock();

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

    fn update_dock(&mut self, ui: &mut egui::Ui) {
        // temporarily move dock state out to avoid borrowing self twice
        let mut dock_state =
            std::mem::replace(&mut self.dock_state, egui_dock::DockState::new(vec![]));

        egui_dock::DockArea::new(&mut dock_state)
            .style(egui_dock::Style::from_egui(ui.style()))
            .show(ui.ctx(), &mut TabViewer::new(self));

        // put it back
        self.dock_state = dock_state;

        // tab adding
        if let Some(tab) = self.tab_to_open.take() {
            self.open_tab(tab);
        }
    }

    /// Opens a tab.
    /// Note: needs to be delayed due to ownership of the dock state.
    pub fn schedule_open_tab(&mut self, tab: Tab) {
        self.tab_to_open = Some(tab);
    }
}
