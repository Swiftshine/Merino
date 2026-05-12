use crate::merino::{archive_viewer::viewer::ArchiveViewer, util::emoji::EmojiMessage};

pub enum Tab {
    Archive,
    // LevelEditor,
    // BSONEditor,
}

impl Tab {
    fn get_name(&self) -> String {
        match self {
            Self::Archive => EmojiMessage::folder_msg("Archive"),
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
        }
    }
}

impl ArchiveViewer {
    /// A default dock state containing just the archive tab.
    pub fn default_dock() -> egui_dock::DockState<Tab> {
        egui_dock::DockState::new(vec![Tab::Archive])
    }
}
