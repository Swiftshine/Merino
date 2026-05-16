use strum::EnumIter;

use crate::merino::{archive_viewer::level_editor::LevelEditor, util::emoji::EmojiMessage};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, EnumIter, Serialize, Deserialize)]
pub enum LevelEditorTab {
    AddObject,
    Canvas,
    CanvasSettings,
    ObjectProperties,
}

impl LevelEditorTab {
    pub fn get_name(&self) -> String {
        match self {
            Self::AddObject => EmojiMessage::add_msg("Add Object"),
            Self::Canvas => EmojiMessage::palette_msg("Canvas"),
            Self::CanvasSettings => EmojiMessage::burger_msg("Canvas Settings"),
            Self::ObjectProperties => EmojiMessage::memo_msg("Object Properties"),
        }
    }
}

pub struct LevelEditorTabViewer<'a> {
    level_editor: &'a mut LevelEditor,
}

impl<'a> LevelEditorTabViewer<'a> {
    pub fn new(level_editor: &'a mut LevelEditor) -> Self {
        Self { level_editor }
    }
}

impl<'a> egui_dock::TabViewer for LevelEditorTabViewer<'a> {
    type Tab = LevelEditorTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.get_name().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            LevelEditorTab::AddObject => {
                self.level_editor.show_add_object_ui(ui);
            }

            LevelEditorTab::Canvas => {
                self.level_editor.show_canvas(ui);
            }

            LevelEditorTab::CanvasSettings => {
                self.level_editor.show_canvas_settings_ui(ui);
            }

            LevelEditorTab::ObjectProperties => {
                self.level_editor.show_object_properties(ui);
            }
        }
    }
}

impl LevelEditor {
    /// A default dock state containing just the archive tab.
    pub fn default_dock() -> egui_dock::DockState<LevelEditorTab> {
        egui_dock::DockState::new(vec![LevelEditorTab::Canvas])
    }

    /// Opens a specified tab.
    pub fn open_tab(&mut self, tab: LevelEditorTab) {
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
            let dock_state = self.dock_state.as_mut().unwrap();

            // create a new floating window
            let _surface = dock_state.add_window(vec![tab]);
        }
    }

    pub fn update_dock(&mut self, ui: &mut egui::Ui) {
        let mut dock_state = self.dock_state.take().unwrap();

        egui_dock::DockArea::new(&mut dock_state)
            .style(egui_dock::Style::from_egui(ui.style()))
            .id(ui.auto_id_with("le_dock"))
            .show_inside(ui, &mut LevelEditorTabViewer::new(self));

        self.dock_state = Some(dock_state);

        if let Some(tab) = self.tab_to_open.take() {
            self.open_tab(tab);
        }
    }
}
