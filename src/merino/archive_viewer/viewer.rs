use crate::merino::archive_viewer::docking::{Tab, TabViewer};

/// View and edit mapbin or bson files.
pub struct ArchiveViewer {
    dock_state: egui_dock::DockState<Tab>
}

impl ArchiveViewer {
    pub fn new() -> Self {
        // todo! load state from file
        let dock_state = Self::default_dock();

        Self {
            dock_state
        }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        self.update_dock(ui);
    }

    fn update_dock(&mut self, ui: &mut egui::Ui) {
        // temporarily move dock state out to avoid borrowing self twice
        let mut dock_state = std::mem::replace(&mut self.dock_state, egui_dock::DockState::new(vec![]));

        egui_dock::DockArea::new(&mut dock_state).style(egui_dock::Style::from_egui(ui.style())).show(ui.ctx(), &mut TabViewer::new(self));

        // put it back
        self.dock_state = dock_state;
    }
}
