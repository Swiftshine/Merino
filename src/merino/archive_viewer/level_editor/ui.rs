use strum::IntoEnumIterator;

use crate::merino::archive_viewer::level_editor::{LevelEditor, docking::LevelEditorTab};

impl LevelEditor {
    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        if !self.has_mapdata() {
            ui.centered_and_justified(|ui| {
                ui.label("No file loaded.");
            });

            return;
        }

        self.show_top_menu(ui);
        self.process_messages();
        self.update_dock(ui);
    }

    pub fn show_top_menu(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(ui.next_auto_id())
        .resizable(false)
        .show_inside(ui, |ui|{
            egui::MenuBar::new().ui(ui, |ui|{
                // tab submenu
                ui.menu_button("Open Tab", |ui|{
                    for tab in LevelEditorTab::iter() {
                        if ui.button(tab.get_name()).clicked() {
                            self.open_tab(tab);
                        }
                    }
                });

                // todo! make this also happen on startup
                if ui.button("Load Parameters").clicked() {
                    if let Ok(string) = Self::load_params() {
                        let _ = self.parse_params(string);
                    }
                }
            });
        });
    }
}
