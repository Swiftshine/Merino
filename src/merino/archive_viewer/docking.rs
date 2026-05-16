use crate::merino::{
    archive_viewer::{
        level_editor::{LevelEditor, docking::LevelEditorTab},
        viewer::ArchiveViewer,
    },
    util::emoji::EmojiMessage,
};
use anyhow::Result;
use std::fs;

const DOCKING_SETTINGS_FILE: &str = "dock_settings.json";

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ArchiveViewerTab {
    Archive,
    LevelEditor,
    // BSONEditor,
}

impl ArchiveViewerTab {
    fn get_name(&self) -> String {
        match self {
            Self::Archive => EmojiMessage::folder_msg("Archive"),
            Self::LevelEditor => EmojiMessage::memo_msg("Level Editor"),
        }
    }
}

pub struct ArchiveViewerTabViewer<'a> {
    archive_viewer: &'a mut ArchiveViewer,
}

impl<'a> ArchiveViewerTabViewer<'a> {
    pub fn new(archive_viewer: &'a mut ArchiveViewer) -> Self {
        Self { archive_viewer }
    }
}

impl<'a> egui_dock::TabViewer for ArchiveViewerTabViewer<'a> {
    type Tab = ArchiveViewerTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.get_name().into()
    }

    fn is_closeable(&self, tab: &ArchiveViewerTab) -> bool {
        *tab != ArchiveViewerTab::Archive
    }

    fn allowed_in_windows(&self, tab: &mut ArchiveViewerTab) -> bool {
        *tab != ArchiveViewerTab::Archive
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            ArchiveViewerTab::Archive => {
                self.archive_viewer.show_archive_ui(ui);
            }

            ArchiveViewerTab::LevelEditor => {
                self.archive_viewer.level_editor.show_ui(ui);
            }
        }
    }
}

impl ArchiveViewer {
    /// A default dock state containing just the archive tab.
    pub fn default_dock() -> egui_dock::DockState<ArchiveViewerTab> {
        egui_dock::DockState::new(vec![ArchiveViewerTab::Archive])
    }

    /// Opens a specified tab.
    pub fn open_tab(&mut self, tab: ArchiveViewerTab) {
        let found = {
            self.dock_state
                .as_ref()
                .unwrap()
                .main_surface()
                .iter()
                .any(|node| node.tabs().is_some_and(|tabs| tabs.contains(&tab)))
        };

        // check if it's not already open first
        if !found {
            self.dock_state
                .as_mut()
                .unwrap()
                .main_surface_mut()
                .push_to_first_leaf(tab);
        }
    }

    pub fn update_dock(&mut self, ui: &mut egui::Ui) {
        // temporarily move dock state out to avoid borrowing self twice
        let mut dock_state = self.dock_state.take().unwrap();

        egui_dock::DockArea::new(&mut dock_state)
            .style(egui_dock::Style::from_egui(ui.style()))
            .id(ui.auto_id_with("av_dock"))
            .show(ui.ctx(), &mut ArchiveViewerTabViewer::new(self));

        // put it back
        self.dock_state = Some(dock_state);

        // tab adding
        if let Some(tab) = self.tab_to_open.take() {
            self.open_tab(tab);
        }
    }

    /// Opens a tab.
    /// Note: needs to be delayed due to ownership of the dock state.
    pub fn schedule_open_tab(&mut self, tab: ArchiveViewerTab) {
        self.tab_to_open = Some(tab);
    }
}

impl LevelEditor {
    pub fn read_dock_state() -> Result<egui_dock::DockState<LevelEditorTab>> {
        let path = Self::get_level_editor_folder()?.join(DOCKING_SETTINGS_FILE);
        let json = fs::read_to_string(path)?;

        let state = serde_json::from_str::<egui_dock::DockState<LevelEditorTab>>(&json)?;

        Ok(state)
    }

    pub fn write_dock_state(&self) -> Result<()> {
        let path = Self::get_level_editor_folder()?.join(DOCKING_SETTINGS_FILE);
        let json = serde_json::to_string_pretty(&self.dock_state)?;
        fs::write(path, json)?;
        Ok(())
    }
}
