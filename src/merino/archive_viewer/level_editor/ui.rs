use strum::IntoEnumIterator;

use crate::merino::archive_viewer::level_editor::{
    LevelEditor,
    docking::LevelEditorTab,
    download::{IMAGEDB_URL, OBJECTDB_URL},
};

impl LevelEditor {
    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        if !self.has_mapdata() {
            ui.centered_and_justified(|ui| {
                ui.label("No file loaded.");
            });

            return;
        }

        self.show_top_menu(ui);
        self.handle_download_messages();
        self.process_messages();
        self.update_dock(ui);
    }

    pub fn show_top_menu(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(ui.next_auto_id())
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    // tab submenu
                    ui.menu_button("Open Tab", |ui| {
                        for tab in LevelEditorTab::iter() {
                            if ui.button(tab.get_name()).clicked() {
                                self.open_tab(tab);
                            }
                        }
                    });

                    if ui
                        .add_enabled(self.has_mapdata(), egui::Button::new("Save to Archive"))
                        .clicked()
                    {
                        self.write_mapdata();
                    }

                    ui.menu_button("Download", |ui| {
                        if ui.button("Object Parameters").clicked() {
                            let _ = self.start_download(OBJECTDB_URL);
                        }

                        if ui.button("Object Images").clicked() {
                            let _ = self.start_download(IMAGEDB_URL);
                        }
                    });

                    ui.menu_button("Load", |ui| {
                        // todo! make this also happen on startup
                        if ui.button("Object Parameters").clicked()
                            && let Ok(string) = Self::load_params()
                        {
                            let _ = self.parse_params(string);
                        }

                        if ui.button("Object Images").clicked()
                            && let Ok(string) = Self::load_image_data()
                        {
                            let _ = self.parse_image_data(string);
                        }
                    });
                });
            });
    }
}
