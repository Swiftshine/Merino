use crate::merino::archive_viewer::{docking::Tab, viewer::ArchiveViewer};

impl ArchiveViewer {
    pub fn show_archive_ui(&mut self, ui: &mut egui::Ui) {
        self.show_top_menu(ui);
        self.show_archive_files(ui);
    }

    fn show_top_menu(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(ui.next_auto_id())
            .resizable(false)
            .show_inside(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    // file submenu
                    ui.menu_button("File", |ui| {
                        if ui.button("Open Archive").clicked() {
                            let _ = self.file_context.open_archive();
                            ui.close();
                        }
                    });
                });
            });
    }

    /// Lists every file open in the current archive.
    fn show_archive_files(&mut self, ui: &mut egui::Ui) {
        if !self.file_context.has_files() {
            return;
        }
        
        let valid_extensions = [
            ".mapbin",
            // ".bson" // later
        ];

        let mut selected_file = None;
        let mut tab_to_open = None;

        for (name, bytes) in self.file_context.archive_contents() {
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
                        tab_to_open = Some(Tab::LevelEditor);
                        let _ = self.level_editor.load_mapdata(bytes);
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
