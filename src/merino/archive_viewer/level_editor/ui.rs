use strum::IntoEnumIterator;

use crate::merino::archive_viewer::level_editor::{
    LevelEditor,
    contexts::log_context::LogCategory,
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
        self.process_messages();
        self.handle_download_messages();
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
                        if ui.button("Object Parameters").clicked() {
                            match Self::load_params() {
                                Ok(string) => {
                                    self.log_context.log(
                                        LogCategory::Load,
                                        "Loaded object parameters.".to_string(),
                                    );

                                    match self.parse_params(string) {
                                        Ok(_) => self.log_context.log(
                                            LogCategory::Parse,
                                            "Parsed object parameters.".to_string(),
                                        ),
                                        Err(e) => self.log_context.log_error(format!(
                                            "Could not parse object parameters. Error: {e}"
                                        )),
                                    }
                                }

                                Err(e) => {
                                    self.log_context.log_error(format!(
                                        "Could not load object parameters. Error: {e}"
                                    ));
                                }
                            }
                        }

                        if ui.button("Object Images").clicked() {
                            match Self::load_image_data() {
                                Ok(string) => {
                                    self.log_context.log(
                                        LogCategory::Load,
                                        "Loaded object images.".to_string(),
                                    );

                                    match self.parse_image_data(string) {
                                        Ok(_) => self.log_context.log(
                                            LogCategory::Parse,
                                            "Parsed object images.".to_string(),
                                        ),
                                        Err(e) => self.log_context.log_error(format!(
                                            "Could not parse object images. Error: {e}"
                                        )),
                                    }
                                }

                                Err(e) => {
                                    self.log_context.log_error(format!(
                                        "Could not load object images. Error: {e}"
                                    ));
                                }
                            }
                        }
                    });
                });
            });
    }
}
