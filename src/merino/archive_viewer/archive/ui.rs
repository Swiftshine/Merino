use crate::merino::archive_viewer::{
    contexts::file_context::FileType, docking::ArchiveViewerTab, viewer::ArchiveViewer,
};

impl ArchiveViewer {
    pub fn show_archive_ui(&mut self, ui: &mut egui::Ui) {
        self.show_top_menu(ui);

        if self.file_context.has_archive_contents() {
            self.show_archive_files(ui);
        } else if self.file_context.has_file() {
            // single file
            ui.centered_and_justified(|ui| {
                ui.label("See the appropriate editor.");
            });
        } else {
            ui.centered_and_justified(|ui| {
                if self.special {
                    ui.label("2026 Swiftshine\nOpen a file to get started.");
                } else {
                    ui.label("Open a file to get started.");
                }
            });
        }
    }

    fn show_top_menu(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(ui.next_auto_id())
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    // file submenu
                    ui.menu_button("File", |ui| {
                        if ui.button("Open Archive").clicked() {
                            if self.file_context.open_archive().is_ok() {
                                self.bson_editor.clear();
                            }

                            ui.close();
                        }

                        if ui.button("Open File").clicked() {
                            if let Ok(file_type) = self.file_context.open_file() {
                                match file_type {
                                    FileType::BSON => {
                                        self.bson_editor.set_is_individual_file(true);
                                        self.schedule_open_tab(ArchiveViewerTab::BSONEditor);
                                        let _ = self
                                            .bson_editor
                                            .load_bson(self.file_context.file_contents().unwrap());
                                    }
                                    FileType::None => {}
                                }
                            }
                            ui.close();
                        }

                        if ui
                            .add_enabled(
                                self.file_context.has_archive_contents(),
                                egui::Button::new("Save Archive"),
                            )
                            .clicked()
                        {
                            if self.file_context.save_archive().is_ok() {
                                self.bson_editor.set_is_individual_file(false);
                            }
                            ui.close();
                        }
                    });
                });
            });
    }

    /// Lists every file open in the current archive.
    fn show_archive_files(&mut self, ui: &mut egui::Ui) {
        if !self.file_context.has_archive_contents() {
            return;
        }

        let bson_ext = [".bson", ".mappath", ".MapScene"];

        let valid_extensions = [".mapbin", ".bson", ".mappath", ".MapScene"];

        let mut selected_file = None;
        let mut tab_to_open = None;

        // sort alphabetically
        // BTree doesn't do that the way a human would so we'll do it ourselves
        let mut files: Vec<_> = self.file_context.archive_contents().iter().collect();
        files.sort_by_key(|(name, _)| name.to_lowercase());

        for (name, bytes) in files {
            // make sure the file is something we're looking for
            let can_select = valid_extensions.iter().any(|ext| name.ends_with(ext));

            let is_selected = self
                .file_context
                .selected_file()
                .map(|s| name == s)
                .unwrap_or(false);

            if can_select {
                if ui.selectable_label(is_selected, name).clicked() {
                    selected_file = Some(name.clone());

                    // open tab based on type
                    if name.ends_with(".mapbin") {
                        tab_to_open = Some(ArchiveViewerTab::LevelEditor);
                        let _ = self.level_editor.load_mapdata(bytes);
                    } else {
                        for ext in bson_ext.iter() {
                            if name.ends_with(ext) {
                                tab_to_open = Some(ArchiveViewerTab::BSONEditor);
                                let _ = self.bson_editor.load_bson(bytes);
                            }
                        }
                    }
                }
            } else {
                ui.label(name);
            }
        }

        // make sure we didn't already select it
        if selected_file.is_some() && selected_file.as_ref() != self.file_context.selected_file() {
            self.file_context.set_selected_file(selected_file);
        }

        if let Some(tab) = tab_to_open {
            self.schedule_open_tab(tab);
        }
    }
}
