use crate::merino::{archive_viewer::viewer::ArchiveViewer, util::emoji::EmojiMessage};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Tab {
    Archive,
    LevelEditor,
    // BSONEditor,
}

impl Tab {
    fn get_name(&self) -> String {
        match self {
            Self::Archive => EmojiMessage::folder_msg("Archive"),
            Self::LevelEditor => EmojiMessage::memo_msg("Level Editor"),
        }
    }
}

pub struct TabViewer<'a> {
    archive_viewer: &'a mut ArchiveViewer,
}

impl<'a> TabViewer<'a> {
    pub fn new(archive_viewer: &'a mut ArchiveViewer) -> Self {
        Self { archive_viewer }
    }
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.get_name().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Archive => {
                self.archive_viewer.show_archive_ui(ui);
            }

            Tab::LevelEditor => {
                self.archive_viewer.level_editor.show_ui(ui);
            }
        }
    }
}

impl ArchiveViewer {
    /// A default dock state containing just the archive tab.
    pub fn default_dock() -> egui_dock::DockState<Tab> {
        egui_dock::DockState::new(vec![Tab::Archive])
    }

    /// Opens a specified tab.
    pub fn open_tab(&mut self, tab: Tab) {
        // check if it's not already open first
        if !self
            .dock_state
            .main_surface()
            .iter()
            .any(|node| node.tabs().is_some_and(|tabs| tabs.contains(&tab)))
        {
            println!("opening tab");
            self.dock_state.main_surface_mut().push_to_first_leaf(tab);
        }
    }
}
